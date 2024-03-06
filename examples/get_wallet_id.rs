use std::str::FromStr;

use unc_ledger::{get_wallet_id, UNCLedgerError};
use slip10::BIP32Path;

#[path = "./common/lib.rs"]
mod common;

fn main() -> Result<(), UNCLedgerError> {
    env_logger::builder().init();
    let hd_path = BIP32Path::from_str("44'/397'/0'/0'/1'").unwrap();

    let public_key = get_wallet_id(hd_path)?;

    common::display_pub_key(public_key);

    Ok(())
}
