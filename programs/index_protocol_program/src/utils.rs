use crate::{errors::*, id, TOKEN_PROGRAM};
use anchor_lang::{
    prelude::*,
    solana_program::{
        program::invoke_signed, program_memory::sol_memcmp, pubkey::PUBKEY_BYTES,
        program_pack::{IsInitialized, Pack},
        system_instruction::create_account, sysvar::instructions::get_instruction_relative,
    },
};
use anchor_spl::{token::spl_token::state::Account, associated_token::get_associated_token_address};

pub fn puffed_out_string(s: &str, size: usize) -> String {
    let mut array_of_zeroes = vec![];
    let puff_amount = size - s.len();
    while array_of_zeroes.len() < puff_amount {
        array_of_zeroes.push(0u8);
    }
    s.to_owned() + std::str::from_utf8(&array_of_zeroes).unwrap()
}

pub fn assert_keys_equal(key1: Pubkey, key2: Pubkey) -> Result<()> {
    if sol_memcmp(key1.as_ref(), key2.as_ref(), PUBKEY_BYTES) != 0 {
        msg!("Wrong public key: {} should be {}", key1, key2);
        return err!(IndexProtocolProgramError::PublicKeyMismatch);
    } else {
        Ok(())
    }
}

// cheapest pubkey comparing
pub fn cmp_pubkeys(a: &Pubkey, b: &Pubkey) -> bool {
    sol_memcmp(a.as_ref(), b.as_ref(), PUBKEY_BYTES) == 0
}

// no cpi from other programs.
pub fn assert_no_cpi(account: &AccountInfo) -> Result<()> {
    let instruction_sysvar_account_info = account.to_account_info();
    // let instruction_sysvar = instruction_sysvar_account_info.data.borrow();
    let current_ix = get_instruction_relative(0, &instruction_sysvar_account_info).unwrap();
    assert_keys_equal(current_ix.program_id, crate::id())?;
    Ok(())
}

pub fn create_one_byte_state_account<'a>(
    system_program: &AccountInfo<'a>,
    fee_payer: &AccountInfo<'a>,
    account_to_create: &AccountInfo<'a>,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    let space = 1;
    let create_state_instruction = create_account(
        fee_payer.key,
        account_to_create.key,
        Rent::get()?.minimum_balance(space),
        space as u64,
        &id(),
    );

    invoke_signed(
        &create_state_instruction,
        &[
            system_program.clone(),
            fee_payer.clone(),
            account_to_create.clone(),
        ],
        &[signer_seeds],
    )?;
    Ok(())
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> Result<()> {
    if account.owner != owner {
        msg!("Wrong account owner: {} should be {}", account.owner, owner);
        return Err(IndexProtocolProgramError::WrongAccountOwner.into());
    }
    Ok(())
}


pub fn assert_initialized<T: Pack + IsInitialized>(account_info: &AccountInfo) -> Result<T> {
    let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    if !account.is_initialized() {
        return Err(IndexProtocolProgramError::Uninitialized.into());
    } else {
        Ok(account)
    }
}

pub fn assert_is_ata(ata: &AccountInfo, wallet: &Pubkey, mint: &Pubkey) -> Result<Account> {
    assert_owned_by(ata, &TOKEN_PROGRAM)?;
    let ata_account: Account = assert_initialized(ata)?;
    assert_keys_equal(ata_account.owner, *wallet)?;
    assert_keys_equal(ata_account.mint, *mint)?;
    assert_keys_equal(get_associated_token_address(wallet, mint), *ata.key)?;
    Ok(ata_account)
}
