use candid::Principal;

use crate::{
    api::deserialiser::{deserialise_delete_cells, deserialise_global_sync, deserialise_login_data_sync, deserialise_login_full_sync, deserialise_login_metadata, deserialise_spreadsheet}, 
    stable::types::{LoginsColumns, LoginsMap, SpreadsheetMap}, 
    vault_type::{
        logins::LoginSiteKey, spreadsheet::{SpreadsheetKey, SpreadsheetValue}
    }
};

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
fn _process_login_data(user_id: Principal, vault_id: Principal, cells: &super::deserialiser_types::Cells, lm: &LoginsMap) {
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

    let login_data = deserialise_login_full_sync(update);
    
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

    let login_data = deserialise_login_data_sync(update);
    
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

pub fn _global_sync(user_id: Principal, vault_id: Principal, update: Vec<u8>, lc: &LoginsColumns, lm: &LoginsMap, sm: &SpreadsheetMap) {
    if update.is_empty() {
        return;
    }
    let global_data = deserialise_global_sync(update);

    _process_login_data(user_id, vault_id, &global_data.logins.cells, lm);
    _process_metadata(user_id, vault_id, &global_data.logins.metadata, lc, lm);
    _process_spreadsheet(user_id, vault_id, &global_data.spreadsheet, sm);
}