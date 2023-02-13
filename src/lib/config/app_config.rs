use std::env;
use std::process::exit;

#[derive(Debug)]
pub enum ExitCode {
    ConsumerDeserializeError = 10,
    InsertNewRetailCustomerError = 11,
    InvalidDate = 12,
    DatabasseError = 13,
    CommitError = 14,
    NotFound = 15,
    ParseAmountError = 16,
    ParseDomainBalanceError = 17,
    InsertAccFromLedgerError = 18,
    InsertAccToLedgerError = 19,
    ProducedSecondRoundTransferError = 20,
    WritetimeNotEqual = 21,
    CannotRetry = 22,
    ManuallyExit = 23,
    CannotStoreOffsetFromMessage = 24,
    InternalServerError = 25,
    WritetimeNotEqualOrDuplicateTransactionIdButTransactionTimestampNotEqual = 26,
    ConsumeError = 27,
}

#[derive(Debug)]
pub struct AppConfig {
    pub argon2_secret: String,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn get_config() -> Self {
        match env::var("ENVIRONMENT") {
            Ok(envi) if { envi.as_str() == "PRODUCTION" } => return Self::get_production_env(),
            _ => return Self::get_dev_env(),
        };
    }

    fn get_dev_env() -> Self {
        let argon2_secret 
            = "9.b?4w#X6Um~(}S}nvcwa%k]m=-eB%/D8fL:j1|FX!2O8=?AXE=|2p4<ZLs9c9P*oiJtRVZQ^CsuX04-v2?o#7|jsIMv2+AQ00y+"
            .to_string();

            let jwt_secret 
            = ">tz]6X&q09A4T[1R#1+9zTbS1lc<dyMcU~c&],qyHwt)^9H3vQ*HV5kb[M/nP6/vH$}6rLp.L1$_jWbB$We^47831{]U|[5kG60Y"
            .to_string();

        Self { argon2_secret,jwt_secret }
    }

    fn get_production_env() -> Self {
        let argon2_secret = env::var("ARGON2_SECRET").expect("ARGON2_SECRET env is not set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET env is not set");

        Self { argon2_secret ,jwt_secret}
    }
}

pub fn exit_program(code: ExitCode) {
    exit(code as i32)
}

