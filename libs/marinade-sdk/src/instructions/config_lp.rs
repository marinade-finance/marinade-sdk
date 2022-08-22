use crate::state::fee::Fee;
use borsh::{BorshDeserialize, BorshSerialize};
use micro_anchor::{Discriminator, InstructionData, Owner, ToAccountInfos, ToAccountMetas};
use solana_program::{account_info::AccountInfo, instruction::AccountMeta, pubkey::Pubkey};

#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ConfigLpData {
    pub min_fee: Option<Fee>,
    pub max_fee: Option<Fee>,
    pub liquidity_target: Option<u64>,
    pub treasury_cut: Option<Fee>,
}

impl Discriminator for ConfigLpData {
    const DISCRIMINATOR: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
}

impl InstructionData for ConfigLpData {}

pub struct ConfigLpAccounts {
    pub marinade: Pubkey,
    pub admin_authority: Pubkey,
}

impl Owner for ConfigLpAccounts {
    fn owner() -> Pubkey {
        crate::ID
    }
}

impl ToAccountMetas for ConfigLpAccounts {
    fn to_account_metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.marinade, false),
            AccountMeta::new_readonly(self.admin_authority, true),
        ]
    }

    type Data = ConfigLpData;
}

pub struct ConfigLpAccountInfos<'info> {
    pub marinade: AccountInfo<'info>,
    pub admin_authority: AccountInfo<'info>,
}

impl<'info> Owner for ConfigLpAccountInfos<'info> {
    fn owner() -> Pubkey {
        crate::ID
    }
}

impl<'info> From<&ConfigLpAccountInfos<'info>> for ConfigLpAccounts {
    fn from(
        ConfigLpAccountInfos {
            marinade,
            admin_authority,
        }: &ConfigLpAccountInfos<'info>,
    ) -> Self {
        Self {
            marinade: marinade.key.clone(),
            admin_authority: admin_authority.key.clone(),
        }
    }
}

impl<'info> ToAccountMetas for ConfigLpAccountInfos<'info> {
    fn to_account_metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.marinade.key.clone(), false),
            AccountMeta::new_readonly(self.admin_authority.key.clone(), true),
        ]
    }
    type Data = ConfigLpData;
}

impl<'info> ToAccountInfos<'info> for ConfigLpAccountInfos<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.marinade.clone(), self.admin_authority.clone()]
    }
}