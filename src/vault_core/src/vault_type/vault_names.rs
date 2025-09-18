use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::storable::Storable;

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VaultNameKey {
    pub principals: Vec<u8>
}
impl VaultNameKey {
    pub fn new(user_id: Principal, vault_id: &Vec<u8>) -> Self {
        let mut principals = Vec::new();
        principals.extend(user_id.to_bytes().iter());
        principals.extend(vault_id.iter());
        Self { principals }
    }
    pub fn user_principals_match(&self, user_id: &Vec<u8>) -> bool {
        let mut index = 0;
        while index < user_id.len() && index < self.principals.len() {
            if user_id[index] != self.principals[index] {
                return false;
            }
            index += 1;
        }
        return true;
    }
    pub fn principals_match(&self, principals: &Vec<u8>) -> bool {
        self.principals.len() == principals.len() && self.principals == *principals
    }
}
impl Storable for VaultNameKey {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded { max_size: 512, is_fixed_size: false };
    
    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        self.principals.clone().into()
    }
    
    fn into_bytes(self) -> Vec<u8> {
        self.principals
    }
    
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self { principals: bytes.to_vec() }
    }
}

pub struct VaultNameValue {
    pub name: Vec<u8>
}
impl VaultNameValue {
    pub fn new(vault_name: &Vec<u8>) -> Self {
        Self { name: vault_name.clone() }
    }
}
impl Storable for VaultNameValue {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded { max_size: 512, is_fixed_size: false };
    
    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        self.name.clone().into()
    }
    
    fn into_bytes(self) -> Vec<u8> {
        self.name
    }
    
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self { name: bytes.to_vec() }
    }
}