use candid::{Principal, CandidType, Deserialize};
use ic_stable_structures::storable::Storable;

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoginSiteKey {
    pub principals: Vec<u8>,
    pub x: u8,
}
impl LoginSiteKey {
    pub fn new(user_id: Principal, vault_id: Principal, x: u8) -> Self {
        let mut principals = Vec::new();
        principals.extend(user_id.as_slice());
        principals.extend(vault_id.as_slice());
        Self {
            principals,
            x,
        }
    }
    pub fn principals_match(&self, principals: &Vec<u8>) -> bool {
        self.principals.len() == principals.len() && self.principals == *principals
    }
}
impl Storable for LoginSiteKey {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded { max_size: 512, is_fixed_size: false };

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

    fn from_bytes(bytes: std::borrow::Cow<'_, [u8]>) -> Self {
        let x = bytes[0];
        let len_principals = bytes.len() - 1;
        let principals = bytes[1..len_principals + 1].to_vec();
        
        Self {
            x,
            principals,
        }
    }
}
