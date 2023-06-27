//use std::convert::TryInto;

use crate::{calc::proportional, checks::check_address, error::CommonError, state::list::List, ID};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ValidatorRecord {
    /// Validator vote pubkey
    pub validator_account: Pubkey,

    /// Validator total balance in lamports
    pub active_balance: u64, // must be 0 for removing
    pub score: u32,
    pub last_stake_delta_epoch: u64,
    pub duplication_flag_bump_seed: u8,
}

impl ValidatorRecord {
    pub const DISCRIMINATOR: &'static [u8; 8] = b"validatr";
    pub const DUPLICATE_FLAG_SEED: &'static [u8] = b"unique_validator";

    pub fn find_duplication_flag(state: &Pubkey, validator_account: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                &state.to_bytes()[..32],
                Self::DUPLICATE_FLAG_SEED,
                &validator_account.to_bytes()[..32],
            ],
            &ID,
        )
    }

    pub fn with_duplication_flag_seeds<R, F: FnOnce(&[&[u8]]) -> R>(
        &self,
        state: &Pubkey,
        f: F,
    ) -> R {
        f(&[
            &state.to_bytes()[..32],
            Self::DUPLICATE_FLAG_SEED,
            &self.validator_account.to_bytes()[..32],
            &[self.duplication_flag_bump_seed],
        ])
    }

    pub fn duplication_flag_address(&self, state: &Pubkey) -> Pubkey {
        self.with_duplication_flag_seeds(state, |seeds| Pubkey::create_program_address(seeds, &ID))
            .unwrap()
    }

    pub fn new(
        validator_account: Pubkey,
        score: u32,
        state: &Pubkey,
        duplication_flag_address: &Pubkey,
    ) -> Result<Self, ProgramError> {
        let (actual_duplication_flag, duplication_flag_bump_seed) =
            Self::find_duplication_flag(state, &validator_account);
        if duplication_flag_address != &actual_duplication_flag {
            msg!(
                "Duplication flag {} does not match {}",
                duplication_flag_address,
                actual_duplication_flag
            );
            return Err(ProgramError::InvalidArgument);
        }
        Ok(Self {
            validator_account,
            active_balance: 0,
            score,
            last_stake_delta_epoch: std::u64::MAX, // never
            duplication_flag_bump_seed,
        })
    }
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct ValidatorSystem {
    pub validator_list: List,
    pub manager_authority: Pubkey,
    pub total_validator_score: u32,
    /// sum of all active lamports staked
    pub total_active_balance: u64,
    /// allow & auto-add validator when a user deposits a stake-account of a non-listed validator
    pub auto_add_validator_enabled: u8,
}

impl ValidatorSystem {
    pub fn bytes_for_list(count: u32, additional_record_space: u32) -> u32 {
        List::bytes_for(
            ValidatorRecord::default().try_to_vec().unwrap().len() as u32 + additional_record_space,
            count,
        )
    }

    pub fn validator_list_address(&self) -> &Pubkey {
        &self.validator_list.account
    }

    pub fn validator_count(&self) -> u32 {
        self.validator_list.len()
    }

    pub fn validator_list_capacity(&self, validator_list_len: usize) -> Result<u32, ProgramError> {
        self.validator_list.capacity(validator_list_len)
    }

    pub fn validator_record_size(&self) -> u32 {
        self.validator_list.item_size()
    }

    pub fn get(
        &self,
        validator_list_data: &[u8],
        index: u32,
    ) -> Result<ValidatorRecord, ProgramError> {
        self.validator_list
            .get(validator_list_data, index, "validator_list")
    }

    pub fn validator_stake_target(
        &self,
        validator: &ValidatorRecord,
        total_stake_target: u64,
    ) -> Result<u64, CommonError> {
        if self.total_validator_score == 0 {
            return Ok(0);
        }
        proportional(
            total_stake_target,
            validator.score as u64,
            self.total_validator_score as u64,
        )
    }

    pub fn check_validator_list<'info>(
        &self,
        validator_list: &AccountInfo<'info>,
    ) -> ProgramResult {
        check_address(validator_list.key, self.validator_list_address())?;
        if &validator_list.data.borrow().as_ref()[0..8] != ValidatorRecord::DISCRIMINATOR {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    pub fn check_validator_manager_authority(&self, manager_authority: &Pubkey) -> ProgramResult {
        check_address(manager_authority, &self.manager_authority)
    }
}
