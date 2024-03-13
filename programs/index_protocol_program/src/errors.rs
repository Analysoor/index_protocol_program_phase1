use anchor_lang::prelude::*;

#[error_code]
pub enum IndexProtocolProgramError {
    // 6000
    #[msg("PublicKeyMismatch")]
    PublicKeyMismatch,
    // 6001
    #[msg("Account is not initialized!")]
    Uninitialized,
    // 6002
    #[msg("Invalid hash name seed length")]
    InvalidSeed,
    // 6003
    #[msg("Token Name too long")]
    TokenNameTooLong,
    // 6004
    #[msg("Protocol is paused")]
    IndexProtocolIsPaused,
    // 6005
    #[msg("Start Date cannot be in the past.")]
    StartDateInvalid,
    // 6006
    #[msg("Max supply cannot be 0.")]
    SupplyTooLow,
    // 6007
    #[msg("Tick is not lowercased.")]
    TickNotLowercased,
    // 6008
    #[msg("Tick is not 4 character in length")]
    TickInvalid,
    // 6009
    #[msg("Start date did not reach yet.")]
    MintHasNotStarted,
    // 6010
    #[msg("Cannot mint more max supply has reached.")]
    MaxSupplyHasReached,
    // 6011
    #[msg("Wrong account owner.")]
    WrongAccountOwner,
    // 6012
    #[msg("Minter limit reached.")]
    MinterLimitReached,
}
