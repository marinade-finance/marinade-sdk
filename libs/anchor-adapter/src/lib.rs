use anchor_lang::{
    error::ErrorCode as AnchorErrorCode, AccountDeserialize as AnchorAccountDeserialize,
    Discriminator as AnchorDiscriminator, Owner as AnchorOwner,
};
use derive_more::Deref;
use micro_anchor::{AccountDeserialize, AccountDeserializeError, Discriminator, Owner};

#[derive(Deref)]
pub struct Wrapper<T>(T);

impl<T: Discriminator> AnchorDiscriminator for Wrapper<T> {
    const DISCRIMINATOR: [u8; 8] = T::DISCRIMINATOR;
    fn discriminator() -> [u8; 8] {
        T::DISCRIMINATOR
    }
}

impl<T: Owner> AnchorOwner for Wrapper<T> {
    fn owner() -> anchor_lang::prelude::Pubkey {
        T::owner()
    }
}

impl From<Wrapper<AccountDeserializeError>> for AnchorErrorCode {
    fn from(Wrapper(e): Wrapper<AccountDeserializeError>) -> Self {
        match e {
            AccountDeserializeError::DiscriminatorNotFound => {
                AnchorErrorCode::AccountDiscriminatorNotFound
            }
            AccountDeserializeError::DiscriminatorMismatch => {
                AnchorErrorCode::AccountDiscriminatorMismatch
            }
            AccountDeserializeError::DidNotDeserialize => AnchorErrorCode::AccountDidNotDeserialize,
        }
    }
}

impl<T: AccountDeserialize> AnchorAccountDeserialize for Wrapper<T> {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        T::try_deserialize_unchecked(buf)
            .map(Wrapper)
            .map_err(|e| AnchorErrorCode::from(Wrapper(e)).into())
    }
}
