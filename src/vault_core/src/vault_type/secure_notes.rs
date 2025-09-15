use candid::{CandidType, Deserialize};
use ic_stable_structures::{Storable};

pub struct SecureNote {
    pub label: Vec<u8>,
    pub note: Vec<u8>
}
impl SecureNote {
    pub fn new(label: Vec<u8>, note: Vec<u8>) -> Self {
        Self {
            label,
            note
        }
    }
}
impl Storable for SecureNote {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        let label_size : u8 = self.label.len().to_le_bytes()[0];
        let mut bytes = Vec::new();
        bytes.push(label_size);
        bytes.extend(self.label.iter());
        bytes.extend(self.note.iter());
        bytes.into()
    }

    fn into_bytes(self) -> Vec<u8> {
        let label_size : u8 = self.label.len().to_le_bytes()[0];
        let mut bytes = Vec::new();
        bytes.push(label_size);
        bytes.extend(self.label.iter());
        bytes.extend(self.note.iter());
        bytes
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let label_size: usize = usize::from(bytes[0]);
        let label = bytes[1..label_size].to_vec();
        let note = bytes[label_size..].to_vec();
        Self {
            label,
            note
        }
    }
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecureNoteKey {
    pub index : u8,
    pub principals: Vec<u8>
}
impl SecureNoteKey {
    pub fn principals_match(&self, principals: &Vec<u8>) -> bool {
        self.principals.len() == principals.len() && self.principals == *principals
    }
}
impl Storable for SecureNoteKey {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded { max_size: 512, is_fixed_size: false };
    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        let mut bytes = Vec::new();
        bytes.push(self.index);
        bytes.extend(self.principals.iter());
        bytes.into()
    }

    fn into_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.index);
        bytes.extend(self.principals.iter());
        bytes
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let index = bytes[0];
        let principals = bytes[1..].to_vec();
        Self {
            index,
            principals
        }
    }
}