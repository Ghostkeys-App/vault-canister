use std::collections::HashMap;
use ic_stable_structures::Storable;
use candid::{Principal, CandidType, Deserialize};

use crate::stable::types::{LoginsColumns, LoginsMap, NotesMap, SpreadsheetMap};


/* 
    Spreadsheet devapi structures.
*/
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

    let column_labels: HashMap<u8, Vec<u8>> = {
        let mut labels = HashMap::new();
        for entry in lc.borrow().iter() {
            let key = entry.key();
            if !key.principals_match(&compare_principals) {
                continue;
            }
            labels.insert(key.x, entry.value().clone());
        }
        labels
    };

    entries.iter().for_each(|entry| {
        let key = entry.key();
        if key.principals_match(&compare_principals) {
            logins.columns
                .entry(key.x)
                .or_insert_with(|| LoginColumn { label: column_labels.get(&key.x).cloned().unwrap_or_default(), rows: HashMap::new() })
                .rows
                .insert(key.y, entry.value().data.clone());
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