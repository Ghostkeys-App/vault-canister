use candid::Principal;
use ic_stable_structures::Storable;
use crate::{
    api::{deserialiser::{deserialise_column_data, deserialise_delete_cells, deserialise_global_sync, deserialise_login_data_sync, deserialise_login_full_sync, deserialise_login_metadata, deserialise_secure_notes, deserialise_spreadsheet, deserialise_vault_names}, dev_api::_get_vault_names}, 
    stable::types::{ColumnsInfo, GeneralState, LoginsColumns, LoginsMap, NotesMap, SpreadsheetMap, VaultNamesMap}, 
    vault_type::{
        logins::LoginSiteKey, 
        secure_notes::{SecureNote, SecureNoteKey}, 
        spreadsheet::{ColumnData, ColumnKey, SpreadsheetKey, SpreadsheetValue}, 
        vault_names::{VaultNameKey, VaultNameValue}
    }
};

// Internal function to process a set of deserialised vault name data
fn _process_vault_names(user_id: Principal, names: &super::deserialiser_types::VaultNames, vnm: &VaultNamesMap) {
    let mut names_map = vnm.borrow_mut();
    for name in names.names.iter() {
        let key = VaultNameKey::new(user_id, &name.vault_id);
        if name.vault_name.is_empty()
        {
            names_map.remove(&key);
            continue;
        }
        names_map.insert(key, VaultNameValue::new(&name.vault_name));
    }
}
pub fn _vault_names_sync(user_id: Principal, update: &Vec<u8>, vnm: &VaultNamesMap) {
    if update.is_empty() {
        return
    }

    let names = deserialise_vault_names(update);
    _process_vault_names(user_id, &names, vnm);

}

fn _process_spreadsheet_columns(user_id: Principal, vault_id: Principal, columns: &super::deserialiser_types::SpreadsheetColumns, sc: &ColumnsInfo) {
    let mut sc = sc.borrow_mut();

    for column in columns.columns.iter() {
        let key = ColumnKey::new(user_id, vault_id, column.header.x);
        let hidden = if column.header.hidden > 0 { true } else { false };
        if column.name.is_empty() && !hidden {
            sc.remove(&key);
            continue;
        }
        let value = ColumnData::new(hidden, column.name.clone());
        sc.insert(key, value);
    }
}

pub fn _vault_spreadsheet_columns_sync(user_id: Principal, vault_id: Principal, update: Vec<u8>, sc: &ColumnsInfo) {
    if update.is_empty() {
        return;
    }

    let column_data = deserialise_column_data(&update);
    _process_spreadsheet_columns(user_id, vault_id, &column_data, sc);
}

// Internal common code to process a set of deserialised spreadsheet data.
fn _process_spreadsheet(user_id: Principal, vault_id: Principal, cells: &super::deserialiser_types::Cells, sm: &SpreadsheetMap) {
    let mut spreadsheets = sm.borrow_mut();
    for cell in cells.cells.iter()
    {
        let key = SpreadsheetKey::new(user_id, vault_id, cell.header.x, cell.header.y);
        if cell.data.is_empty() {
            spreadsheets.remove(&key);
            continue;
        }
        spreadsheets.insert(key, SpreadsheetValue::new(cell.data.clone()));
    }
}

// Interface function to deserialise and process a full sync of spreadsheet data
pub fn _vault_spreadsheet_sync(user_id: Principal, vault_id: Principal, update: Vec<u8>, sm: &SpreadsheetMap) {
    if update.is_empty() {
        return;
    }

    let cell_data = deserialise_spreadsheet(update);
    _process_spreadsheet(user_id, vault_id, &cell_data, sm);
}

// Interface function to deserialise and process a delete update of spreadsheet data
pub fn _vault_spreadsheet_delete(user_id: Principal, vault_id: Principal, update: Vec<u8>, sm: &SpreadsheetMap) {
    if update.is_empty() {
        return;
    }
    
    let deletes = deserialise_delete_cells(update);
    let mut spreadsheets = sm.borrow_mut();
    for cell in deletes.cells.iter()
    {
        let key = SpreadsheetKey::new(user_id, vault_id, cell.x, cell.y);
        spreadsheets.remove(&key);
    }
}

