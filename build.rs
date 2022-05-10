use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Bank {
    code: String,
    name: String,
    kana: String,
    hira: String,
    roma: String,
}

impl Bank {
    fn generate_code(&self) -> String {
        format!(
            r###"Bank {{ code: "{}".to_owned(), name: "{}".to_owned(), kana: "{}".to_owned(), hira: "{}".to_owned(), roma: "{}".to_owned() }}"###,
            self.code, self.name, self.kana, self.hira, self.roma
        )
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Branch {
    code: String,
    name: String,
    kana: String,
    hira: String,
    roma: String,
}

impl Branch {
    fn generate_code(&self) -> String {
        format!(
            r###"Branch {{ code: "{}".to_owned(), name: "{}".to_owned(), kana: "{}".to_owned(), hira: "{}".to_owned(), roma: "{}".to_owned() }}"###,
            self.code, self.name, self.kana, self.hira, self.roma
        )
    }
}

struct BankWithBranches {
    bank: Bank,
    branches: BranchSourceData,
}

type BankSourceData = HashMap<String, Bank>;
type BranchSourceData = HashMap<String, Branch>;
type SourceData = HashMap<String, BankWithBranches>;

fn read_source_data(source_data_dir: &PathBuf) -> SourceData {
    let bank_source_data = read_bank_source_data(source_data_dir);
    bank_source_data
        .into_iter()
        .map(|(bank_code, bank)| {
            let branches = read_branch_source_data(source_data_dir, bank_code.as_str());
            (bank_code, BankWithBranches { bank, branches })
        })
        .collect()
}

fn read_bank_source_data(source_data_dir: &PathBuf) -> BankSourceData {
    let filepath = source_data_dir.join("banks.json");
    let file = File::open(&filepath).expect("Failed to open banks.json file...");
    let mut reader = std::io::BufReader::new(file);
    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .expect("Failed to read banks.json file...");
    serde_json::from_str(&content).expect("Cannot parse banks.json file...")
}

fn read_branch_source_data(source_data_dir: &PathBuf, bank_code: &str) -> BranchSourceData {
    let filepath = source_data_dir.join(format!("branches/{}.json", bank_code));
    let file = File::open(&filepath).expect("Failed to open branches.json file...");
    let mut reader = std::io::BufReader::new(file);
    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .expect("Failed to read branches.json file...");
    serde_json::from_str(&content).expect("Cannot parse branches.json file...")
}

fn generate_code(data: &SourceData) -> String {
    format!(
        r###"{}

{}
"###,
        generate_get_bank_code(data),
        generate_get_branches_by_bank_and_branch_code(data),
    )
}

fn generate_all_banks_code(data: &SourceData) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice("pub fn all_banks() -> BTreeMap<String, Bank> {\n".as_bytes());
    bytes.extend_from_slice("    let mut map = BTreeMap::new();\n".as_bytes());
    for (bank_code, bank) in data {
        let code = format!(
            r###"map.insert("{}".to_owned(), {});"###,
            bank_code,
            bank.bank.generate_code()
        );
        bytes.extend_from_slice(format!("    {}\n", code).as_bytes());
    }
    bytes.extend_from_slice("    map\n".as_bytes());
    bytes.extend_from_slice("}".as_bytes());
    String::from_utf8(bytes).unwrap()
}

fn generate_get_bank_code(data: &SourceData) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice("pub fn get_bank(code: &str) -> Option<Bank> {\n".as_bytes());
    bytes.extend_from_slice("    match code {\n".as_bytes());
    for (bank_code, bank) in data {
        let code = format!(
            r###""{}" => Some({}),"###,
            bank_code,
            bank.bank.generate_code()
        );
        bytes.extend_from_slice(format!("        {}\n", code).as_bytes());
    }
    bytes.extend_from_slice("        _ => None,\n".as_bytes());
    bytes.extend_from_slice("    }\n".as_bytes());
    bytes.extend_from_slice("}".as_bytes());
    String::from_utf8(bytes).unwrap()
}

