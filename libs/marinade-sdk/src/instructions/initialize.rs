use crate::state::fee::Fee;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([175, 175, 109, 31, 13, 152, 155, 237])]
pub struct InitializeData {
    pub admin_authority: Pubkey,
    pub validator_manager_authority: Pubkey,
    pub min_stake: u64,
    pub reward_fee: Fee,
    pub liq_pool: LiqPoolInitializeData,
    pub additional_stake_record_space: u32,
    pub additional_validator_record_space: u32,
    pub slots_for_stake_delta: u64,
}

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([1,2,3,4,5,6,7,8])] // fake discriminator
pub struct LiqPoolInitializeData {
    pub lp_liquidity_target: u64,
    pub lp_max_fee: Fee,
    pub lp_min_fee: Fee,
    pub lp_treasury_cut: Fee,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=InitializeData)]
pub struct InitializeAccounts {
    #[account(signer)]
    pub creator_authority: Pubkey,
    pub marinade: Pubkey, // state
    pub reserve_pda: Pubkey,
    #[account(mut)]
    pub stake_list: Pubkey,
    #[account(mut)]
    pub validator_list: Pubkey,
    pub msol_mint: Pubkey,
    pub operational_sol_account: Pubkey,
    pub liq_pool: LiqPoolInitializeAccounts,
    pub treasury_msol_account: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=LiqPoolInitializeData)]
pub struct LiqPoolInitializeAccounts {
    pub lp_mint: Pubkey,
    pub sol_leg_pda: Pubkey,
    pub msol_leg: Pubkey,
}
