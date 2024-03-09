use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};

#[account]
pub struct IndexProtocolState {
    pub super_authority: Pubkey,
    pub is_paused: bool,
    /// anyone can deploy if below is true
    pub is_open: bool,
    pub counter: u64,
    pub reserved: [u8; 64],
}

impl IndexProtocolState {
    pub const INDEX_PROTOCOL_STATE_SIZE: usize = 8 + 32 + 1 + 1 + 8 + 64;
}
