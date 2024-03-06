use std::str::FromStr;

use unc_account_id::AccountId;
use unc_ledger::UNCLedgerError;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> unc_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key);
    tx.actions = vec![unc_primitives::transaction::Action::DeleteAccount(
        unc_primitives::transaction::DeleteAccountAction {
            beneficiary_id: AccountId::from_str("bob.unc").unwrap(),
        },
    )];
    tx
}

fn main() -> Result<(), UNCLedgerError> {
    common::get_key_sign_and_verify_flow(tx)
}
