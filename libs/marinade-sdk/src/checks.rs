use solana_program::stake::state::StakeState;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::state::Account as TokenAccount;
use spl_token::state::Mint;

use crate::error::CommonError;

pub fn check_min_amount(amount: u64, min_amount: u64) -> ProgramResult {
    if amount >= min_amount {
        Ok(())
    } else {
        Err(CommonError::NumberTooLow.into())
    }
}

pub fn check_address(actual_address: &Pubkey, reference_address: &Pubkey) -> ProgramResult {
    if actual_address == reference_address {
        Ok(())
    } else {
        Err(ProgramError::InvalidArgument)
    }
}

pub fn check_owner_program<'info>(account: &AccountInfo<'info>, owner: &Pubkey) -> ProgramResult {
    let actual_owner = account.owner;
    if actual_owner == owner {
        Ok(())
    } else {
        Err(ProgramError::InvalidArgument)
    }
}

pub fn check_mint_authority(mint: &Mint, mint_authority: Pubkey) -> ProgramResult {
    if mint.mint_authority.contains(&mint_authority) {
        Ok(())
    } else {
        Err(ProgramError::InvalidAccountData)
    }
}

pub fn check_freeze_authority(mint: &Mint) -> ProgramResult {
    if mint.freeze_authority.is_none() {
        Ok(())
    } else {
        Err(ProgramError::InvalidAccountData)
    }
}

pub fn check_mint_empty(mint: &Mint) -> ProgramResult {
    if mint.supply == 0 {
        Ok(())
    } else {
        Err(ProgramError::InvalidArgument)
    }
}

pub fn check_token_mint(token: &TokenAccount, mint: Pubkey) -> ProgramResult {
    if token.mint == mint {
        Ok(())
    } else {
        Err(ProgramError::InvalidAccountData)
    }
}

pub fn check_token_owner(token: &TokenAccount, owner: &Pubkey) -> ProgramResult {
    if token.owner == *owner {
        Ok(())
    } else {
        Err(ProgramError::InvalidAccountData)
    }
}

// check that the account is delegated and to the right validator
// also that the stake amount is updated
pub fn check_stake_amount_and_validator(
    stake_state: &StakeState,
    expected_stake_amount: u64,
    validator_vote_pubkey: &Pubkey,
) -> ProgramResult {
    let currently_staked = if let Some(delegation) = stake_state.delegation() {
        if delegation.voter_pubkey != *validator_vote_pubkey {
            return Err(ProgramError::InvalidInstructionData);
        }
        delegation.stake
    } else {
        return Err(CommonError::StakeNotDelegated.into());
    };
    // do not allow to operate on an account where last_update_delegated_lamports != currently_staked
    if currently_staked != expected_stake_amount {
        return Err(CommonError::StakeAccountNotUpdatedYet.into());
    }
    Ok(())
}
