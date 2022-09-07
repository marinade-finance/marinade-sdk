use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([242, 35, 198, 137, 82, 225, 242, 182])]
pub struct DepositData {
    pub lamports: u64,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=DepositData)]
pub struct DepositAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(mut)]
    pub msol_mint: Pubkey,
    #[account(mut)]
    pub liq_pool_sol_leg_pda: Pubkey,
    #[account(mut)]
    pub liq_pool_msol_leg: Pubkey,
    pub liq_pool_msol_leg_authority: Pubkey,
    #[account(mut)]
    pub reserve_pda: Pubkey,
    #[account(mut, signer)]
    pub transfer_from: Pubkey,
    #[account(mut)]
    pub mint_to: Pubkey,
    pub msol_mint_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
