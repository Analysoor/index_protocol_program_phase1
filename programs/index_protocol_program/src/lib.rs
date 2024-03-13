use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod processor;
pub mod state;
pub mod utils;
pub use {constants::*, errors::*, processor::*, state::*, utils::*};

declare_id!("indxL6jiTVfJL48JFdRu7Bz4WKXBQX1otGgvnvpsaPE");

#[program]
pub mod index_protocol_program {
    use super::*;

    pub fn initialize_index_protocol_state(
        ctx: Context<InitializeeIndexProtocolState>,
        counter: u64
    ) -> Result<()> {
        handle_initialize_index_protocol_state(ctx, counter)
    }

    pub fn update_index_protocol_state(
        ctx: Context<UpdateIndexProtocolState>,
        new_super_authority: Option<Pubkey>,
        is_paused: Option<bool>,
        is_open: Option<bool>,
    ) -> Result<()> {
        handle_update_index_protocol_state(ctx, new_super_authority, is_paused, is_open)
    }

    pub fn deploy_mint_config(
        ctx: Context<Deploy>,
        config: Config,
        funds_recipient: Pubkey,
        spl_token_name: String,
        mint_type: MintType,
        mint_phase: MintPhase,
    ) -> Result<()> {
        handle_mint_deployment(
            ctx,
            config,
            funds_recipient,
            spl_token_name,
            mint_type,
            mint_phase,
        )
    }

    pub fn mint(ctx: Context<Mint>, _tick: String) -> Result<()> {
        handle_mint(ctx)
    }

    pub fn update_mint_config(
        ctx: Context<UpdateMintConfig>,
        new_authority: Option<Pubkey>,
        mint_phase: Option<MintPhase>,
        new_price: Option<u64>,
        new_start_date: Option<u64>,
    ) -> Result<()> {
        handle_update_mint_config(ctx, new_authority, mint_phase, new_price, new_start_date)
    }
    
    pub fn close_minter_states<'info>(
        ctx: Context<'_, '_, '_, 'info, CloseMinterStates<'info>>) -> Result<()> {
        handle_close_minter_states(ctx)
    }
}
