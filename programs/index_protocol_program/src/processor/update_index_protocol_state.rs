use crate::{constants::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct UpdateIndexProtocolState<'info> {
    #[account(mut)]
    pub super_authority: Signer<'info>,

    #[account(mut,
        seeds=[INDEX_PROTOCOL_STATE_PREFIX.as_bytes()],
        bump,
        has_one=super_authority)]
    pub index_protocol_state: Box<Account<'info, IndexProtocolState>>,
    /// The system program account
    pub system_program: Program<'info, System>,
}

pub fn handle_update_index_protocol_state(
    ctx: Context<UpdateIndexProtocolState>,
    new_super_authority: Option<Pubkey>,
    is_paused: Option<bool>,
    is_open: Option<bool>,
) -> Result<()> {
    let index_protocol_state = &mut ctx.accounts.index_protocol_state;

    if let Some(new_super_authority) = new_super_authority {
        index_protocol_state.super_authority = new_super_authority;
    }
    if let Some(is_paused) = is_paused {
        index_protocol_state.is_paused = is_paused;
    }
    if let Some(is_open) = is_open {
        index_protocol_state.is_open = is_open;
    }

    Ok(())
}
