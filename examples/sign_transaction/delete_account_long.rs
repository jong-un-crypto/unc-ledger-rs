use unc_account_id::AccountId;
use unc_ledger::UNCLedgerError;

#[path = "../common/lib.rs"]
mod common;

#[allow(deprecated)]
fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> unc_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key);
    tx.actions = vec![unc_primitives::transaction::Action::DeleteAccount(
        unc_primitives::transaction::DeleteAccountAction {
            beneficiary_id: AccountId::new_unvalidated(
                "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c1b11b3b31673033936ad07bddc01f9da27d974811e480fb197c799e23480a489".to_string()),
        },
    )];
    tx
}

fn main() -> Result<(), UNCLedgerError> {
    common::get_key_sign_and_verify_flow(tx)
}
