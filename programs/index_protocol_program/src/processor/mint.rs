use crate::{constants::*, errors::*, state::*, utils::*};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};
use anchor_spl::{
    // associated_token::AssociatedToken,
    token::{
        spl_token::instruction::{burn, close_account},
        Mint as SplMint, Token,
    },
};

#[derive(Accounts)]
#[instruction(tick: String)]
pub struct Mint<'info> {
    #[account(mut)]
    pub fee_payer: Signer<'info>,

    #[account(
        seeds=[INDEX_PROTOCOL_STATE_PREFIX.as_bytes()],
        bump)]
    pub index_protocol_state: Box<Account<'info, IndexProtocolState>>,

    #[account(mut,
        has_one = funds_recipient,
        seeds=[MINT_DATA_CONFIG_PREFIX.as_bytes(), tick.as_bytes()],
        bump
    )]
    pub mint_data_config: Account<'info, MintDataConfig>,
    #[account(init_if_needed,
        payer=fee_payer,
        space=MinterState::get_minter_state_space(minter_state.data_len()),
        seeds=[MINTER_STATE_PREFIX.as_bytes(), tick.as_bytes(), fee_payer.key().as_ref()],
        bump
    )]
    pub minter_state: Account<'info, MinterState>,

    /// any mint that would be burnt later on.
    #[account(mut)]
    pub burnable_mint: Box<Account<'info, SplMint>>,

    /// CHECK: it is checked below.
    #[account(mut)]
    pub burnable_mint_ata: UncheckedAccount<'info>,

    // needed if via token
    // pub ata_program: Program<'info, AssociatedToken>,
    /// The SPL token program account
    pub spl_token_program: Program<'info, Token>,

    /// CHECK: checked above in mint_data_config
    #[account(mut)]
    pub funds_recipient: AccountInfo<'info>,

    /// The system program account
    pub system_program: Program<'info, System>,
}

pub fn handle_mint(ctx: Context<Mint>) -> Result<()> {
    let index_protocol_state = &ctx.accounts.index_protocol_state;
    let mint_data_config = &mut ctx.accounts.mint_data_config;
    let fee_payer = &ctx.accounts.fee_payer;
    let funds_recipient = &ctx.accounts.funds_recipient;
    let minter_state = &mut ctx.accounts.minter_state;
    let burnable_mint = &ctx.accounts.burnable_mint;
    let burnable_mint_ata = &ctx.accounts.burnable_mint_ata;
    // let ata_program = &ctx.accounts.ata_program;
    let spl_token_program = &ctx.accounts.spl_token_program;
    let system_program = &ctx.accounts.system_program;

    let is_super_authority = cmp_pubkeys(&fee_payer.key(), &index_protocol_state.super_authority);
    if !is_super_authority {
        // index protocol is paused check for minting
        if index_protocol_state.is_paused {
            msg!("Index Protocol mint is currently paused.");
            return err!(IndexProtocolProgramError::IndexProtocolIsPaused);
        }

        // check is minting phase is minting.
        if mint_data_config.mint_phase != MintPhase::Minting {
            msg!("Mint Phase is not Minting.");
            return err!(IndexProtocolProgramError::IndexProtocolIsPaused);
        }

        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        // Check if the start_date is greater than the current timestamp
        if mint_data_config.config.start_date > current_timestamp as u64 {
            msg!("Mint has not started.");
            return err!(IndexProtocolProgramError::MintHasNotStarted);
        }
    }

    // Check if the start_date is greater than the current timestamp
    if mint_data_config.config.counter == mint_data_config.config.supply {
        msg!("Max supply has reached.");
        return err!(IndexProtocolProgramError::MaxSupplyHasReached);
    }

    assert_is_ata(burnable_mint_ata, &fee_payer.key(), &burnable_mint.key())?;
    // burn nft
    let burn_nft_ix = burn(
        &spl_token_program.key(),
        &burnable_mint_ata.key(),
        &burnable_mint.key(),
        fee_payer.key,
        &[],
        1,
    )?;

    invoke(
        &burn_nft_ix,
        &[
            spl_token_program.to_account_info(),
            burnable_mint_ata.to_account_info(),
            burnable_mint.to_account_info(),
            fee_payer.to_account_info(),
        ],
    )?;

    let close_ata_ix = close_account(
        &spl_token_program.key(),
        &burnable_mint_ata.key(),
        fee_payer.key,
        fee_payer.key,
        &[],
    )?;

    invoke(
        &close_ata_ix,
        &[
            spl_token_program.to_account_info(),
            burnable_mint_ata.to_account_info(),
            fee_payer.to_account_info(),
        ],
    )?;
    // funds transfer (below will fail if the amount is insufficient)
    invoke(
        &system_instruction::transfer(
            fee_payer.key,
            funds_recipient.key,
            // price must be in lamports
            mint_data_config.config.price,
        ),
        &[
            fee_payer.to_account_info(),
            funds_recipient.to_account_info(),
            system_program.to_account_info(),
        ],
    )?;

    if minter_state.burnt_mints.len() == 0 {
        let mints_vec = vec![burnable_mint.key()];
        minter_state.burnt_mints = mints_vec;
    } else {
        // realloc and add mint.
        let required_lamports = Rent::get()?.minimum_balance(32);
        
        invoke(
            &system_instruction::transfer(
                fee_payer.key,
                &minter_state.key(),
                required_lamports,
            ),
            &[
                fee_payer.to_account_info(),
                minter_state.to_account_info(),
                system_program.to_account_info(),
            ],
        )?;

        let minter_state_account_info = minter_state.to_account_info();
        minter_state_account_info.realloc(minter_state_account_info.data_len() + 32, false)?;

        // add new mint
        minter_state.burnt_mints.push(burnable_mint.key())
    }

    Ok(())
}
