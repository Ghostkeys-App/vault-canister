use candid::{CandidType, Deserialize, Encode, Decode};
use ic_stable_structures::{storable::Storable, storable::Bound};
use std::{borrow::Cow};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FlexGridDataKey {
    pub col: u32,         
    pub row: u32, 
}

// Types
pub type FlexGridData = HashMap<FlexGridDataKey, String>;
pub type FlexGridColumns = HashMap<String, (u32, bool)>; 

// Uses Candid for serialization - this is not efficient, but simple.
// TODO: Change to serde serialization
impl Storable for FlexGridDataKey {
    const BOUND: Bound = Bound::Unbounded;
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode FlexGridDataKey"))
    }
    fn into_bytes(self) -> Vec<u8> {
        Encode!(&self).expect("Failed to encode FlexGridDataKey").into()
    }
    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(&bytes, FlexGridDataKey).expect("Failed to decode FlexGridDataKey")
    }
}