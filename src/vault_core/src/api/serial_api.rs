use candid::Principal;

use crate::{
    api::deserialiser::{deserialise_delete_cells, deserialise_global_sync, deserialise_login_data_sync, deserialise_login_full_sync, deserialise_login_metadata, deserialise_spreadsheet}, 
    stable::types::{LoginsColumns, LoginsMap, SpreadsheetMap}, 
    vault_type::{
        logins::LoginSiteKey, spreadsheet::{SpreadsheetKey, SpreadsheetValue}
    }
};

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

pub fn _vault_spreadsheet_sync(user_id: Principal, vault_id: Principal, update: Vec<u8>, sm: &SpreadsheetMap) {
    if update.is_empty() {
        return;
    }

    let cell_data = deserialise_spreadsheet(update);
    _process_spreadsheet(user_id, vault_id, &cell_data, sm);
}

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

fn _process_metadata(user_id: Principal, vault_id: Principal, metadata: &super::deserialiser_types::LoginMetadata, lc: &LoginsColumns) {
    let mut columns = lc.borrow_mut();
    for meta in metadata.metadatas.iter() {
        let column_key = LoginSiteKey::new(user_id, vault_id, meta.header.x);
        if meta.data.is_empty() {
            columns.remove(&column_key);
            continue;
        }
        let column_name = meta.data.clone();
        columns.insert(column_key, column_name);
    }
}

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

pub fn _login_full_sync(user_id: Principal, vault_id: Principal, update: Vec<u8>, lc: &LoginsColumns, lm: &LoginsMap) {
    if update.is_empty() {
        return;
    }

    let login_data = deserialise_login_full_sync(update);
    
    _process_metadata(user_id, vault_id, &login_data.metadata, lc);
    _process_login_data(user_id, vault_id, &login_data.cells, lm);
}

pub fn _login_metadata_sync(user_id: Principal, vault_id: Principal, update: Vec<u8>, lc: &LoginsColumns) {
    if update.is_empty() {
        return;
    }

    let login_data = deserialise_login_full_sync(update);
    _process_metadata(user_id, vault_id, &login_data.metadata, lc);
}

pub fn _login_metadata_delete(user_id: Principal, vault_id: Principal, update: Vec<u8>, lc: &LoginsColumns) {
    if update.is_empty() {
        return;
    }
    
    let deletes = deserialise_login_metadata(update);
    let mut columns = lc.borrow_mut();
    for cell in deletes.metadatas.iter()
    {
        let column_key = LoginSiteKey::new(user_id, vault_id, cell.header.x);
        columns.remove(&column_key);
    }
}

pub fn _login_data_sync(user_id: Principal, vault_id: Principal, update: Vec<u8>, lm: &LoginsMap) {
    if update.is_empty() {
        return;
    }

    let login_data = deserialise_login_data_sync(update);
    
    _process_login_data(user_id, vault_id, &login_data.cells, lm);
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
    _process_metadata(user_id, vault_id, &global_data.logins.metadata, lc);
    _process_spreadsheet(user_id, vault_id, &global_data.spreadsheet, sm);
}