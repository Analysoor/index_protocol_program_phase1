use crate::{constants::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateMintConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,
        seeds=[INDEX_PROTOCOL_STATE_PREFIX.as_bytes()],
        bump,)]
    pub index_protocol_state: Box<Account<'info, IndexProtocolState>>,

    #[account(mut,
        seeds=[MINT_DATA_CONFIG_PREFIX.as_bytes(), mint_data_config.config.tick.as_bytes()],
        bump,
        has_one=authority,)]
    pub mint_data_config: Box<Account<'info, MintDataConfig>>,

    /// The system program account
    pub system_program: Program<'info, System>,
}

pub fn handle_update_mint_config(
    ctx: Context<UpdateMintConfig>,
    new_authority: Option<Pubkey>,
    mint_phase: Option<MintPhase>,
    new_price: Option<u64>,
    new_start_date: Option<u64>,
) -> Result<()> {
    let mint_data_config = &mut ctx.accounts.mint_data_config;

    if let Some(new_authority) = new_authority {
        mint_data_config.authority = new_authority;
    }

    if let Some(mint_phase) = mint_phase {
        mint_data_config.mint_phase = mint_phase;
    }

    if let Some(new_start_date) = new_start_date {
        mint_data_config.config.start_date = new_start_date;
    }

    if let Some(new_price) = new_price {
        mint_data_config.config.price = new_price;
    }

    Ok(())
}