// This function deletes all login identities associated with a given column (x value).
fn _delete_login_identities(user_id: Principal, vault_id: Principal, x: u8, lm: &LoginsMap) {
    let mut logins = lm.borrow_mut();
    let mut principals: Vec<u8> = Vec::new();
    principals.extend(user_id.as_slice());
    principals.extend(vault_id.as_slice());

    let keys_to_delete: Vec<_> = logins
        .iter()
        .filter_map(|entry| {
            let SpreadsheetKey {
                principals: column_principals,
                x: cell_x,
                y,
            } = entry.key();
            (cell_x == &x && column_principals.cmp(&principals).is_eq()).then(|| SpreadsheetKey {
                principals: principals.to_vec(), // Principals are not needed for deletion
                x: *cell_x,
                y: *y,
            })
        }).collect();

    // StableBTreeMap does not support bulk delete, so we have to do it one by one.
    // It also doesn't let you mutate the map while iterating over it, hence the 
    // two-pass approach.
    for key in keys_to_delete {
        logins.remove(&key);
    }
}

// Internal common code to process a set of deserialised login metadata.
fn _process_metadata(user_id: Principal, vault_id: Principal, metadata: &super::deserialiser_types::LoginMetadata, lc: &LoginsColumns, lm: &LoginsMap) {
    let mut columns = lc.borrow_mut();
    for meta in metadata.metadatas.iter() {
        let column_key = LoginSiteKey::new(user_id, vault_id, meta.header.x);
        if meta.data.is_empty() {
            columns.remove(&column_key);
            _delete_login_identities(user_id, vault_id, column_key.x, lm);
            continue;
        }
        let column_name = meta.data.clone();
        columns.insert(column_key, column_name);
    }
}


// Internal common code to process a set of deserialised login identity data.
fn _process_login_data(user_id: Principal, vault_id: Principal,  cells: &super::deserialiser_types::Cells, lm: &LoginsMap) {
    let mut logins = lm.borrow_mut();
    for cell in cells.cells.iter() {
        let key = SpreadsheetKey::new(user_id, vault_id, cell.header.x, cell.header.y);
        if cell.data.is_empty() {
            logins.remove(&key);
            continue;
        }
        logins.insert(key, SpreadsheetValue::new(cell.data.clone()));
    }
}

// Interface function to deserialise and process a full sync of login metadata and identity data
pub fn _login_full_sync(user_id: Principal, vault_id: Principal, update: Vec<u8>, lc: &LoginsColumns, lm: &LoginsMap) {
    if update.is_empty() {
        return;
    }

    let login_data = deserialise_login_full_sync(&update);
    
    _process_metadata(user_id, vault_id, &login_data.metadata, lc, lm);
    _process_login_data(user_id, vault_id, &login_data.cells, lm);
}

// Interface function to deserialise and process a metadata-only sync of login data
pub fn _login_metadata_sync(user_id: Principal, vault_id: Principal, update: Vec<u8>, lc: &LoginsColumns, lm: &LoginsMap) {
    if update.is_empty() {
        return;
    }

    let login_data = deserialise_login_metadata(update);
    _process_metadata(user_id, vault_id, &login_data, lc, lm);
}

// Interface function to deserialise and process a metadata-only delete of login data. Note this 
// alse deletes all associated login identities for the deleted columns.
pub fn _login_metadata_delete(user_id: Principal, vault_id: Principal, update: Vec<u8>, lc: &LoginsColumns, lm: &LoginsMap) {
    if update.is_empty() {
        return;
    }
    
    let deletes = deserialise_login_metadata(update);
    let mut columns = lc.borrow_mut();

    for cell in deletes.metadatas.iter()
    {
        let column_key = LoginSiteKey::new(user_id, vault_id, cell.header.x);
        columns.remove(&column_key);

        // Also remove all associated login identities for this column
        _delete_login_identities(user_id, vault_id, column_key.x, lm);
    }
}

pub fn _login_data_sync(user_id: Principal, vault_id: Principal, update: Vec<u8>, lm: &LoginsMap) {
    if update.is_empty() {
        return;
    }

    let login_data = deserialise_login_data_sync(&update);
    
    _process_login_data(user_id, vault_id, &login_data, lm);
}

pub fn _login_data_deletes(user_id: Principal, vault_id: Principal, update: Vec<u8>, lm: &LoginsMap) {
    if update.is_empty() {
        return;
    }
    
    let deletes = deserialise_delete_cells(update);
    let mut logins = lm.borrow_mut();
    for cell in deletes.cells.iter()
    {
        let key = SpreadsheetKey::new(user_id, vault_id, cell.x, cell.y);
        logins.remove(&key);
    }
}

