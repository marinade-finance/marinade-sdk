use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([55, 241, 205, 221, 45, 114, 205, 163])]
pub struct PartialUnstakeData {
    pub stake_index: u32,
    pub validator_index: u32,
    pub desired_unstake_amount: u64,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=PartialUnstakeData)]
pub struct PartialUnstakeAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(signer)]
    pub validator_manager_authority: Pubkey,
    #[account(mut)]
    pub validator_list: Pubkey,
    #[account(mut)]
    pub stake_list: Pubkey,
    #[account(mut)]
    pub stake_account: Pubkey,
    pub stake_deposit_authority: Pubkey,
    pub reserve_pda: Pubkey,
    #[account(mut, signer)]
    pub split_stake_account: Pubkey,
    #[account(mut, signer)]
    pub split_stake_rent_payer: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub stake_history: Pubkey,
    pub system_program: Pubkey,
    pub stake_program: Pubkey,
}
