use crate::{constants::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct InitializeeIndexProtocolState<'info> {
    #[account(mut)]
    pub super_authority: Signer<'info>,

    #[account(init,
        space=IndexProtocolState::INDEX_PROTOCOL_STATE_SIZE,
        seeds=[INDEX_PROTOCOL_STATE_PREFIX.as_bytes()],
        bump,
        payer=super_authority,)]
    pub index_protocol_state: Account<'info, IndexProtocolState>,
    /// The system program account
    pub system_program: Program<'info, System>,
}

// this ixn will fail if it has already been created.
// no need to add more checks.
pub fn handle_initialize_index_protocol_state(
    ctx: Context<InitializeeIndexProtocolState>,
    counter: u64
) -> Result<()> {
    let index_protocol_state = &mut ctx.accounts.index_protocol_state;
    let super_authority = &ctx.accounts.super_authority;

    let index_protocol_state_data = IndexProtocolState {
        super_authority: super_authority.key(),
        // only super_authority can deploy
        is_open: false,
        // only super_authority can mint
        is_paused: true,
        counter,
        reserved: [0_u8; 64],
    };

    index_protocol_state.set_inner(index_protocol_state_data);

    Ok(())
}
