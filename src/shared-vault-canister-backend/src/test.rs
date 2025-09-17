use std::cell::RefCell;

use candid::Principal;
use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager}, DefaultMemoryImpl, StableBTreeMap, Storable};
use vault_core::{api::{dev_api::{_get_logins, _get_notes}, serial_api::{_login_data_sync, _login_metadata_sync, _secret_notes_sync, _vault_names_sync}}, vault_type::spreadsheet::SpreadsheetKey};
use vault_core::api::dev_api::_get_vault_names;

fn some_user_id() -> Principal {
    Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap()
}

fn some_vault_id() -> Principal {
    Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap()
}

fn some_other_principal() -> Principal {
    Principal::from_text("aaaaa-aa").unwrap()
}

fn some_spreadsheet_data() -> Vec<u8> {
    vec![
        0x00, 0x0F, 0x00, 0x02, 0x74, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6B, 0x20, 0x62, 0x72, 0x6F, 0x77, 0x6E, 
        0x00, 0x00, 0x03, 0x04, 
        0x00, 0x17, 0x0B, 0x05, 0x66, 0x6F, 0x78, 0x20, 0x6A, 0x75, 0x6D, 0x70, 0x73, 0x20, 0x6F, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6C, 0x61, 0x7A, 0x79, 
        0x00, 0x03, 0x04, 0x6B, 0x64, 0x6F, 0x67
    ]
}

fn some_more_spreadsheet_data() -> Vec<u8> {
    vec![
        0x00, 0x0F, 0x01, 0x05, 0x74, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6B, 0x20, 0x62, 0x72, 0x6F, 0x77, 0x6E, 
        0x00, 0x03, 0x03, 0x04, 0x20, 0x71, 0x75,
        0x00, 0x17, 0x0B, 0x05, 0x66, 0x6F, 0x78, 0x20, 0x6A, 0x75, 0x6D, 0x70, 0x73, 0x20, 0x6F, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6C, 0x61, 0x7A, 0x79, 
        0x00, 0x03, 0x04, 0x6B, 0x64, 0x6F, 0x67
    ]
}

fn some_login_data() -> Vec<u8> {
    vec![
        0x00, 0x01, 0x00, 0x09, 0x00, 0x06, 0x74, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6B, 0x20, 0x62, 0x72, 0x6F, 0x77, 0x6E, 
        0x00, 0x02, 0x00, 0x01, 0x00, 0x02, 0x20, 0x71, 0x75,
        0x01, 0x01, 0x00, 0x07, 0x00, 0x10, 0x66, 0x6F, 0x78, 0x20, 0x6A, 0x75, 0x6D, 0x70, 0x73, 0x20, 0x6F, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6C, 0x61, 0x7A, 0x79, 
        0x02, 0x01, 0x00, 0x01, 0x00, 0x02, 0x64, 0x6F, 0x67   
    ]
}

fn some_login_metadata() -> Vec<u8> {
    vec![
        0x00, 0x0F, 0x00, 0x74, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6B, 0x20, 0x62, 0x72, 0x6F, 0x77, 0x6E, 
        0x00, 0x17, 0x01, 0x66, 0x6F, 0x78, 0x20, 0x6A, 0x75, 0x6D, 0x70, 0x73, 0x20, 0x6F, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6C, 0x61, 0x7A, 0x79, 
        0x00, 0x03, 0x02, 0x64, 0x6F, 0x67   
    ]
}

fn some_notes_data() -> Vec<u8> {
    vec![
        0x05, 0x00, 0x0e, 0x00, b'l', b'a', b'b', b'e', b'l', b's', b'o', b'm', b'e', b' ', b'n', b'o', b't', b'e', b' ', b'd', b'a', b't', b'a',
        0x02, 0x00, 0x05, 0x01, b'l', b'a', b's', b'o', b'm', b'e', b' ',
    ]
}

