use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Path, Token};
use syn_unnamed_struct::{Meta, MetaPath};

/// Example of macro generation that generates
/// `impl InstructionData` and `Discriminator`
///
/// ```
/// use marinade_sdk_macro::InstructionData;
/// use borsh::{BorshDeserialize, BorshSerialize};
///
/// #[derive(InstructionData, BorshSerialize, BorshDeserialize)]
/// #[discriminator([1,2,3,4,5,6,7,8])]
/// pub struct TestData {
///   pub lamports: u64
/// }
/// ```
///
/// will generate
///
/// ```
/// use borsh::{BorshDeserialize, BorshSerialize};
///
/// #[derive(BorshSerialize, BorshDeserialize)]
/// pub struct TestData {
///   pub lamports: u64
/// }
///
/// // GENERATED CODE:
/// impl micro_anchor::InstructionData for TestData {}
/// impl micro_anchor::Discriminator for TestData {
///   const DISCRIMINATOR: [u8; 8] = ([1,2,3,4,5,6,7,8]);
/// }
/// ```
#[proc_macro_derive(InstructionData, attributes(discriminator))]
pub fn derive_instruction_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, attrs, .. } = &input;

    // token stream to be used as return value
    let mut output = proc_macro2::TokenStream::new();

    // Data struct requires implementing InstructionData trait
    let name = ident;
    let expanded = quote! {
        impl micro_anchor::InstructionData for #name {}
    };
    output.extend(expanded);

    // discriminator attribute is required
    let discriminator_attrs = attrs
        .iter()
        .filter(|a| a.path.is_ident("discriminator"))
        .flat_map(|attr| attr.tokens.clone())
        .collect::<Vec<_>>();
    if let Some(discriminator_attr) = discriminator_attrs.get(0) {
        let discriminator_impl = quote! {
            impl micro_anchor::Discriminator for #name {
                const DISCRIMINATOR:[u8;8] = #discriminator_attr;
            }
        };
        output.extend(discriminator_impl);
    } else {
        panic!("Discriminator attribute is required for macro InstructionData, as parameter required [u8;8].")
    }

    output.into()
}

const AM_READ_ONLY: &str = "solana_program::instruction::AccountMeta::new_readonly({}, false)";
const AM_READ_ONLY_SIGNER: &str =
    "solana_program::instruction::AccountMeta::new_readonly({}, true)";
const AM_MUT: &str = "solana_program::instruction::AccountMeta::new({}, false)";
const AM_MUT_SIGNER: &str = "solana_program::instruction::AccountMeta::new({}, true)";

struct AccountsFieldData {
    name: String,
    type_name: TokenStream2,
    type_is_pubkey: bool,
    account_meta_formatter: String,
    signer: bool,
    mutate: bool,
}

impl AccountsFieldData {
    fn new(
        name: String,
        account_meta_formatter: String,
        type_name: TokenStream2,
        type_is_pubkey: bool,
    ) -> Self {
        AccountsFieldData {
            name,
            type_name,
            type_is_pubkey,
            account_meta_formatter,
            signer: false,
            mutate: false,
        }
    }
}

