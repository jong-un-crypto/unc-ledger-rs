use unc_crypto::SecretKey;
use unc_ledger::UNCLedgerError;

#[path = "../common/lib.rs"]
mod common;

fn tx(ledger_pub_key: ed25519_dalek::PublicKey) -> unc_primitives::transaction::Transaction {
    let mut tx = common::tx_template(ledger_pub_key.clone());
    let sk = SecretKey::from_seed(
        unc_crypto::KeyType::SECP256K1,
        &format!("{:?}", ledger_pub_key),
    );
    let public_key = sk.public_key();
    tx.actions = vec![unc_primitives::transaction::Action::AddKey(Box::new(
        unc_primitives::transaction::AddKeyAction {
            public_key,
            access_key: unc_primitives_core::account::AccessKey {
                nonce: 127127127127,
                permission: unc_primitives_core::account::AccessKeyPermission::FullAccess,
            },
        },
    ))];
    tx
}

fn main() -> Result<(), UNCLedgerError> {
    common::get_key_sign_and_verify_flow(tx)
}
