use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([165, 158, 229, 97, 168, 220, 187, 225])]
pub struct DeactivateStakeData {
    pub stake_index: u32,
    pub validator_index: u32,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=DeactivateStakeData)]
pub struct DeactivateStakeAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    pub reserve_pda: Pubkey,
    #[account(mut)]
    pub validator_list: Pubkey,
    #[account(mut)]
    pub stake_list: Pubkey,
    #[account(mut)]
    pub stake_account: Pubkey,
    pub stake_deposit_authority: Pubkey,
    #[account(mut, signer)]
    pub split_stake_account: Pubkey,
    #[account(mut, signer)]
    pub split_stake_rent_payer: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub epoch_schedule: Pubkey,
    pub stake_history: Pubkey,
    pub system_program: Pubkey,
    pub stake_program: Pubkey,
}
