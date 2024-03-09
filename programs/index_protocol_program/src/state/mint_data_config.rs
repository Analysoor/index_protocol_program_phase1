use crate::{constants::MAX_TOKEN_NAME_LENGTH, errors::IndexProtocolProgramError};
use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};

#[account]
pub struct MintDataConfig {
    pub authority: Pubkey,
    // mint distributor.(not used in the current version)
    pub distributor: Pubkey,
    // where tokens are recieved when mint occurs.
    pub funds_recipient: Pubkey,
    // max length is 10
    pub spl_token_name: String,
    // SPL token or NFT
    pub mint_type: MintType,
    // several mint phases used for the future and the current protocol version.
    pub mint_phase: MintPhase,
    pub bump: u8,
    pub config: Config,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Eq, PartialEq, Debug)]
pub enum MintType {
    SplToken,
    Nft,
}

// only is paused is used in the current version of the program.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Eq, PartialEq, Debug)]
pub enum MintPhase {
    // phase not set
    Uninitialized,
    // minting is enabled
    Minting,
    // minting is paused
    Paused,
    // minting is done and distribution phase is on.
    Distributing,
    // distribution is done and claiming phase is on.
    Claiming,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Config {
    // price in lamports (SOL)
    pub price: u64,
    pub amount_per_mint: u64,
    // date in unix timestamp
    pub start_date: u64,
    pub counter: u64,
    pub supply: u64,
    // lowercased and 4 characters in length. must be unique.
    pub tick: String,
    pub liquidity_bootstrapping: bool,
    pub filter: Vec<String>,
}

impl MintDataConfig {
    pub const MINT_DATA_CONFIG_SIZE: usize =
        8 + 32 + 32 + 32 + 4 + 10 + 1 + 1 + 1 + 8 + 8 + 8 + 8 + 8 + 4 + 4 + 1 + 4 + 36 + 256;

    // make sure that length does not exist a certain value
    pub fn validate_spl_token_name(token_name: &String) -> Result<()> {
        if token_name.as_bytes().len() > MAX_TOKEN_NAME_LENGTH {
            msg!("{} max is {}", token_name.len(), MAX_TOKEN_NAME_LENGTH);
            return Err(IndexProtocolProgramError::TokenNameTooLong.into());
        }
        Ok(())
    }

    // make sure tick is lowercased
    pub fn validate_tick(config: &Config) -> Result<()> {
        // Check if tick is not lowercased
        if config.tick != config.tick.to_lowercase() {
            return Err(IndexProtocolProgramError::TickNotLowercased.into());
        }

        // Check if tick has a length of 4 characters
        if config.tick.len() != 4 {
            return Err(IndexProtocolProgramError::TickInvalid.into());
        }

        Ok(())
    }

    pub fn validate_start_date(start_date: &u64) -> Result<()> {
        // Get the current timestamp from the clock sysvar
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        // Check if the start_date is higher than the current timestamp
        if *start_date <= current_timestamp as u64 {
            return Err(IndexProtocolProgramError::StartDateInvalid.into());
        }

        Ok(())
    }

    pub fn validate_supply(supply: &u64) -> Result<()> {
        // Check if the supply is 0
        if *supply == 0 {
            return Err(IndexProtocolProgramError::SupplyTooLow.into());
        }

        Ok(())
    }

    // counter must be +1 from the index_protocol_state.counter
    pub fn validate_counter(config: &Config, current_counter: u64) -> Result<()> {
        if config.counter != current_counter + 1 {
            return Err(IndexProtocolProgramError::SupplyTooLow.into());
        }

        Ok(())
    }
}
