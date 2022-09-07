use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([101, 41, 206, 33, 216, 111, 25, 78])]
pub struct SetValidatorScoreData {
    pub index: u32,
    pub validator_vote: Pubkey,
    pub score: u32,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=SetValidatorScoreData)]
pub struct SetValidatorScoreAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(signer)]
    pub manager_authority: Pubkey,
    #[account(mut)]
    pub validator_list: Pubkey,
}
