use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([250, 113, 53, 54, 141, 117, 215, 185])]
pub struct AddValidatorData {
    pub score: u32,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=AddValidatorData)]
pub struct AddValidatorAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(signer)]
    pub manager_authority: Pubkey,
    #[account(mut)]
    pub validator_list: Pubkey,
    pub validator_vote: Pubkey,
    #[account(mut)]
    pub duplication_flag: Pubkey,
    #[account(mut, signer)]
    pub rent_payer: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
