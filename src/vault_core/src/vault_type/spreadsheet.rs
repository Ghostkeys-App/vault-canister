use std::u8;

use candid::{Principal};
use ic_stable_structures::storable::Storable;

pub struct SpreadsheetKey {
    // Combination of user_id and vault_id. We don't need to know which is which 
    // or be able to reconstruct them, just use them to uniquely ID the entry.
    pub principals: Vec<u8>,

    // X and Y coordinates of the cell in the spreadsheet. This is required by 
    // frontend to identify the cell.
    pub x: u8,
    pub y: u8,
}
impl SpreadsheetKey {
    pub fn new(user_id: Principal, vault_id: Principal, x: u8, y: u8) -> Self {
        let mut principals = Vec::new();
        principals.extend(user_id.to_bytes().iter());
        principals.extend(vault_id.to_bytes().iter());
        Self {
            principals,
            x,
            y,
        }
    }
    pub fn principals_match(&self, principals: &Vec<u8>) -> bool {
        self.principals.len() == principals.len() && self.principals == *principals
    }
}
impl Storable for SpreadsheetKey {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 50,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        let mut bytes = Vec::new();
        bytes.push(self.x);
        bytes.push(self.y);
        bytes.extend(self.principals.iter());
        bytes.into()
    }

    fn into_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.x);
        bytes.push(self.y);
        bytes.extend(self.principals.iter());
        bytes
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let x = bytes[0];
        let y = bytes[1];
        let principals = bytes[2..].to_vec();
        
        Self {
            principals,
            x,
            y,
        }
    }
}

impl Ord for SpreadsheetKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_bytes().cmp(&other.to_bytes())
    }
}

impl PartialOrd for SpreadsheetKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SpreadsheetKey {}

impl PartialEq for SpreadsheetKey {
    fn eq(&self, other: &Self) -> bool {
        match self.to_bytes().cmp(&other.to_bytes()) {
            std::cmp::Ordering::Equal => true,
            _ => false,
        }
    }
}

impl Clone for SpreadsheetKey {
    fn clone(&self) -> Self {
        Self {
            principals: self.principals.clone(),
            x: self.x,
            y: self.y,
        }
    }
}

pub struct SpreadsheetValue {
    pub data: Vec<u8>,
}

impl SpreadsheetValue {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}
impl Storable for SpreadsheetValue {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
    
    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        self.data.clone().into()
    }
    
    fn into_bytes(self) -> Vec<u8> {
        self.data
    }
    
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self { data: bytes.into_owned() }
    }
}

pub struct LoginValue {
    pub username: Vec<u8>,
    pub password: Vec<u8>
}
impl LoginValue {
    pub fn new(username: Vec<u8>, password: Vec<u8>) -> Self {
        Self {
            username,
            password
        }
    }
}
impl Storable for LoginValue {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        let uname_len: u16 = self.username.len() as u16;
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(uname_len.to_be_bytes());
        bytes.extend(self.username.iter());
        bytes.extend(self.password.iter());
        bytes.into()
    }

    fn into_bytes(self) -> Vec<u8> {
        let uname_len: u16 = self.username.len() as u16;
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(uname_len.to_be_bytes());
        bytes.extend(self.username.iter());
        bytes.extend(self.password.iter());
        bytes
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let uname_len = u16::from_be_bytes([bytes[0], bytes[1]]);
        let username = bytes[2..2 + uname_len as usize].to_vec();
        let password = bytes[2 + uname_len as usize..].to_vec();
        Self {
            username,
            password
        }
    }
}

pub struct ColumnKey {
    pub principals: Vec<u8>,
    pub x: u8
}
impl ColumnKey {
    pub fn new(user_id: Principal, vault_id: Principal, x: u8) -> Self {
        let mut principals = Vec::new();
        principals.extend(user_id.to_bytes().iter());
        principals.extend(vault_id.to_bytes().iter());
        Self {
            principals,
            x,
        }
    }
    pub fn principals_match(&self, principals: &Vec<u8>) -> bool {
        self.principals.len() == principals.len() && self.principals == *principals
    }
}
impl Storable for ColumnKey {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 50,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        let mut bytes = Vec::new();
        bytes.push(self.x);
        bytes.extend(self.principals.iter());
        bytes.into()
    }

    fn into_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.x);
        bytes.extend(self.principals.iter());
        bytes
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let x = bytes[0];
        let principals = bytes[1..].to_vec();
        
        Self {
            principals,
            x,
        }
    }
}

impl Ord for ColumnKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_bytes().cmp(&other.to_bytes())
    }
}

impl PartialOrd for ColumnKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ColumnKey {}

impl PartialEq for ColumnKey {
    fn eq(&self, other: &Self) -> bool {
        match self.to_bytes().cmp(&other.to_bytes()) {
            std::cmp::Ordering::Equal => true,
            _ => false,
        }
    }
}

impl Clone for ColumnKey {
    fn clone(&self) -> Self {
        Self {
            principals: self.principals.clone(),
            x: self.x,
        }
    }
}

pub struct ColumnData {
    pub hidden: bool,
    pub name: Vec<u8>
}
impl ColumnData {
    pub fn new(hidden: bool, name: Vec<u8>) -> Self {
        Self { hidden, name }
    }
}

impl Storable for ColumnData {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
    
    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        let mut data : Vec<u8> = Vec::new();
        if self.hidden {
            data.push(0x0);
        }
        else {
            data.push(0x1);
        }
        data.extend(self.name.iter());
        data.into()
    }
    
    fn into_bytes(self) -> Vec<u8> {
        let mut data : Vec<u8> = Vec::new();
        if self.hidden {
            data.push(0x0);
        }
        else {
            data.push(0x1);
        }
        data.extend(self.name.iter());
        data
    }
    
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let hidden = if bytes[0] > 0 { true } else { false };
        let name = bytes[1..].to_vec();
        Self {
            hidden,
            name
        }
    }
}