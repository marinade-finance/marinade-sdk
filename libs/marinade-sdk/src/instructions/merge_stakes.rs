use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([216, 36, 141, 225, 243, 78, 125, 237])]
pub struct MergeStakesData {
    pub destination_stake_index: u32,
    pub source_stake_index: u32,
    pub validator_index: u32,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=MergeStakesData)]
pub struct MergeStakesAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(mut)]
    pub stake_list: Pubkey,
    #[account(mut)]
    pub validator_list: Pubkey,
    #[account(mut)]
    pub destination_stake: Pubkey,
    #[account(mut)]
    pub source_stake: Pubkey,
    pub stake_deposit_authority: Pubkey,
    pub stake_withdraw_authority: Pubkey,
    #[account(mut)]
    pub operational_sol_account: Pubkey,
    pub clock: Pubkey,
    pub stake_history: Pubkey,
    pub stake_program: Pubkey,
}
