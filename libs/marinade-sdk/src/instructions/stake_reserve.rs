use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([87, 217, 23, 179, 205, 25, 113, 129])]
pub struct StakeReserveData {
    pub validator_index: u32,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=StakeReserveData)]
pub struct StakeReserveAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(mut)]
    pub validator_list: Pubkey,
    #[account(mut)]
    pub stake_list: Pubkey,
    #[account(mut)]
    pub validator_vote: Pubkey,
    #[account(mut)]
    pub reserve_pda: Pubkey,
    #[account(mut)]
    pub stake_account: Pubkey,
    pub stake_deposit_authority: Pubkey,
    pub clock: Pubkey,
    pub epoch_schedule: Pubkey,
    pub rent: Pubkey,
    pub stake_history: Pubkey,
    pub stake_config: Pubkey,
    pub system_program: Pubkey,
    pub stake_program: Pubkey,
}
