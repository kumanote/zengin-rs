# zengin-rs

> The Rust implementation of "Zengin Code".
> 
> "Zengin Code" defines Japanese bank codes and branch codes.
> 
> e.g.  "0001" for "みずほ銀行" / "001" for "東京営業部支店"


## Installation

#### Dependencies

- [Rust with Cargo](http://rust-lang.org)

**rust-toolchain**

```text
1.60.0
```

#### Importing

**Cargo.toml**

```toml
[dependencies]
zengin = { version = "1.1.2", git = "https://github.com/kumanote/zengin-rs", branch = "main" }
```

**rust files**

```rust
use zengin;
```

## Setup

This library requires [json file data](https://github.com/zengin-code/source-data/tree/2022-04-25/data).   
**Please download data files in advance.**

```bash
% git clone git@github.com:zengin-code/source-data.git
```


## Usage

```rust
use zengin::{Bank, Branch};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source_data_dir = {
        let mut result = std::env::current_dir().unwrap();
        result.push("zengin-rs/source-data/data");
        result
    };
    zengin::load_all_data(Some(source_data_dir)).expect("data must be loaded");
    let bank_code = "0001";
    let branch_code = "988";
    let bank = zengin::get_bank(bank_code).await.unwrap().unwrap();
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
    let branches = zengin::get_bank_branches(bank_code).await.unwrap().unwrap();
    assert_eq!(branches.len(), 494);
    let branch = zengin::get_branch(bank_code, branch_code)
        .await
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
    Ok(())
}
```
