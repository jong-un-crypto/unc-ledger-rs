# unc-ledger-rs

[![Rust](https://github.com/khorolets/unc-ledger-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/khorolets/unc-ledger-rs/actions/workflows/rust.yml)
[![](http://meritbadge.herokuapp.com/unc-ledger)](https://crates.io/crates/unc-ledger)
[![]( https://docs.rs/unc-ledger/badge.svg)]( https://docs.rs/unc-ledger/)

It is UNC <-> Ledger transport


Provides a set of commands that can be executed to communicate with UNC App installed on Ledger device:

* Read PublicKey from Ledger device by HD Path
* Sign a Transaction


## Examples


### Get PublicKey from Ledger


```rust
use unc_ledger::get_public_key;
use slip10::BIP32Path;
use std::str::FromStr;

let hd_path = BIP32Path::from_str("44'/397'/0'/0'/1'").unwrap();
let public_key = get_public_key(hd_path).unwrap();
println!("{:#?}", public_key);
```


#### Trick


To convert the answer into `unc_crypto::PublicKey` do:

```rust
let public_key = unc_crypto::PublicKey::ED25519(
    unc_crypto::ED25519PublicKey::from(
        public_key.to_bytes(),
    )
);
```


### How to sign a transaction


```rust
use unc_ledger::{sign_transaction, SignTarget};
use unc_primitives::borsh::BorshSerialize;
use slip10::BIP32Path;
use std::str::FromStr;

let hd_path = BIP32Path::from_str("44'/397'/0'/0'/1'").unwrap();
let borsh_transaction = unc_unsigned_transaction.try_to_vec().unwrap();
let signature = sign_transaction(SignTarget::BorshUnsignedTx(borsh_transaction), hd_path).unwrap();
println!("{:#?}", signature);
```


#### Trick

To convert the answer into `unc_crypto::Signature` do:


```rust
let signature = unc_crypto::Signature::from_parts(unc_crypto::KeyType::ED25519, &signature)
    .expect("Signature is not expected to fail on deserialization");
```

## Executable examples

### Get version

```bash
RUST_LOG=info cargo run --example get_version
```

### Get PublicKey from Ledger

#### Display

```bash
RUST_LOG=info cargo run --example get_public_key_display
```
#### Silent

```bash
RUST_LOG=info cargo run --example get_public_key_silent
```

### Get WalletID from Ledger

```bash
RUST_LOG=info cargo run --example get_wallet_id
```
### Sign a transaction

#### Transfer

```bash
RUST_LOG=info cargo run --example sign_transfer
```

#### Other

```bash
export RUST_LOG=info
cargo run --example sign_create_account
cargo run --example sign_delete_account_short
cargo run --example sign_delete_account_long
cargo run --example sign_delete_key_ed25519
cargo run --example sign_delete_key_secp256k1
cargo run --example sign_stake
cargo run --example sign_add_key_fullaccess
cargo run --example sign_add_key_functioncall
cargo run --example sign_deploy_contract
cargo run --example sign_functioncall_str
cargo run --example sign_functioncall_bin
cargo run --example sign_functioncall_str_parse_err
cargo run --example sign_batch_all_actions
```

### Sign a NEP-413 message

```bash
RUST_LOG=info cargo run --example sign_nep_413_message
```

### Sign a NEP-366 delegate action

```bash
RUST_LOG=info cargo run --example sign_nep_366_delegate_action
```
