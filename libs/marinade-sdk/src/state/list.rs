use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{msg, program_error::ProgramError, pubkey::Pubkey};

use crate::error::CommonError;

#[derive(Default, Clone, BorshSerialize, BorshDeserialize, BorshSchema, Debug)]
pub struct List {
    pub account: Pubkey,
    pub item_size: u32,
    pub count: u32,
    // For chunked change account
    pub new_account: Pubkey,
    pub copied_count: u32,
}

impl List {
    pub fn bytes_for(item_size: u32, count: u32) -> u32 {
        8 + count * item_size
    }

    pub fn capacity_of(item_size: u32, account_len: usize) -> u32 {
        (account_len as u32 - 8) / item_size
    }

    pub fn item_size(&self) -> u32 {
        self.item_size
    }

    pub fn len(&self) -> u32 {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_changing_account(&self) -> bool {
        self.new_account != Pubkey::default()
    }

    pub fn capacity(&self, account_len: usize) -> Result<u32, ProgramError> {
        Ok(u32::try_from(
            account_len
                .checked_sub(8)
                .ok_or(ProgramError::AccountDataTooSmall)?,
        )
        .map_err(|_| ProgramError::from(CommonError::CalculationFailure))?
        .checked_div(self.item_size())
        .unwrap_or(std::u32::MAX)) // for zst element (why you are using it in list?)
    }

    pub fn get<I: BorshDeserialize>(
        &self,
        data: &[u8],
        index: u32,
        list_name: &str,
    ) -> Result<I, ProgramError> {
        if index >= self.len() {
            msg!(
                "list {} index out of bounds ({}/{})",
                list_name,
                index,
                self.len()
            );
            return Err(ProgramError::InvalidArgument);
        }
        let start = 8 + (index * self.item_size()) as usize;
        I::deserialize(&mut &data[start..(start + self.item_size() as usize)])
            .map_err(|err| ProgramError::BorshIoError(err.to_string()))
    }
}
