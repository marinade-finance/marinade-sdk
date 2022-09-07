use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([110, 130, 115, 41, 164, 102, 2, 59])]
pub struct DepositStakeAccountData {
    pub validator_index: u32,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID, data=DepositStakeAccountData)]
pub struct DepositStakeAccountAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(mut)]
    pub validator_list: Pubkey,
    #[account(mut)]
    pub stake_list: Pubkey,
    #[account(mut)]
    pub stake_account: Pubkey,
    #[account(signer)]
    pub stake_authority: Pubkey,
    #[account(mut)]
    pub duplication_flag: Pubkey,
    #[account(mut, signer)]
    pub rent_payer: Pubkey,
    #[account(mut)]
    pub msol_mint: Pubkey,
    #[account(mut)]
    pub mint_to: Pubkey,
    pub msol_mint_authority: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
}
