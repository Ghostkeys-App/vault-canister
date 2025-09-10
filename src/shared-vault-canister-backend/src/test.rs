use std::cell::RefCell;

use candid::Principal;
use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager}, StableBTreeMap, DefaultMemoryImpl};
use vault_core::{api::{dev_api::_get_logins, serial_api::{_login_data_sync, _login_metadata_sync}}, vault_type::spreadsheet::SpreadsheetKey};

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
        0x00, 0x0F, 0x00, 0x01, 0x74, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6B, 0x20, 0x62, 0x72, 0x6F, 0x77, 0x6E, 
        0x00, 0x03, 0x00, 0x02, 0x20, 0x71, 0x75,
        0x00, 0x17, 0x01, 0x01, 0x66, 0x6F, 0x78, 0x20, 0x6A, 0x75, 0x6D, 0x70, 0x73, 0x20, 0x6F, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6C, 0x61, 0x7A, 0x79, 
        0x00, 0x03, 0x02, 0x01, 0x64, 0x6F, 0x67   
    ]
}

fn some_login_metadata() -> Vec<u8> {
    vec![
        0x00, 0x0F, 0x00, 0x74, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6B, 0x20, 0x62, 0x72, 0x6F, 0x77, 0x6E, 
        0x00, 0x17, 0x01, 0x66, 0x6F, 0x78, 0x20, 0x6A, 0x75, 0x6D, 0x70, 0x73, 0x20, 0x6F, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6C, 0x61, 0x7A, 0x79, 
        0x00, 0x03, 0x02, 0x64, 0x6F, 0x67   
    ]
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
    assert_eq!(logins_data.columns.len(), 2);
}