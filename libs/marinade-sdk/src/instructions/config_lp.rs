use crate::state::fee::Fee;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([10, 24, 168, 119, 86, 48, 225, 17])]
pub struct ConfigLpData {
    pub min_fee: Option<Fee>,
    pub max_fee: Option<Fee>,
    pub liquidity_target: Option<u64>,
    pub treasury_cut: Option<Fee>,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID, data=ConfigLpData)]
pub struct ConfigLpAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(signer)]
    pub admin_authority: Pubkey,
}

impl ConfigLpData {
    pub fn with_min_fee(mut self, v: Fee) -> Self {
        let old = self.min_fee.replace(v);
        assert!(old.is_none(), "Min fee was already set");
        self
    }

    pub fn with_max_fee(mut self, v: Fee) -> Self {
        let old = self.max_fee.replace(v);
        assert!(old.is_none(), "Max fee was already set");
        self
    }

    pub fn with_liquidity_target(mut self, v: u64) -> Self {
        let old = self.liquidity_target.replace(v);
        assert!(old.is_none(), "Liquidity target was already set");
        self
    }

    pub fn with_treasury_cut(mut self, v: Fee) -> Self {
        let old = self.treasury_cut.replace(v);
        assert!(old.is_none(), "Treasury cut was already set");
        self
    }
}
