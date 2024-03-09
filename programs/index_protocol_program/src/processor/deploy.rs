use crate::{
    constants::*,
    errors::*,
    state::*,
    utils::{assert_keys_equal, cmp_pubkeys},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(config: Config)]
pub struct Deploy<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds=[INDEX_PROTOCOL_STATE_PREFIX.as_bytes()],
        bump)]
    pub index_protocol_state: Box<Account<'info, IndexProtocolState>>,

    #[account(init,
        space=MintDataConfig::MINT_DATA_CONFIG_SIZE,
        seeds=[MINT_DATA_CONFIG_PREFIX.as_bytes(), config.tick.as_bytes()],
        bump,
        payer=authority,)]
    pub mint_data_config: Account<'info, MintDataConfig>,

    /// The system program account
    pub system_program: Program<'info, System>,
}

pub fn handle_mint_deployment(
    ctx: Context<Deploy>,
    config: Config,
    funds_recipient: Pubkey,
    spl_token_name: String,
    mint_type: MintType,
    mint_phase: MintPhase,
) -> Result<()> {
    let index_protocol_state = &mut ctx.accounts.index_protocol_state;
    let mint_data_config = &mut ctx.accounts.mint_data_config;
    let authority = &ctx.accounts.authority;

    // check if protocol state is open
    if !index_protocol_state.is_open {
        // only super authority can deploy a mint
        assert_keys_equal(authority.key(), index_protocol_state.super_authority)?;
    }

    let is_super_authority = cmp_pubkeys(&authority.key(), &index_protocol_state.super_authority);
    if !is_super_authority {
        // index protocol is paused check for deployments
        if index_protocol_state.is_paused {
            msg!("Index Protocol mint deployment is currently paused.");
            return err!(IndexProtocolProgramError::IndexProtocolIsPaused);
        }
    }

    // name must not exceed 10 characters
    MintDataConfig::validate_spl_token_name(&spl_token_name)?;
    // start_data must be in the future
    MintDataConfig::validate_start_date(&config.start_date)?;
    // supply cannot be 0
    MintDataConfig::validate_supply(&config.supply)?;
    // tick validation
    MintDataConfig::validate_tick(&config)?;
    // counter validation
    MintDataConfig::validate_counter(&config, index_protocol_state.counter)?;

    // we increment after we validate
    index_protocol_state.counter +=1; 

    let mint_data_config_bump = ctx.bumps.mint_data_config;

    let mint_data_config_data = MintDataConfig {
        authority: authority.key(),
        // not used yet.
        distributor: Pubkey::default(),
        funds_recipient,
        spl_token_name,
        mint_type,
        mint_phase,
        bump: mint_data_config_bump,
        config,
    };

    mint_data_config.set_inner(mint_data_config_data);

    Ok(())
}
