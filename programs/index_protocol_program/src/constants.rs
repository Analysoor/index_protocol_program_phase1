use solana_program::{pubkey, pubkey::Pubkey};


pub const INDEX_PROTOCOL_STATE_PREFIX: &str = "index_protocol_state";
pub const MINT_DATA_CONFIG_PREFIX: &str = "mint_data_config";
pub const MINTER_STATE_PREFIX: &str = "minter_state";

pub const TOKEN_PROGRAM: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const MAX_TOKEN_NAME_LENGTH: usize = 10;