use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Bank {
    pub code: String,
    pub name: String,
    pub kana: String,
    pub hira: String,
    pub roma: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Branch {
    pub code: String,
    pub name: String,
    pub kana: String,
    pub hira: String,
    pub roma: String,
}

include!("generated/data.rs");

/*
impl Bank {
    pub fn get(code: &str) -> Option<Self> {
        get_bank(code)
    }

    pub fn branches(&self) -> BTreeMap<String, Branch> {
        get_branches_by_bank_code(self.code.as_str())
    }

    pub fn get_branch(&self, code: &str) -> Option<Branch> {
        get_branches_by_bank_and_branch_code(self.code.as_str(), code)
    }
}
*/
