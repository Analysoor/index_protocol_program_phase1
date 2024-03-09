use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};

#[account]
pub struct MinterState {
    pub burnt_mints: Vec<Pubkey>,
}

impl MinterState {
    pub const MIN_MINTER_STATE_SIZE: usize = 8 + 4 + 32;


    pub fn get_minter_state_space(current_size: usize) -> usize {
        if current_size == 0 {
            Self::MIN_MINTER_STATE_SIZE
        } else {
            current_size
        }
    }

}