fn _process_notes_data(user_id: Principal, vault_id: Principal, notes_data: &super::deserialiser_types::SecureNotesData, nm: &NotesMap) {
    let mut nm = nm.borrow_mut();

    for note in notes_data.notes.iter() {
        let principals = vec![user_id.into_bytes(), vault_id.into_bytes()].concat();
        let key = SecureNoteKey {
            index: note.header.x,
            principals
        };
        if note.label.is_empty()
        {
            nm.remove(&key);
            continue;
        }
        nm.insert(key, SecureNote::new(note.label.clone(), note.note.clone()));
    }
}

pub fn _secret_notes_sync(user_id: Principal, vault_id: Principal, update: Vec<u8>, nm: &NotesMap) {
    if update.is_empty() {
        return;
    }

    let notes = deserialise_secure_notes(update);
    _process_notes_data(user_id, vault_id, &notes, nm);
}

pub fn _global_sync(user_id: Principal, vault_id: Principal, update: Vec<u8>, state: &GeneralState) {
    if update.is_empty() {
        return;
    }
    let global_data = deserialise_global_sync(update);

    _process_login_data(user_id, vault_id, &global_data.logins.cells, &state.logins_map);
    _process_notes_data(user_id, vault_id, &global_data.secure_notes, &state.notes_map);
    _process_metadata(user_id, vault_id, &global_data.logins.metadata, &state.logins_columns, &state.logins_map);
    _process_spreadsheet(user_id, vault_id, &global_data.spreadsheet, &state.spreadsheet_map);
    _process_spreadsheet_columns(user_id, vault_id, &global_data.spreadsheet_columns, &state.spreadsheet_columns);
}

fn _process_delete_vault(principals: Vec<u8>, state: &GeneralState) {
     let mut lc = state.logins_columns.borrow_mut();
    let keys_to_remove: Vec<_> = lc.iter()
        .filter(|entry| entry.key().principals_match(&principals))
        .map(|entry| entry.key().clone())
        .collect();

    for key in keys_to_remove {
        lc.remove(&key);
    }
    
    // Remove from spreadsheet_columns
    let mut sc = state.spreadsheet_columns.borrow_mut();
    let keys_to_remove: Vec<_> = sc.iter()
        .filter(|entry| entry.key().principals_match(&principals))
        .map(|entry| entry.key().clone())
        .collect();
    for key in keys_to_remove {
        sc.remove(&key);
    }

    // Remove from spreadsheet_map
    let mut sm = state.spreadsheet_map.borrow_mut();
    let keys_to_remove: Vec<_> = sm.iter()
        .filter(|entry| entry.key().principals_match(&principals))
        .map(|entry| entry.key().clone())
        .collect();
    for key in keys_to_remove {
        sm.remove(&key);
    }

    // Remove from logins_map
    let mut lm = state.logins_map.borrow_mut();
    let keys_to_remove: Vec<_> = lm.iter()
        .filter(|entry| entry.key().principals_match(&principals))
        .map(|entry| entry.key().clone())
        .collect();
    for key in keys_to_remove {
        lm.remove(&key);
    }

    // Remove from notes_map
    let mut nm = state.notes_map.borrow_mut();
    let keys_to_remove: Vec<_> = nm.iter()
        .filter(|entry| entry.key().principals == principals)
        .map(|entry| entry.key().clone())
        .collect();
    for key in keys_to_remove {
        nm.remove(&key);
    }

    // Remove from vault_names_map
    let mut vnm = state.vault_names_map.borrow_mut();
    let keys_to_remove: Vec<_> = vnm.iter()
        .filter(|entry| entry.key().principals == principals)
        .map(|entry| entry.key().clone())
        .collect();
    for key in keys_to_remove {
        vnm.remove(&key);
    }
}

pub fn _delete_vault(user_id: Principal, vault_id: Principal, state: &GeneralState) {
    let principals = vec![user_id.into_bytes(), vault_id.into_bytes()].concat();
    _process_delete_vault(principals, state);
}

pub fn _purge_user(user_id: Principal, state: &GeneralState) {
    let vault_names = _get_vault_names(user_id, &state.vault_names_map);

    for (vault_id, _) in vault_names.names {
        let principals = vec![user_id.into_bytes(), vault_id.clone().into_bytes()].concat();
        _process_delete_vault(principals, state);
    }
}