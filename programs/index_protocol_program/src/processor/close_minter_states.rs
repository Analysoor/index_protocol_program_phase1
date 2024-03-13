use crate::{constants::*, state::*, utils::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct CloseMinterStates<'info> {
    #[account(mut)]
    pub super_authority: Signer<'info>,

    #[account(
        has_one = super_authority,
        seeds=[INDEX_PROTOCOL_STATE_PREFIX.as_bytes()],
        bump)]
    pub index_protocol_state: Box<Account<'info, IndexProtocolState>>,

    /// The system program account
    pub system_program: Program<'info, System>,
    // remaining accounts sits here..
}

/// super_authority only ixn, checked by anchor with the has_one
pub fn handle_close_minter_states<'info>(
        ctx: Context<'_, '_, '_, 'info, CloseMinterStates<'info>>) -> Result<()> {
    let super_authority = &ctx.accounts.super_authority;


    for remaining_account in ctx.remaining_accounts {
        let minter_state_account = remaining_account;
        close_account(minter_state_account, &super_authority.to_account_info())?;
    }

    Ok(())
}
