use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    InstructionData, Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize,
)]
#[discriminator([50, 106, 66, 104, 99, 118, 145, 88])]
pub struct ChangeAuthorityData {
    pub admin: Option<Pubkey>,
    pub validator_manager: Option<Pubkey>,
    pub operational_sol_account: Option<Pubkey>,
    pub treasury_msol_account: Option<Pubkey>,
}

#[derive(InstructionAccounts)]
#[accounts(ownerid=crate::ID, data=ChangeAuthorityData)]
pub struct ChangeAuthorityAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(signer)]
    pub admin_authority: Pubkey,
}

impl ChangeAuthorityData {
    pub fn with_admin(mut self, v: Pubkey) -> Self {
        let old = self.admin.replace(v);
        assert!(old.is_none(), "Parameter admin pubkey was already set");
        self
    }
    pub fn with_validator_manager(mut self, v: Pubkey) -> Self {
        let old = self.validator_manager.replace(v);
        assert!(
            old.is_none(),
            "Parameter validator manager pubkey was already set"
        );
        self
    }
    pub fn with_operational_sol_account(mut self, v: Pubkey) -> Self {
        let old = self.operational_sol_account.replace(v);
        assert!(
            old.is_none(),
            "Parameter operational sol account pubkey was already set"
        );
        self
    }
    pub fn with_treasury_msol_account(mut self, v: Pubkey) -> Self {
        let old = self.treasury_msol_account.replace(v);
        assert!(
            old.is_none(),
            "Parameter treasury msol account pubkey was already set"
        );
        self
    }
}
