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