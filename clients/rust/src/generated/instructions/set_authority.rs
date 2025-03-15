//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
#[derive(Debug)]
pub struct SetAuthority {
    /// Buffer or ProgramData account.
    pub buffer_or_program_data_account: solana_program::pubkey::Pubkey,
    /// Current authority.
    pub current_authority: solana_program::pubkey::Pubkey,
    /// New authority (optional).
    pub new_authority: Option<solana_program::pubkey::Pubkey>,
}

impl SetAuthority {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.buffer_or_program_data_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.current_authority,
            true,
        ));
        if let Some(new_authority) = self.new_authority {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                new_authority,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::QED_DATA_LOADER_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&SetAuthorityInstructionData::new()).unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::QED_DATA_LOADER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetAuthorityInstructionData {
    discriminator: u8,
}

impl SetAuthorityInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 2 }
    }
}

impl Default for SetAuthorityInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `SetAuthority`.
///
/// ### Accounts:
///
///   0. `[writable]` buffer_or_program_data_account
///   1. `[signer]` current_authority
///   2. `[optional]` new_authority
#[derive(Clone, Debug, Default)]
pub struct SetAuthorityBuilder {
    buffer_or_program_data_account: Option<solana_program::pubkey::Pubkey>,
    current_authority: Option<solana_program::pubkey::Pubkey>,
    new_authority: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl SetAuthorityBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Buffer or ProgramData account.
    #[inline(always)]
    pub fn buffer_or_program_data_account(
        &mut self,
        buffer_or_program_data_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.buffer_or_program_data_account = Some(buffer_or_program_data_account);
        self
    }
    /// Current authority.
    #[inline(always)]
    pub fn current_authority(
        &mut self,
        current_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.current_authority = Some(current_authority);
        self
    }
    /// `[optional account]`
    /// New authority (optional).
    #[inline(always)]
    pub fn new_authority(
        &mut self,
        new_authority: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.new_authority = new_authority;
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = SetAuthority {
            buffer_or_program_data_account: self
                .buffer_or_program_data_account
                .expect("buffer_or_program_data_account is not set"),
            current_authority: self
                .current_authority
                .expect("current_authority is not set"),
            new_authority: self.new_authority,
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `set_authority` CPI accounts.
pub struct SetAuthorityCpiAccounts<'a, 'b> {
    /// Buffer or ProgramData account.
    pub buffer_or_program_data_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Current authority.
    pub current_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// New authority (optional).
    pub new_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `set_authority` CPI instruction.
pub struct SetAuthorityCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Buffer or ProgramData account.
    pub buffer_or_program_data_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Current authority.
    pub current_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// New authority (optional).
    pub new_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

impl<'a, 'b> SetAuthorityCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: SetAuthorityCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            buffer_or_program_data_account: accounts.buffer_or_program_data_account,
            current_authority: accounts.current_authority,
            new_authority: accounts.new_authority,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.buffer_or_program_data_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.current_authority.key,
            true,
        ));
        if let Some(new_authority) = self.new_authority {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *new_authority.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::QED_DATA_LOADER_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = borsh::to_vec(&SetAuthorityInstructionData::new()).unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::QED_DATA_LOADER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(4 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.buffer_or_program_data_account.clone());
        account_infos.push(self.current_authority.clone());
        if let Some(new_authority) = self.new_authority {
            account_infos.push(new_authority.clone());
        }
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `SetAuthority` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` buffer_or_program_data_account
///   1. `[signer]` current_authority
///   2. `[optional]` new_authority
#[derive(Clone, Debug)]
pub struct SetAuthorityCpiBuilder<'a, 'b> {
    instruction: Box<SetAuthorityCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SetAuthorityCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SetAuthorityCpiBuilderInstruction {
            __program: program,
            buffer_or_program_data_account: None,
            current_authority: None,
            new_authority: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Buffer or ProgramData account.
    #[inline(always)]
    pub fn buffer_or_program_data_account(
        &mut self,
        buffer_or_program_data_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.buffer_or_program_data_account = Some(buffer_or_program_data_account);
        self
    }
    /// Current authority.
    #[inline(always)]
    pub fn current_authority(
        &mut self,
        current_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.current_authority = Some(current_authority);
        self
    }
    /// `[optional account]`
    /// New authority (optional).
    #[inline(always)]
    pub fn new_authority(
        &mut self,
        new_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.new_authority = new_authority;
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = SetAuthorityCpi {
            __program: self.instruction.__program,

            buffer_or_program_data_account: self
                .instruction
                .buffer_or_program_data_account
                .expect("buffer_or_program_data_account is not set"),

            current_authority: self
                .instruction
                .current_authority
                .expect("current_authority is not set"),

            new_authority: self.instruction.new_authority,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct SetAuthorityCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    buffer_or_program_data_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    current_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    new_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
