use crate::api::deserialiser_types::{SecureNotesData, SpreadsheetColumns, VaultNames};

use super::deserialiser_types::{Cells, DeleteCells, LoginData, LoginMetadata, GlobalSyncData};

/*
    Vault name deserialiser
*/
pub fn deserialise_vault_names(data: &Vec<u8>) -> VaultNames {
    VaultNames::new(data)
}

/*

 * Spreadsheet deserialisers
*/

pub fn deserialise_spreadsheet(data: Vec<u8>) -> Cells {
    Cells::new(data)
}

pub fn deserialise_delete_cells(data: Vec<u8>) -> DeleteCells {
    DeleteCells::new(data)
}

pub fn deserialise_column_data(data: &Vec<u8>) -> SpreadsheetColumns {
    SpreadsheetColumns::new(data)
}


/*
 * Login deserialisers
*/

pub fn deserialise_login_full_sync(data : Vec<u8>) -> LoginData {
    LoginData::new(data)
}


pub fn deserialise_login_data_sync(data : Vec<u8>) -> Cells {
    Cells::new(data)
}

pub fn deserialise_login_metadata(data : Vec<u8>) -> LoginMetadata {
    LoginMetadata::new(data)
}

pub fn deserialise_login_sync(data : Vec<u8>) -> Cells {
    Cells::new(data)
}

/*
    Secure notes deserialiser
*/
pub fn deserialise_secure_notes(data: Vec<u8>) -> SecureNotesData {
    SecureNotesData::new(data)
}

/*
 * Global sync deserialiser
 */
pub fn deserialise_global_sync(data : Vec<u8>) -> super::deserialiser_types::GlobalSyncData {
    GlobalSyncData::new(data)
}