struct AccountsNameValueParser {
    pub path: MetaPath,
    pub _eq_token: Token![=],
    pub value: Path,
}
impl Parse for AccountsNameValueParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse::<MetaPath>()?;
        Ok(AccountsNameValueParser {
            path,
            _eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

#[derive(Default)]
struct AccountsMetaAttributes {
    pub owner_id: Option<TokenStream2>,
    pub data_struct_name: Option<TokenStream2>,
}

fn get_related_struct_idents(
    struct_name_as_string: String,
) -> (proc_macro2::Ident, proc_macro2::Ident) {
    let struct_name_stripped = struct_name_as_string.strip_suffix("Accounts");
    if struct_name_stripped.is_none() {
        panic!(
            "Struct {} annotated with InstructionAccounts is expected to have a name ending with 'Accounts'",
            struct_name_as_string
        );
    }
    let struct_name_stripped = struct_name_stripped.unwrap();
    let infos_struct_name = proc_macro2::Ident::new(
        format!("{}AccountInfos", struct_name_stripped).as_str(),
        proc_macro2::Span::call_site(),
    );
    let data_struct_name = proc_macro2::Ident::new(
        format!("{}Data", struct_name_stripped).as_str(),
        proc_macro2::Span::call_site(),
    );
    (data_struct_name, infos_struct_name)
}

fn emit_struct_fields_non_pubkey(
    struct_fields: &Vec<(Ident, AccountsFieldData)>,
    base_pattern: &str,
    f: fn(&AccountsFieldData) -> String,
) -> Vec<TokenStream2> {
    struct_fields
        .iter()
        .filter_map(|(_, props)| {
            let adjusted_pattern = base_pattern.replace("{}", f(props).as_str());
            let parsed_pattern: proc_macro2::TokenStream = adjusted_pattern.parse().unwrap();
            if !props.type_is_pubkey {
                Some(quote!(#parsed_pattern))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn emit_struct_fields_at_pattern(
    struct_fields: &Vec<(Ident, AccountsFieldData)>,
    base_pattern: &str,
    f: fn(&AccountsFieldData) -> &str,
) -> Vec<TokenStream2> {
    struct_fields
        .iter()
        .filter_map(|(_, props)| {
            let self_name = base_pattern.replace("{}", f(props));
            let account_meta_def: proc_macro2::TokenStream = props
                .account_meta_formatter
                .replace("{}", self_name.as_str())
                .parse()
                .unwrap();
            if props.type_is_pubkey {
                Some(quote!(#account_meta_def))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

/// Example of macro generation that will generate a new struct `TestAccountInfos`
/// and all micro anchor implementations required for the instruction would work.
///
/// ```
/// use marinade_sdk_macro::{InstructionAccounts, InstructionData};
/// use borsh::{BorshDeserialize, BorshSerialize};
///
/// // TestData is reequired as this macro depends on existent of it, based on its name!
/// #[derive(InstructionData, BorshSerialize, BorshDeserialize)]
/// #[discriminator([1,2,3,4,5,6,7,8])]
/// pub struct TestData {
///   pub lamports: u64
/// }
///
/// #[derive(InstructionAccounts)]
/// #[accounts(ownerid=solana_program::bpf_loader::ID, data=TestData)]
/// pub struct TestAccounts {
///     #[account(mut)]
///     pub marinade: solana_program::pubkey::Pubkey,
/// }
/// ```
///
/// ```
/// use marinade_sdk_macro::InstructionData;
/// use borsh::{BorshDeserialize, BorshSerialize};
///
/// #[derive(InstructionData, BorshSerialize, BorshDeserialize)]
/// #[discriminator([1,2,3,4,5,6,7,8])]
/// pub struct TestData {
///   pub lamports: u64
/// }
///
/// pub struct TestAccounts {
///     pub marinade: solana_program::pubkey::Pubkey,
/// }
///
/// // GENERATED CODE:
/// impl micro_anchor::Owner for TestAccounts {
///     fn owner() -> solana_program::pubkey::Pubkey {
///         solana_program::bpf_loader::ID
///     }
/// }
/// pub struct TestAccountInfos<'info> {
///     pub marinade: solana_program::account_info::AccountInfo<'info>,
/// }
/// impl<'info> From<&TestAccountInfos<'info>> for TestAccounts {
///     fn from(
///         TestAccountInfos {
///             marinade,
///         }: &TestAccountInfos<'info>,
///     ) -> Self {
///         Self {
///             marinade: marinade.key.clone(),
///         }
///     }
/// }
/// impl<'info> micro_anchor::ToAccountInfos<'info> for TestAccountInfos<'info> {
///     fn to_account_infos(
///         &self,
///     ) -> Vec<solana_program::account_info::AccountInfo<'info>> {
///             vec![self.marinade.clone()]
///     }
/// }
/// impl micro_anchor::ToAccountMetas for TestAccounts {
///     fn to_account_metas(&self) -> Vec<solana_program::instruction::AccountMeta> {
///         vec![solana_program::instruction::AccountMeta::new(self.marinade, false)]
///     }
///     type Data = TestData;
/// }
/// impl<'info> micro_anchor::ToAccountMetas for TestAccountInfos<'info> {
///     fn to_account_metas(&self) -> Vec<solana_program::instruction::AccountMeta> {
///         vec![solana_program::instruction::AccountMeta::new(self.marinade.key.clone(), false)]
///     }
///     type Data = TestData;
/// }
/// impl<'info> micro_anchor::Owner for TestAccountInfos<'info> {
///     fn owner() -> solana_program::pubkey::Pubkey {
///         solana_program::bpf_loader::ID
///     }
/// }
/// ```
#[proc_macro_derive(InstructionAccounts, attributes(account, accounts))]
pub fn derive_instruction_accounts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput {
        ident, attrs, data, ..
    } = &input;

    let struct_name = ident.clone();

    let mut meta_attributes = AccountsMetaAttributes::default();
    attrs
        .iter()
        .filter(|a| a.path.is_ident("accounts"))
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<AccountsNameValueParser, Token![,]>::parse_terminated)
                .expect("Could not parse 'accounts' attribute")
        })
        .for_each(|meta| {
            match meta.path.to_token_stream().to_string().as_str() {
                "ownerid" => meta_attributes.owner_id = Some(meta.value.to_token_stream()),
                "data" => meta_attributes.data_struct_name = Some(meta.value.to_token_stream()),
                x => panic!("'accounts' attribute contain unexpected 'name' {}", x),
            };
        });
    if meta_attributes.owner_id.is_none() {
        panic!(
            "'ownerid' argument of 'accounts' attribute is required for macro InstructionAccounts"
        );
    }
    let owner_id = meta_attributes.owner_id.unwrap();

    let (generatad_data_struct_name, infos_struct_name) =
        get_related_struct_idents(struct_name.to_string());
    let data_struct_name = if meta_attributes.data_struct_name.is_some() {
        meta_attributes.data_struct_name.unwrap()
    } else {
        generatad_data_struct_name.to_token_stream()
    };

    let obj = match data {
        syn::Data::Struct(obj) => obj,
        _ => panic!("Only structs supported in InstructionAccounts macro"),
    };
    let struct_fields = obj
        .fields
        .iter()
        .map(|field| {
            let field_ident = field
                .ident
                .as_ref()
                .expect("Structs must contain named fields")
                .clone();
            // when pubkey we want to work with AccountMeta conversion, otherwise leaving it
            let type_is_pubkey = match &field.ty {
                syn::Type::Path(_ty) => field.ty.to_token_stream().to_string().ends_with("Pubkey"),
                _ => false,
            };

            let mut field_data = AccountsFieldData::new(
                field_ident.to_token_stream().to_string(),
                AM_READ_ONLY.to_string(),
                field.ty.to_token_stream(),
                type_is_pubkey,
            );
            field
                .attrs
                .iter()
                .filter(|a| a.path.is_ident("account"))
                .flat_map(|attr| {
                    attr.parse_args_with(<Punctuated<Meta, Token![,]>>::parse_terminated)
                        .expect("Could not parse 'from' attribute")
                })
                .for_each(|meta| match meta {
                    Meta::Path(path) => match path.to_token_stream().to_string().as_str() {
                        "mut" => {
                            field_data.mutate = true;
                        }
                        "signer" => {
                            field_data.signer = true;
                        }
                        _ => panic!("Unrecognized attribute of field '{}'", field_data.name),
                    },
                    _ => panic!(
                        "Attribute for field {} contains unrecognized value",
                        field_data.name
                    ),
                });
            if field_data.signer && field_data.mutate {
                field_data.account_meta_formatter = AM_MUT_SIGNER.to_string();
            } else if field_data.mutate {
                field_data.account_meta_formatter = AM_MUT.to_string();
            } else if field_data.signer {
                field_data.account_meta_formatter = AM_READ_ONLY_SIGNER.to_string();
            }

            (field_ident, field_data)
        })
        .collect::<Vec<_>>();
    // preparation of identifiers and data to be quoted later
    let fields_declaration = struct_fields
        .iter()
        .map(|(field, props)| {
            if props.type_is_pubkey {
                quote!(pub #field: solana_program::account_info::AccountInfo<'info>)
            } else {
                let (_, type_ai_name) = get_related_struct_idents(props.type_name.to_string());
                quote!(pub #field: #type_ai_name<'info>)
            }
        })
        .collect::<Vec<_>>();
    let fields_names = struct_fields
        .iter()
        .map(|(field, _)| quote!(#field))
        .collect::<Vec<_>>();
    let from_infos_fields_declaration_cloning = struct_fields
        .iter()
        .map(|(field, props)| {
            if props.type_is_pubkey {
                quote!(#field: #field.key.clone())
            } else {
                quote!(#field: #field.into())
            }
        })
        .collect::<Vec<_>>();
    let to_infos_fields_cloning_inner = struct_fields
        .iter()
        .filter_map(|(field, props)| {
            if props.type_is_pubkey {
                Some(quote!(self.#field.clone()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let to_account_metas_inner_fields = emit_struct_fields_at_pattern(
        &struct_fields,
        "self.{}",
        |props: &AccountsFieldData| -> &str { props.name.as_str() },
    );
    let to_account_metas_inner_cloning = emit_struct_fields_at_pattern(
        &struct_fields,
        "self.{}.key.clone()",
        |props: &AccountsFieldData| -> &str { props.name.as_str() },
    );
    let to_account_metas_nested_iter_fields = emit_struct_fields_non_pubkey(
        &struct_fields,
        "self.{}.to_account_metas().into_iter().for_each(|i| output.push(i));",
        |props: &AccountsFieldData| -> String { props.name.clone() },
    );
    let to_account_infos_nested_iter_fields = emit_struct_fields_non_pubkey(
        &struct_fields,
        "self.{}.to_account_infos().into_iter().for_each(|i| output.push(i));",
        |props: &AccountsFieldData| -> String { props.name.clone() },
    );

    let token_stream_output = quote! {
        pub struct #infos_struct_name<'info> {
            #(#fields_declaration),*
        }
        impl<'info> From<&#infos_struct_name<'info>> for #struct_name {
            fn from(
                #infos_struct_name {
                    #(#fields_names),*
                }: &#infos_struct_name<'info>,
            ) -> Self {
                Self {
                    #(#from_infos_fields_declaration_cloning),*
                }
            }
        }
        impl #struct_name {
            fn to_account_metas_inner(&self) -> Vec<solana_program::instruction::AccountMeta> {
                vec![
                    #(#to_account_metas_inner_fields),*
                ]
            }
        }
        impl micro_anchor::ToAccountMetas for #struct_name {
            fn to_account_metas(&self) -> Vec<solana_program::instruction::AccountMeta> {
                let mut output: Vec<solana_program::instruction::AccountMeta> = Vec::new();
                self.to_account_metas_inner().into_iter().for_each(|i| output.push(i));
                #(#to_account_metas_nested_iter_fields);*
                output
            }
            type Data = #data_struct_name;
        }
        impl<'info> #infos_struct_name<'info> {
             fn to_account_infos_inner(&self) -> Vec<solana_program::account_info::AccountInfo<'info>> {
                vec![
                    #(#to_infos_fields_cloning_inner),*
                ]
            }
            fn to_account_metas_inner(&self) -> Vec<solana_program::instruction::AccountMeta> {
                vec![
                    #(#to_account_metas_inner_cloning),*
                ]
            }
        }
        impl<'info> micro_anchor::ToAccountInfos<'info> for #infos_struct_name<'info> {
            fn to_account_infos(&self) -> Vec<solana_program::account_info::AccountInfo<'info>> {
                let mut output: Vec<solana_program::account_info::AccountInfo<'info>> = Vec::new();
                self.to_account_infos_inner().into_iter().for_each(|i| output.push(i));
                #(#to_account_infos_nested_iter_fields);*
                output
            }
        }
        impl<'info> micro_anchor::ToAccountMetas for #infos_struct_name<'info> {
            fn to_account_metas(&self) -> Vec<solana_program::instruction::AccountMeta> {
                let mut output: Vec<solana_program::instruction::AccountMeta> = Vec::new();
                self.to_account_metas_inner().into_iter().for_each(|i| output.push(i));
                #(#to_account_metas_nested_iter_fields);*
                output
            }
            type Data = #data_struct_name;
        }
        impl micro_anchor::Owner for #struct_name {
            fn owner() -> solana_program::pubkey::Pubkey {
                #owner_id
            }
        }
        impl<'info> micro_anchor::Owner for #infos_struct_name<'info> {
            fn owner() -> solana_program::pubkey::Pubkey {
                #owner_id
            }
        }
    };

    TokenStream::from(token_stream_output)
}
