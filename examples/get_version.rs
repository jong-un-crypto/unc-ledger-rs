use env_logger;
use unc_ledger::{get_version, UNCLedgerError};

fn main() -> Result<(), UNCLedgerError> {
    env_logger::builder().init();

    let version = get_version()?;

    log::info!("{:#?}", version);
    Ok(())
}
