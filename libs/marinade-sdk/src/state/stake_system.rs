use crate::{
    checks::check_address,
    located::Located,
    state::{list::List, marinade::Marinade},
    ID,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct StakeRecord {
    pub stake_account: Pubkey,
    pub last_update_delegated_lamports: u64,
    pub last_update_epoch: u64,
    pub is_emergency_unstaking: u8, // 1 for cooling down after emergency unstake, 0 otherwise
}

impl StakeRecord {
    pub const DISCRIMINATOR: &'static [u8; 8] = b"staker__";
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct StakeSystem {
    pub stake_list: List,
    //pub last_update_epoch: u64,
    //pub updated_during_last_epoch: u32,
    pub delayed_unstake_cooling_down: u64,
    pub stake_deposit_bump_seed: u8,
    pub stake_withdraw_bump_seed: u8,

    /// set by admin, how much slots before the end of the epoch, stake-delta can start
    pub slots_for_stake_delta: u64,
    /// Marks the start of stake-delta operations, meaning that if somebody starts a delayed-unstake ticket
    /// after this var is set with epoch_num the ticket will have epoch_created = current_epoch+1
    /// (the user must wait one more epoch, because their unstake-delta will be execute in this epoch)
    pub last_stake_delta_epoch: u64,
    pub min_stake: u64, // Minimal stake account delegation
    /// can be set by validator-manager-auth to allow a second run of stake-delta to stake late stakers in the last minute of the epoch
    /// so we maximize user's rewards
    pub extra_stake_delta_runs: u32,
}

impl StakeSystem {
    pub const STAKE_WITHDRAW_SEED: &'static [u8] = b"withdraw";
    pub const STAKE_DEPOSIT_SEED: &'static [u8] = b"deposit";

    pub fn bytes_for_list(count: u32, additional_record_space: u32) -> u32 {
        List::bytes_for(
            StakeRecord::default().try_to_vec().unwrap().len() as u32 + additional_record_space,
            count,
        )
    }

    pub fn find_stake_withdraw_authority(state: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[&state.to_bytes()[..32], Self::STAKE_WITHDRAW_SEED], &ID)
    }

    pub fn find_stake_deposit_authority(state: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[&state.to_bytes()[..32], Self::STAKE_DEPOSIT_SEED], &ID)
    }

    pub fn stake_list_address(&self) -> &Pubkey {
        &self.stake_list.account
    }

    pub fn stake_count(&self) -> u32 {
        self.stake_list.len()
    }

    pub fn stake_list_capacity(&self, stake_list_len: usize) -> Result<u32, ProgramError> {
        self.stake_list.capacity(stake_list_len)
    }

    pub fn stake_record_size(&self) -> u32 {
        self.stake_list.item_size()
    }

    pub fn get(&self, stake_list_data: &[u8], index: u32) -> Result<StakeRecord, ProgramError> {
        self.stake_list.get(stake_list_data, index, "stake_list")
    }

    pub fn check_stake_list<'info>(&self, stake_list: &AccountInfo<'info>) -> ProgramResult {
        check_address(stake_list.key, self.stake_list_address())?;
        if &stake_list.data.borrow().as_ref()[0..8] != StakeRecord::DISCRIMINATOR {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}

pub trait StakeSystemHelpers {
    fn stake_withdraw_authority(&self) -> Pubkey;
    fn with_stake_withdraw_authority_seeds<R, F: FnOnce(&[&[u8]]) -> R>(&self, f: F) -> R;
    fn check_stake_withdraw_authority(&self, stake_withdraw_authority: &Pubkey) -> ProgramResult;

    fn stake_deposit_authority(&self) -> Pubkey;
    fn with_stake_deposit_authority_seeds<R, F: FnOnce(&[&[u8]]) -> R>(&self, f: F) -> R;
    fn check_stake_deposit_authority(&self, stake_deposit_authority: &Pubkey) -> ProgramResult;
}

impl<T> StakeSystemHelpers for T
where
    T: Located<Marinade>,
{
    fn stake_withdraw_authority(&self) -> Pubkey {
        self.with_stake_withdraw_authority_seeds(|seeds| {
            Pubkey::create_program_address(seeds, &ID).unwrap()
        })
    }

    fn with_stake_withdraw_authority_seeds<R, F: FnOnce(&[&[u8]]) -> R>(&self, f: F) -> R {
        f(&[
            &self.key().to_bytes()[..32],
            StakeSystem::STAKE_WITHDRAW_SEED,
            &[self.as_ref().stake_system.stake_withdraw_bump_seed],
        ])
    }

    fn check_stake_withdraw_authority(&self, stake_withdraw_authority: &Pubkey) -> ProgramResult {
        check_address(stake_withdraw_authority, &self.stake_withdraw_authority())
    }

    fn stake_deposit_authority(&self) -> Pubkey {
        self.with_stake_deposit_authority_seeds(|seeds| {
            Pubkey::create_program_address(seeds, &ID).unwrap()
        })
    }

    fn with_stake_deposit_authority_seeds<R, F: FnOnce(&[&[u8]]) -> R>(&self, f: F) -> R {
        f(&[
            &self.key().to_bytes()[..32],
            StakeSystem::STAKE_DEPOSIT_SEED,
            &[self.as_ref().stake_system.stake_deposit_bump_seed],
        ])
    }

    fn check_stake_deposit_authority(&self, stake_deposit_authority: &Pubkey) -> ProgramResult {
        check_address(stake_deposit_authority, &self.stake_deposit_authority())
    }
}
