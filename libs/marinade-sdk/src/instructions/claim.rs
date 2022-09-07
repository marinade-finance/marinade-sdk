use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([62, 198, 214, 193, 213, 159, 108, 210])]
pub struct ClaimData {}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=ClaimData)]
pub struct ClaimAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(mut)]
    pub reserve_pda: Pubkey,
    #[account(mut)]
    pub ticket_account: Pubkey,
    #[account(mut)]
    pub transfer_sol_to: Pubkey,
    pub clock: Pubkey,
    pub system_program: Pubkey,
}