fn some_vault_names() -> Vec<u8> {
    let vault_1_principal: Vec<u8> = some_vault_id().to_bytes().into();
    let vault_2_principal: Vec<u8> = some_user_id().to_bytes().into();
    let mut data = Vec::new();

    let mut key_1: Vec<u8> = Vec::new();
    
    key_1.extend_from_slice(&vault_1_principal);
    let val_1: Vec<u8> = b"My First Vault".to_vec();
    let principal_len = key_1.len() as u8;
    let name_len = val_1.len() as u16;

    let mut header = Vec::with_capacity(3);
    header.push(principal_len);
    header.push((name_len >> 8) as u8);
    header.push((name_len & 0xFF) as u8);

    data.extend_from_slice(&header);
    data.extend_from_slice(&key_1);
    data.extend_from_slice(&val_1);
    
    let mut key_2: Vec<u8> = Vec::new();
    
    key_2.extend_from_slice(&vault_2_principal);
    let val_2: Vec<u8> = b"Other Vault".to_vec();
    let principal_len = key_2.len() as u8;
    let name_len = val_2.len() as u16;

    let mut header = Vec::with_capacity(3);
    header.push(principal_len);
    header.push((name_len >> 8) as u8);
    header.push((name_len & 0xFF) as u8);

    data.extend_from_slice(&header);
    data.extend_from_slice(&key_2);
    data.extend_from_slice(&val_2);

    data
}

#[test]
pub fn test_deserialise_vault_names() {

    let data = some_vault_names();
    let user_id = some_user_id();
    let other_user_id = some_vault_id();
    let vault_id_1 = some_vault_id().to_bytes().to_vec();
    let vault_id_2 = some_user_id().to_bytes().to_vec();

    let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
    let vault_names_map = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(0))));

    // Deserialise into VaultNamesMap
    _vault_names_sync(user_id, &data, &vault_names_map);

    // Add extra noise associated with another user to ensure we filter properly for the caller
    _vault_names_sync(other_user_id, &data, &vault_names_map);

    assert_eq!(vault_names_map.borrow().len(), 4);

    // Use _get_vault_names to retrieve names
    let names = _get_vault_names(user_id, &vault_names_map);

    assert_eq!(names.names.get(&vault_id_1).unwrap(), &b"My First Vault".to_vec());
    assert_eq!(names.names.get(&vault_id_2).unwrap(), &b"Other Vault".to_vec());
    assert_eq!(names.names.len(), 2);
}


#[test]
pub fn test_deserialise_spreadsheet() {
    let data : Vec<u8> = some_spreadsheet_data();
    let cells = vault_core::api::deserialiser::deserialise_spreadsheet(data);
    assert_eq!(cells.cells[0].header.x, 0);
    assert_eq!(cells.cells[0].header.y, 2);
    assert_eq!(cells.cells[0].data, vec![0x74, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6B, 0x20, 0x62, 0x72, 0x6F, 0x77, 0x6E]);
    assert_eq!(cells.cells[1].header.x, 3);
    assert_eq!(cells.cells[1].header.y, 4);
    assert_eq!(cells.cells[2].header.x, 11);
    assert_eq!(cells.cells[2].header.y, 5);
    assert_eq!(cells.cells[2].data, vec![0x66, 0x6F, 0x78, 0x20, 0x6A, 0x75, 0x6D, 0x70, 0x73, 0x20, 0x6F, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6C, 0x61, 0x7A, 0x79]);
    assert_eq!(cells.cells[3].header.x, 4);
    assert_eq!(cells.cells[3].header.y, 107);
    assert_eq!(cells.cells[3].data, vec![0x64, 0x6F, 0x67]);
}

