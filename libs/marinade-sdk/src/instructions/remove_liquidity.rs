use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([80, 85, 209, 72, 24, 206, 177, 108])]
pub struct RemoveLiquidityData {
    pub tokens: u64,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=RemoveLiquidityData)]
pub struct RemoveLiquidityAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(mut)]
    pub lp_mint: Pubkey,
    #[account(mut)]
    pub burn_from: Pubkey,
    #[account(signer)]
    pub burn_from_authority: Pubkey,
    #[account(mut)]
    pub transfer_sol_to: Pubkey,
    #[account(mut)]
    pub transfer_msol_to: Pubkey,
    #[account(mut)]
    pub liq_pool_sol_leg_pda: Pubkey,
    #[account(mut)]
    pub liq_pool_msol_leg: Pubkey,
    pub liq_pool_msol_leg_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
