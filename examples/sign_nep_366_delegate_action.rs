use std::{convert::TryInto, str::FromStr};

use unc_account_id::AccountId;
use unc_crypto::Signature;
use unc_ledger::UNCLedgerError;
use unc_primitives::action::delegate::{DelegateAction, SignedDelegateAction};
use slip10::BIP32Path;

use crate::common::display_pub_key;

#[path = "./common/lib.rs"]
mod common;

fn main() -> Result<(), UNCLedgerError> {
    env_logger::builder().init();

    let hd_path = BIP32Path::from_str("44'/397'/0'/0'/1'").unwrap();
    let ledger_pub_key = unc_ledger::get_public_key_with_display_flag(hd_path.clone(), false)?;
    display_pub_key(ledger_pub_key);

    let sender_id = AccountId::from_str("bob.unc").unwrap();

    let actions = common::batch_of_all_types_of_actions(ledger_pub_key)
        .into_iter()
        .map(|action| action.try_into().unwrap())
        .collect::<Vec<_>>();

    let ledger_pub_key = unc_crypto::PublicKey::ED25519(unc_crypto::ED25519PublicKey::from(
        ledger_pub_key.to_bytes(),
    ));

    let delegate_action = DelegateAction {
        sender_id,
        receiver_id: AccountId::from_str("alice.unc").unwrap(),
        actions,
        nonce: 127127122121,
        max_block_height: 100500,
        public_key: ledger_pub_key,
    };

    let signature_bytes =
        unc_ledger::sign_message_nep366_delegate_action(&delegate_action, hd_path)?;

    let signature = Signature::from_parts(unc_crypto::KeyType::ED25519, &signature_bytes).unwrap();

    let signed_delegate = SignedDelegateAction {
        delegate_action,
        signature,
    };
    log::info!("{:#?}", signed_delegate);
    assert!(signed_delegate.verify());

    common::display_signature(signature_bytes);
    Ok(())
}
