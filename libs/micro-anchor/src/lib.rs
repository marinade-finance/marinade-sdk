use borsh::{BorshDeserialize, BorshSerialize};
use derive_more::{Display, Error};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};

/// 8 byte unique identifier for a type.
pub trait Discriminator {
    const DISCRIMINATOR: [u8; 8];
}

/// Defines an address expected to own an account.
pub trait Owner {
    fn owner() -> Pubkey;
}

#[derive(Debug, Display, Error)]
pub enum AccountDeserializeError {
    DiscriminatorNotFound,
    DiscriminatorMismatch,
    DidNotDeserialize,
}

/// A data structure that can be deserialized and stored into account storage,
/// i.e. an
/// [`AccountInfo`](../solana_program/account_info/struct.AccountInfo.html#structfield.data)'s
/// mutable data slice.
pub trait AccountDeserialize: Sized + BorshDeserialize + Discriminator + Owner {
    /// Deserializes previously initialized account data. Should fail for all
    /// uninitialized accounts, where the bytes are zeroed. Implementations
    /// should be unique to a particular account type so that one can never
    /// successfully deserialize the data of one account type into another.
    /// For example, if the SPL token program were to implement this trait,
    /// it should be impossible to deserialize a `Mint` account into a token
    /// `Account`.
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self, AccountDeserializeError> {
        let mut data: &[u8] = &buf[8..];
        BorshDeserialize::deserialize(&mut data)
            .map_err(|_| AccountDeserializeError::DidNotDeserialize.into())
    }

    /// Deserializes account data without checking the account discriminator.
    /// This should only be used on account initialization, when the bytes of
    /// the account are zeroed.
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, AccountDeserializeError> {
        if buf.len() < Self::DISCRIMINATOR.len() {
            return Err(AccountDeserializeError::DiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &Self::DISCRIMINATOR != given_disc {
            return Err(AccountDeserializeError::DiscriminatorMismatch.into());
        }
        Self::try_deserialize_unchecked(buf)
    }
}

/// Calculates the data for an instruction invocation, where the data is
/// `Sha256(<namespace>:<method_name>)[..8] || BorshSerialize(args)`.
/// `args` is a borsh serialized struct of named fields for each argument given
/// to an instruction.
pub trait InstructionData: BorshSerialize + BorshDeserialize + Discriminator {
    fn data(&self) -> Vec<u8> {
        let mut result = Self::DISCRIMINATOR.to_vec();
        result.append(&mut self.try_to_vec().expect("Instruction data must serialize"));
        result
    }
}

pub trait ToAccountMetas: Owner {
    fn to_account_metas(&self) -> Vec<AccountMeta>;
    type Data: InstructionData;
}

pub trait ToAccountInfos<'info>: ToAccountMetas {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>>;
}

pub struct InstructionBuilder<A, D> {
    pub accounts: A,
    pub data: D,
}

impl<'info, A> InstructionBuilder<A, A::Data>
where
    A: ToAccountInfos<'info>,
{
    pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> ProgramResult {
        invoke_signed(
            &self.into(),
            &self.accounts.to_account_infos(),
            signers_seeds,
        )
    }

    pub fn invoke(&self) -> ProgramResult {
        invoke(&self.into(), &self.accounts.to_account_infos())
    }
}

impl<A> From<&InstructionBuilder<A, A::Data>> for Instruction
where
    A: ToAccountMetas,
{
    fn from(
        InstructionBuilder {
            accounts: account_infos,
            data,
            ..
        }: &InstructionBuilder<A, A::Data>,
    ) -> Self {
        Instruction::new_with_bytes(A::owner(), &data.data(), account_infos.to_account_metas())
    }
}

#[cfg(test)]
mod tests {}
