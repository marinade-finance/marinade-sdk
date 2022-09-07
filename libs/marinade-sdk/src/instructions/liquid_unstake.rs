use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([30, 30, 119, 240, 191, 227, 12, 16])]
pub struct LiquidUnstakeData {
    pub msol_amount: u64,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=LiquidUnstakeData)]
pub struct LiquidUnstakeAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(mut)]
    pub msol_mint: Pubkey,
    #[account(mut)]
    pub liq_pool_sol_leg_pda: Pubkey,
    #[account(mut)]
    pub liq_pool_msol_leg: Pubkey,
    #[account(mut)]
    pub treasury_msol_account: Pubkey,
    #[account(mut)]
    pub get_msol_from: Pubkey,
    #[account(signer)]
    pub get_msol_from_authority: Pubkey,
    #[account(mut)]
    pub transfer_sol_to: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
