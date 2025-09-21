use std::collections::HashMap;
use ic_stable_structures::Storable;
use candid::{Principal, CandidType, Deserialize};

use crate::{stable::types::{ColumnsInfo, GeneralState, LoginsColumns, LoginsMap, NotesMap, SpreadsheetMap, VaultNamesMap}};

/* 
    Vault names devapi structures
*/

#[derive(CandidType, Deserialize)]
pub struct VaultNames {
    pub names: HashMap<Vec<u8>, Vec<u8>>
}
pub fn _get_vault_names(user_id: Principal, vnm: &VaultNamesMap) -> VaultNames {
    let vault_names = vnm.borrow();
    let uid_bytes = user_id.as_slice().to_vec();
    let mut names_map: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();

    for entry in vault_names.iter() {
        let (key, value) = entry.into_pair();
        if !key.user_principals_match(&uid_bytes) {
            continue;
        }
        let vault_id = key.principals[uid_bytes.len()..].to_vec();
        names_map.insert(vault_id, value.name);
    }

    VaultNames { names:  names_map }
}

pub fn _get_vault_name(user_id: Principal, vault_id: Principal, vnm: &VaultNamesMap) -> Vec<u8> {
    let compare_principals: Vec<u8> = {
        let mut principals = Vec::new();
        principals.extend(user_id.to_bytes().iter());
        principals.extend(vault_id.to_bytes().iter());
        principals
    };
    for entry in vnm.borrow().iter() {
        if entry.key().principals_match(&compare_principals) {
            return entry.value().name;
        }
    }
    return Vec::new();
}



/* 
    Spreadsheet devapi structures.
*/

pub type FlexGridColumns = HashMap<u8, (Vec<u8>, bool)>;
pub fn _get_columns_info(user_id: Principal, vault_id: Principal, sc: &ColumnsInfo) -> FlexGridColumns {
    let sc = sc.borrow();
    let mut columns : FlexGridColumns = HashMap::new();

    let compare_principals: Vec<u8> = {
        let mut principals = Vec::new();
        principals.extend(user_id.to_bytes().iter());
        principals.extend(vault_id.to_bytes().iter());
        principals
    };

    sc.iter().for_each(|entry| {
        let (key, value) = entry.into_pair();
        if key.principals_match(&compare_principals) {
            columns.entry(key.x).or_insert_with(|| (value.name, value.hidden ));
        }
    });

    columns
}

#[derive(CandidType, Deserialize)]
pub struct SpreadsheetColumn {
    pub rows: HashMap<u8, Vec<u8>>, // key is y
}

#[derive(CandidType, Deserialize)]
pub struct Spreadsheet {
    pub columns: HashMap<u8, SpreadsheetColumn>, // key is x
}

pub fn _get_spreadsheet(user_id: Principal, vault_id: Principal, sm: &SpreadsheetMap) -> Spreadsheet {
    let cells = sm.borrow();
    let mut spreadsheet = Spreadsheet {
        columns: HashMap::new(),
    };

    let compare_principals: Vec<u8> = {
        let mut principals = Vec::new();
        principals.extend(user_id.to_bytes().iter());
        principals.extend(vault_id.to_bytes().iter());
        principals
    };

    cells.iter().for_each(|entry| {
        let key = entry.key();
        if key.principals_match(&compare_principals) {
            spreadsheet.columns
                .entry(key.x)
                .or_insert_with(|| SpreadsheetColumn { rows: HashMap::new() })
                .rows
                .insert(key.y, entry.value().data.clone());
        }
    });

    spreadsheet
}

/*
    Login devapi structures.
*/

#[derive(CandidType, Deserialize)]
pub struct LoginColumn {
    pub label : Vec<u8>,
    pub rows: HashMap<u8, Vec<u8>>, // key is y
}

#[derive(CandidType, Deserialize)]
pub struct Logins {
    pub columns: HashMap<u8, LoginColumn>, // key is x
}

pub fn _get_logins(user_id: Principal, vault_id: Principal, lm: &LoginsMap, lc: &LoginsColumns) -> Logins {
    let entries = lm.borrow();
    let mut logins = Logins {
        columns: HashMap::new(),
    };

    let compare_principals: Vec<u8> = {
        let mut principals = Vec::new();
        principals.extend(user_id.to_bytes().iter());
        principals.extend(vault_id.to_bytes().iter());
        principals
    };

    for entry in lc.borrow().iter() {
        let key = entry.key();
        if !key.principals_match(&compare_principals) {
            continue;
        }
        let column: LoginColumn = LoginColumn { label: entry.value().clone(), rows: HashMap::new() };
        logins.columns.insert(key.x, column);
    }

    entries.iter().for_each(|entry| {
        let key = entry.key();
        if key.principals_match(&compare_principals) {
            let column = logins.columns.get_mut(&key.x);
            if column.is_none() {
                ic_cdk::trap(format!("Login column {} has no associated label metadata", &key.x));
            }
            else {
                column.unwrap().rows
                .insert(key.y, entry.value().data.clone());
            }
        }
    });

    logins
}

#[derive(CandidType, Deserialize)]
pub struct Note {
    pub label: Vec<u8>,
    pub note: Vec<u8>
}


#[derive(CandidType, Deserialize)]
pub struct Notes{
    pub notes: HashMap<u8, Note>
}

pub fn _get_notes(user_id: Principal, vault_id: Principal, nm: &NotesMap) -> Notes {
    let nm = nm.borrow();

    let compare_principals: Vec<u8> = {
        let mut principals = Vec::new();
        principals.extend(user_id.to_bytes().iter());
        principals.extend(vault_id.to_bytes().iter());
        principals
    };

    let mut notes = Notes {
        notes: HashMap::new()
    };

    nm.iter().for_each(|entry| {
        let key = entry.key();
        if key.principals_match(&compare_principals) {
            let value = entry.value();
            notes.notes.insert(key.index, Note { label: value.label, note: value.note });
        }
    });

    notes
}

/*
    Global fetches
*/
#[derive(CandidType, Deserialize)]
pub struct VaultData {
    pub vault_name: Vec<u8>,
    pub spreadsheet_columns: FlexGridColumns,
    pub spreadsheet: Spreadsheet,
    pub logins: Logins,
    pub notes: Notes,
}

pub fn _get_vault(vault_name: &Vec<u8>, user_id: Principal, vault_id: Principal, state: &GeneralState) -> VaultData {
    let spreadsheet_columns = _get_columns_info(user_id, vault_id, &state.spreadsheet_columns);
    let spreadsheet = _get_spreadsheet(user_id, vault_id, &state.spreadsheet_map);
    let logins = _get_logins(user_id, vault_id, &state.logins_map, &state.logins_columns);
    let notes = _get_notes(user_id, vault_id, &state.notes_map);

    VaultData {
        vault_name: vault_name.to_vec(),
        spreadsheet_columns,
        spreadsheet,
        logins,
        notes
    }
}

#[derive(CandidType, Deserialize)]
pub struct UserVaults {
    pub vaults: HashMap<Vec<u8>, VaultData>
}

pub fn _get_user_vaults(user_id: Principal, state: &GeneralState) -> UserVaults {
    let vault_names = _get_vault_names(user_id, &state.vault_names_map);
    let mut vaults = HashMap::new();

    for vault in vault_names.names.iter() {
        let (vault_id, vault_name) = vault;
        let vault_data = _get_vault(vault_name, user_id, Principal::from_bytes(vault_id.to_bytes()), state);
        vaults.insert(vault_id.to_vec(), vault_data);
    }

    UserVaults { vaults }
}