use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([123, 69, 168, 195, 183, 213, 199, 214])]
pub struct EmergencyUnstakeData {
    pub stake_index: u32,
    pub validator_index: u32,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=EmergencyUnstakeData)]
pub struct EmergencyUnstakeAccounts {
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
    pub clock: Pubkey,
    pub stake_program: Pubkey,
}
