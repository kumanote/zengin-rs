mod error;
pub use error::Error;
pub type Result<T> = core::result::Result<T, Error>;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static SOURCE_DATA: OnceCell<SourceData> = OnceCell::new();

pub fn load_all_data<P: AsRef<Path>>(source_data_dir: Option<P>) -> Result<()> {
    match SOURCE_DATA.get() {
        Some(_) => Ok(()),
        None => {
            let source_data = read_source_data(source_data_dir)?;
            SOURCE_DATA
                .set(source_data)
                .expect("source data must be stored...");
            Ok(())
        }
    }
}

pub fn get_all_banks() -> Result<BTreeMap<String, Bank>> {
    let source_data = SOURCE_DATA
        .get()
        .expect("source data must be loaded in advance...");
    Ok(source_data
        .into_iter()
        .map(|(bank_code, bank_with_branches)| (bank_code.clone(), bank_with_branches.bank.clone()))
        .collect())
}

pub fn get_bank(bank_code: &str) -> Result<Option<Bank>> {
    let source_data = SOURCE_DATA
        .get()
        .expect("source data must be loaded in advance...");
    let bank_with_branches = source_data.get(bank_code);
    Ok(bank_with_branches.map(|b| b.bank.clone()))
}

pub fn get_bank_branches(bank_code: &str) -> Result<Option<BTreeMap<String, Branch>>> {
    let source_data = SOURCE_DATA
        .get()
        .expect("source data must be loaded in advance...");
    let bank_with_branches = source_data.get(bank_code);
    Ok(bank_with_branches.map(|b| b.branches.clone()))
}

pub fn get_branch(bank_code: &str, branch_code: &str) -> Result<Option<Branch>> {
    let source_data = SOURCE_DATA
        .get()
        .expect("source data must be loaded in advance...");
    Ok(match source_data.get(bank_code) {
        Some(bank_with_branches) => bank_with_branches
            .branches
            .get(branch_code)
            .map(|branch| branch.clone()),
        None => None,
    })
}

pub fn get_all_banks_from_file<P: AsRef<Path>>(
    source_data_dir: P,
) -> Result<BTreeMap<String, Bank>> {
    let bank_source_data = read_bank_source_data(source_data_dir)?;
    Ok(bank_source_data)
}
pub fn get_bank_from_file<P: AsRef<Path>>(
    bank_code: &str,
    source_data_dir: P,
) -> Result<Option<Bank>> {
    let bank_source_data = read_bank_source_data(source_data_dir)?;
    let bank = bank_source_data.get(bank_code).map(Clone::clone);
    Ok(bank)
}

pub fn get_bank_branches_from_file<P: AsRef<Path>>(
    bank_code: &str,
    source_data_dir: P,
) -> Result<Option<BTreeMap<String, Branch>>> {
    let bank = get_bank_from_file(bank_code, source_data_dir.as_ref())?;
    if bank.is_none() {
        return Ok(None);
    }
    let branch_source_data = read_branch_source_data(source_data_dir, bank_code)?;
    Ok(Some(branch_source_data))
}

pub fn get_branch_from_file<P: AsRef<Path>>(
    bank_code: &str,
    branch_code: &str,
    source_data_dir: P,
) -> Result<Option<Branch>> {
    let bank = get_bank_from_file(bank_code, source_data_dir.as_ref())?;
    if bank.is_none() {
        return Ok(None);
    }
    let branch_source_data = read_branch_source_data(source_data_dir, bank_code)?;
    let branch = branch_source_data.get(branch_code).map(Clone::clone);
    Ok(branch)
}

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

type BankSourceData = BTreeMap<String, Bank>;
type BranchSourceData = BTreeMap<String, Branch>;
type SourceData = BTreeMap<String, BankWithBranches>;

#[derive(Debug, Clone, PartialEq)]
struct BankWithBranches {
    bank: Bank,
    branches: BranchSourceData,
}

