use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([27, 90, 97, 209, 17, 115, 7, 40])]
pub struct ConfigValidatorSystemData {
    pub extra_runs: u32,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=ConfigValidatorSystemData)]
pub struct ConfigValidatorSystemAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(signer)]
    pub manager_authority: Pubkey,
}
