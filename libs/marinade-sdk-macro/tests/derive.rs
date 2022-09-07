#[cfg(test)]
mod tests {
    use borsh::{BorshDeserialize, BorshSerialize};
    use is_trait::is_trait;
    use marinade_sdk_macro::{InstructionAccounts, InstructionData};
    use solana_program::pubkey::Pubkey;

    #[test]
    fn test_instruction_data() {
        #[derive(InstructionData, BorshSerialize, BorshDeserialize)]
        #[discriminator([1,2,3,4,5,6,7,8])]
        struct SimpleTestData {
            lamports: u64,
        }

        assert!(is_trait!(SimpleTestData, micro_anchor::Discriminator));
        assert!(is_trait!(SimpleTestData, micro_anchor::InstructionData));
    }

    #[test]
    fn test_account_infos<'info>() {
        // SimpleTestData struct is required ->
        //   as the macro InstructionAccounts depends on existence of it
        #[derive(BorshSerialize, BorshDeserialize)]
        pub struct SimpleMyTestData {
            lamports: u64,
        }
        impl micro_anchor::InstructionData for SimpleMyTestData {}
        impl micro_anchor::Discriminator for SimpleMyTestData {
            const DISCRIMINATOR: [u8; 8] = ([1, 2, 3, 4, 5, 6, 7, 8]);
        }

        // for being able to work with micro anchor to_account_metas()
        use micro_anchor::ToAccountMetas;

        #[derive(InstructionAccounts)]
        #[accounts(ownerid=solana_program::bpf_loader::ID,data=SimpleMyTestData)]
        pub struct SimpleTestAccounts {
            #[account(mut)]
            pub writable_no_signer: Pubkey,
            #[account(mut, signer)]
            pub writable_signer: Pubkey,
            #[account(signer)]
            pub readable_signer: Pubkey,
            pub readable: Pubkey,
        }

        let writable_no_signer_pk = Pubkey::new_unique();
        let writable_signer_pk = Pubkey::new_unique();
        let readable_signer_pk = Pubkey::new_unique();
        let readable_no_signer_pk = Pubkey::new_unique();
        let simple_test_accounts = SimpleTestAccounts {
            writable_no_signer: writable_no_signer_pk,
            writable_signer: writable_signer_pk,
            readable_signer: readable_signer_pk,
            readable: readable_no_signer_pk,
        };

        assert!(is_trait!(SimpleTestAccounts, micro_anchor::Owner));
        assert!(is_trait!(SimpleTestAccountInfos, micro_anchor::Owner));

        let account_metas: Vec<solana_program::instruction::AccountMeta> =
            simple_test_accounts.to_account_metas();
        assert_eq!(account_metas.len(), 4);
        for am in account_metas {
            match am.pubkey {
                _ if am.pubkey == writable_no_signer_pk => {
                    assert!(!am.is_signer);
                    assert!(am.is_writable);
                }
                _ if am.pubkey == writable_signer_pk => {
                    assert!(am.is_signer);
                    assert!(am.is_writable);
                }
                _ if am.pubkey == readable_signer_pk => {
                    assert!(am.is_signer);
                    assert!(!am.is_writable);
                }
                _ if am.pubkey == readable_no_signer_pk => {
                    assert!(!am.is_signer);
                    assert!(!am.is_writable);
                }
                any_pk => panic!("Got unrecognized pub key {}", any_pk),
            }
        }
    }

    #[test]
    fn test_account_infos_nested<'info>() {
        // requred by InstructionAccounts
        #[derive(InstructionData, BorshSerialize, BorshDeserialize)]
        #[discriminator([1,2,3,4,5,6,7,8])]
        pub struct NestedTestData {
            nested_lamports: u64,
        }
        #[derive(InstructionData, BorshSerialize, BorshDeserialize)]
        #[discriminator([1,2,3,4,5,6,7,8])]
        pub struct OuterTestData {
            outer_lamports: u64,
        }

        // for being able to work with micro anchor to_account_metas()
        use micro_anchor::ToAccountMetas;

        #[derive(InstructionAccounts)]
        #[accounts(ownerid=solana_program::bpf_loader::ID,data=NestedTestData)]
        pub struct NestedTestAccounts {
            pub nested_pk: Pubkey,
        }

        #[derive(InstructionAccounts)]
        #[accounts(ownerid=solana_program::bpf_loader::ID,data=OuterTestData)]
        pub struct OuterTestAccounts {
            pub outer_pk: Pubkey,
            pub nested_struct: NestedTestAccounts,
        }

        let outer_pk = Pubkey::new_unique();
        let nested_pk = Pubkey::new_unique();
        let test_accounts = OuterTestAccounts {
            outer_pk,
            nested_struct: NestedTestAccounts { nested_pk },
        };

        assert!(is_trait!(OuterTestAccounts, micro_anchor::Owner));
        assert!(is_trait!(OuterTestAccountInfos, micro_anchor::Owner));

        let account_metas: Vec<solana_program::instruction::AccountMeta> =
            test_accounts.to_account_metas();
        assert_eq!(account_metas.len(), 2);
    }
}