fn generate_get_branches_by_bank_code(data: &SourceData) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice(
        "pub fn get_branches_by_bank_code(bank_code: &str) -> BTreeMap<String, Branch> {\n"
            .as_bytes(),
    );
    bytes.extend_from_slice("    match bank_code {\n".as_bytes());
    for (bank_code, bank) in data {
        let mut branches_code_bytes: Vec<u8> = Vec::new();
        branches_code_bytes.extend_from_slice(
            r###"{
            let mut map = BTreeMap::new();
"###
            .as_bytes(),
        );
        for (branch_code, branch) in &bank.branches {
            let code = format!(
                r###"map.insert("{}".to_owned(), {});"###,
                branch_code,
                branch.generate_code()
            );
            branches_code_bytes.extend_from_slice(format!("            {}\n", code).as_bytes());
        }
        branches_code_bytes.extend_from_slice("            map\n".as_bytes());
        branches_code_bytes.extend_from_slice("        }".as_bytes());

        let code = format!(
            r###""{}" => {},"###,
            bank_code,
            String::from_utf8(branches_code_bytes).unwrap()
        );
        bytes.extend_from_slice(format!("        {}\n", code).as_bytes());
    }
    bytes.extend_from_slice("        _ => BTreeMap::new(),\n".as_bytes());
    bytes.extend_from_slice("    }\n".as_bytes());
    bytes.extend_from_slice("}".as_bytes());
    String::from_utf8(bytes).unwrap()
}

fn generate_get_branches_by_bank_and_branch_code(data: &SourceData) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice(
        "pub fn get_branches_by_bank_and_branch_code(bank_code: &str, branch_code: &str) -> Option<Branch> {\n"
            .as_bytes(),
    );
    bytes.extend_from_slice("    match bank_code {\n".as_bytes());
    for (bank_code, bank) in data {
        let mut branches_code_bytes: Vec<u8> = Vec::new();
        branches_code_bytes.extend_from_slice(
            r###"match branch_code {
"###
            .as_bytes(),
        );
        for (branch_code, branch) in &bank.branches {
            let code = format!(
                r###""{}" => Some({}),"###,
                branch_code,
                branch.generate_code()
            );
            branches_code_bytes.extend_from_slice(format!("            {}\n", code).as_bytes());
        }
        branches_code_bytes.extend_from_slice("            _ => None,\n".as_bytes());
        branches_code_bytes.extend_from_slice("        }".as_bytes());
        let code = format!(
            r###""{}" => {},"###,
            bank_code,
            String::from_utf8(branches_code_bytes).unwrap()
        );
        bytes.extend_from_slice(format!("        {}\n", code).as_bytes());
    }
    bytes.extend_from_slice("        _ => None,\n".as_bytes());
    bytes.extend_from_slice("    }\n".as_bytes());
    bytes.extend_from_slice("}".as_bytes());
    String::from_utf8(bytes).unwrap()
}

fn flush_code(dest_dir: &PathBuf, code: String) {
    let filepath = dest_dir.join("data.rs");
    let mut output = File::create(&filepath).unwrap();
    output
        .write(code.as_bytes())
        .expect("Cannot write generated data.rs");
}

fn main() {
    let source_data_dir = {
        let d = env::current_dir().unwrap();
        d.join("source-data/data")
    };
    let dest_dir = {
        let d = env::current_dir().unwrap();
        d.join("src/generated")
    };
    let is_empty = dest_dir
        .read_dir()
        .expect("directory must exist")
        .next()
        .is_none();
    if is_empty {
        println!("let's generate data.rs");
        let source_data = read_source_data(&source_data_dir);
        let code = generate_code(&source_data);
        println!("{}", code);
        flush_code(&dest_dir, code);
    } else {
        println!("force update data.rs");
        let source_data = read_source_data(&source_data_dir);
        let code = generate_code(&source_data);
        println!("{}", code);
        // flush_code(&dest_dir, code);
    }
}