fn read_source_data<P: AsRef<Path>>(source_data_dir: Option<P>) -> Result<SourceData> {
    let default_dir = {
        let mut result = std::env::current_dir().unwrap();
        result.push("source-data/data");
        result
    };
    let source_data_dir = match &source_data_dir {
        Some(p) => p.as_ref(),
        None => default_dir.as_path(),
    };
    let bank_source_data = read_bank_source_data(source_data_dir)?;
    let mut result = BTreeMap::new();
    for (bank_code, bank) in bank_source_data {
        let branches = read_branch_source_data(source_data_dir, bank_code.as_str())?;
        result.insert(bank_code, BankWithBranches { bank, branches });
    }
    Ok(result)
}

fn read_bank_source_data<P: AsRef<Path>>(source_data_dir: P) -> Result<BankSourceData> {
    let filepath = source_data_dir.as_ref().join("banks.json");
    let file = File::open(&filepath)?;
    let mut reader = std::io::BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(serde_json::from_str(&content)?)
}

fn read_branch_source_data<P: AsRef<Path>>(
    source_data_dir: P,
    bank_code: &str,
) -> Result<BranchSourceData> {
    let filepath = source_data_dir
        .as_ref()
        .join(format!("branches/{}.json", bank_code));
    let file = File::open(&filepath)?;
    let mut reader = std::io::BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(serde_json::from_str(&content)?)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    #[serial_test::serial]
    fn test_load_data() {
        let path: Option<PathBuf> = None;
        load_all_data(path).expect("data must be loaded");
        let bank_code = "0001";
        let branch_code = "988";
        let bank = get_bank(bank_code).unwrap().unwrap();
        assert_eq!(
            bank,
            Bank {
                code: "0001".to_owned(),
                name: "みずほ".to_owned(),
                kana: "ミズホ".to_owned(),
                hira: "みずほ".to_owned(),
                roma: "mizuho".to_owned(),
            }
        );
        let branches = get_bank_branches(bank_code).unwrap().unwrap();
        assert_eq!(branches.len(), 494);
        let branch = get_branch(bank_code, branch_code).unwrap().unwrap();
        assert_eq!(
            branch,
            Branch {
                code: "988".to_owned(),
                name: "カゴメ".to_owned(),
                kana: "カゴメ".to_owned(),
                hira: "かごめ".to_owned(),
                roma: "kagome".to_owned(),
            }
        );
        let all_banks = get_all_banks().unwrap();
        let bank_from_hash_map = all_banks.get("0001").unwrap();
        assert_eq!(bank_from_hash_map, &bank);
    }

    #[test]
    fn test_from_file() {
        let source_data_dir = {
            let mut result = std::env::current_dir().unwrap();
            result.push("source-data/data");
            result
        };
        let bank_code = "0001";
        let branch_code = "988";
        let bank = get_bank_from_file(bank_code, &source_data_dir)
            .unwrap()
            .unwrap();
        assert_eq!(
            bank,
            Bank {
                code: "0001".to_owned(),
                name: "みずほ".to_owned(),
                kana: "ミズホ".to_owned(),
                hira: "みずほ".to_owned(),
                roma: "mizuho".to_owned(),
            }
        );
        let branches = get_bank_branches_from_file(bank_code, &source_data_dir)
            .unwrap()
            .unwrap();
        assert_eq!(branches.len(), 494);
        let branch = get_branch_from_file(bank_code, branch_code, &source_data_dir)
            .unwrap()
            .unwrap();
        assert_eq!(
            branch,
            Branch {
                code: "988".to_owned(),
                name: "カゴメ".to_owned(),
                kana: "カゴメ".to_owned(),
                hira: "かごめ".to_owned(),
                roma: "kagome".to_owned(),
            }
        );
        let all_banks = get_all_banks_from_file(&source_data_dir).unwrap();
        let bank_from_hash_map = all_banks.get("0001").unwrap();
        assert_eq!(bank_from_hash_map, &bank);
    }
}
