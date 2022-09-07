use crate::state::fee::Fee;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([67, 3, 34, 114, 190, 185, 17, 62])]
pub struct ConfigMarinadeData {
    pub rewards_fee: Option<Fee>,
    pub slots_for_stake_delta: Option<u64>,
    pub min_stake: Option<u64>,
    pub min_deposit: Option<u64>,
    pub min_withdraw: Option<u64>,
    pub staking_sol_cap: Option<u64>,
    pub liquidity_sol_cap: Option<u64>,
    pub auto_add_validator_enabled: Option<bool>,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID,data=ConfigMarinadeData)]
pub struct ConfigMarinadeAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(signer)]
    pub admin_authority: Pubkey,
}

impl ConfigMarinadeData {
    pub fn with_rewards_fee(mut self, v: Fee) -> Self {
        let old = self.rewards_fee.replace(v);
        assert!(old.is_none(), "Parameter rewards_fee was already set");
        self
    }
    pub fn with_slots_for_stake_delta(mut self, v: u64) -> Self {
        let old = self.slots_for_stake_delta.replace(v);
        assert!(
            old.is_none(),
            "Parameter slots_for_stake_delta was already set"
        );
        self
    }
    pub fn with_min_stake(mut self, v: u64) -> Self {
        let old = self.min_stake.replace(v);
        assert!(old.is_none(), "Parameter min_stake was already set");
        self
    }
    pub fn with_min_deposit(mut self, v: u64) -> Self {
        let old = self.min_deposit.replace(v);
        assert!(old.is_none(), "Parameter min_deposit was already set");
        self
    }
    pub fn with_min_withdraw(mut self, v: u64) -> Self {
        let old = self.min_withdraw.replace(v);
        assert!(old.is_none(), "Parameter min_withdraw was already set");
        self
    }
    pub fn with_staking_sol_cap(mut self, v: u64) -> Self {
        let old = self.staking_sol_cap.replace(v);
        assert!(old.is_none(), "Parameter staking_sol_cap was already set");
        self
    }
    pub fn with_liquidity_sol_cap(mut self, v: u64) -> Self {
        let old = self.liquidity_sol_cap.replace(v);
        assert!(old.is_none(), "Parameter liquidity_sol_cap was already set");
        self
    }
    pub fn with_auto_add_validator_enabled(mut self, v: bool) -> Self {
        let old = self.auto_add_validator_enabled.replace(v);
        assert!(
            old.is_none(),
            "Parameter auto_add_validator_enabled was already set"
        );
        self
    }
}