#[test]
pub fn test_vault_spreadsheet_sync () {
    let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
    let spreadsheet_map = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(0))));
    
    let user_id = some_user_id();
    let vault_id = some_vault_id();
    let data : Vec<u8> = some_spreadsheet_data();
    
    let key2 = SpreadsheetKey::new(user_id, vault_id, 3, 4);
    
    spreadsheet_map.borrow_mut().insert(key2.clone(), vault_core::vault_type::spreadsheet::SpreadsheetValue::new(vec![0x61, 0x62, 0x63]));
    vault_core::api::serial_api::_vault_spreadsheet_sync(user_id.clone(), vault_id.clone(), data.clone(), &spreadsheet_map);
    
    let spreadsheets = spreadsheet_map.borrow();
    
    let key1 = SpreadsheetKey::new(user_id, vault_id,  0, 2);
    let key3 = SpreadsheetKey::new(user_id, vault_id, 11, 5);
    let key4 = SpreadsheetKey::new(user_id, vault_id, 4, 107);

    let first = spreadsheets.get(&key1);
    let second = spreadsheets.get(&key2);
    let third = spreadsheets.get(&key3);
    let fourth = spreadsheets.get(&key4);
    assert_eq!(first.unwrap().data, vec![0x74, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6B, 0x20, 0x62, 0x72, 0x6F, 0x77, 0x6E]);
    assert_eq!(second.is_some(), false);
    assert_eq!(third.unwrap().data, vec![0x66, 0x6F, 0x78, 0x20, 0x6A, 0x75, 0x6D, 0x70, 0x73, 0x20, 0x6F, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6C, 0x61, 0x7A, 0x79]);
    assert_eq!(fourth.unwrap().data, vec![0x64, 0x6F, 0x67]);
}

#[test]
pub fn test_get_vault_data() {
    let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
    let spreadsheet_map = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(0))));
    
    let user_id = some_user_id();
    let vault_id = some_vault_id();
    let some_other_id = some_other_principal();
    let data : Vec<u8> = some_spreadsheet_data();
    let some_more_data : Vec<u8> = some_more_spreadsheet_data();

    // populate with target date
    vault_core::api::serial_api::_vault_spreadsheet_sync(user_id.clone(), vault_id.clone(), data.clone(), &spreadsheet_map);
    
    // add some noise
    vault_core::api::serial_api::_vault_spreadsheet_sync(user_id.clone(), some_other_id.clone(), data.clone(), &spreadsheet_map);
    vault_core::api::serial_api::_vault_spreadsheet_sync(some_other_id.clone(), vault_id.clone(), some_more_data.clone(), &spreadsheet_map);
    
    for entry in spreadsheet_map.borrow().iter() {
        let key = entry.key();
        let value = entry.value();
        println!("Key: x: {}, y: {}, principals: {:?} => Value: {:?}", key.x, key.y, key.principals, value.data);
    }

    let spreadsheet = vault_core::api::dev_api::_get_spreadsheet(user_id, vault_id, &spreadsheet_map);
    assert_eq!(spreadsheet.columns.len(), 3);
    assert_eq!(spreadsheet.columns.get(&0).unwrap().rows.get(&2).unwrap(), &vec![0x74, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6B, 0x20, 0x62, 0x72, 0x6F, 0x77, 0x6E]);
    assert_eq!(spreadsheet.columns.get(&11).unwrap().rows.get(&5).unwrap(), &vec![0x66, 0x6F, 0x78, 0x20, 0x6A, 0x75, 0x6D, 0x70, 0x73, 0x20, 0x6F, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6C, 0x61, 0x7A, 0x79]);
    assert_eq!(spreadsheet.columns.get(&3).is_none(), true);
    assert_eq!(spreadsheet.columns.get(&4).unwrap().rows.get(&107).unwrap(), &vec![0x64, 0x6F, 0x67]);
}

#[test]
pub fn test_get_logins() {
    let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
    let logins = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(0))));
    let logins_columns = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(1))));
    
    let user_id = some_user_id();
    let vault_id = some_vault_id();
    let login_data = some_login_data();
    let login_metadata = some_login_metadata();

    _login_data_sync(user_id.clone(), vault_id.clone(), login_data.clone(), &logins);
    _login_metadata_sync(user_id.clone(), vault_id.clone(), login_metadata.clone(), &logins_columns, &logins);
    
    let logins_data = _get_logins(user_id.clone(), vault_id.clone(), &logins, &logins_columns);
    assert_eq!(logins_data.columns.len(), 3);
}

#[test]
pub fn test_get_notes() {
    let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
    let notes = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(0))));

    let user_id = some_user_id();
    let vault_id = some_vault_id();
    let notes_data = some_notes_data();

    _secret_notes_sync(user_id, vault_id, notes_data, &notes);

    assert_eq!(notes.borrow().is_empty(), false);

    let get_notes = _get_notes(user_id.clone(), vault_id.clone(), &notes);

    assert_eq!(get_notes.notes.len(), 2);
}