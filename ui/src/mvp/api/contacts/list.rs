//! A list of contacts is a simple key value store.
//! The key is the address, and the value is the label.
//! The label is a user defined string that is used to identify the address.

use std::{collections::BTreeMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::*;

pub type ContactKey = Address;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ContactValue {
    pub label: String,
    pub class: Class,
    pub artifact: Option<PathBuf>,
}

/// A group list of contacts, sorted and stored by address key.
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ContactList {
    addresses: BTreeMap<ContactKey, ContactValue>,
}

impl ContactList {
    pub fn new() -> Self {
        Self {
            addresses: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, address: Address, contact: ContactValue) -> Option<ContactValue> {
        self.addresses.insert(address, contact)
    }

    pub fn get(&self, address: &Address) -> Option<&ContactValue> {
        self.addresses.get(address)
    }

    pub fn remove(&mut self, address: &Address) -> Option<ContactValue> {
        self.addresses.remove(address)
    }

    pub fn contains(&self, address: &Address) -> bool {
        self.addresses.contains_key(address)
    }

    pub fn get_all(&self) -> Vec<(&Address, &ContactValue)> {
        self.addresses.iter().collect()
    }

    pub fn clear(&mut self) {
        self.addresses.clear();
    }

    pub fn len(&self) -> usize {
        self.addresses.len()
    }

    /// Try to add a String to the list.
    pub fn try_add(
        &mut self,
        address: String,
        contact: ContactValue,
    ) -> anyhow::Result<(), anyhow::Error> {
        let address = address
            .parse::<Address>()
            .map_err(|e| anyhow::anyhow!("Failed to parse address: {}", e.to_string()))?;

        self.add(address, contact);
        Ok(())
    }
}
