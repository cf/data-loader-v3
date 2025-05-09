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
pub struct Write {
    /// Buffer account.
    pub buffer_account: solana_program::pubkey::Pubkey,
    /// Buffer authority.
    pub buffer_authority: solana_program::pubkey::Pubkey,
}

impl Write {
    pub fn instruction(
        &self,
        args: WriteInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: WriteInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.buffer_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.buffer_authority,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&WriteInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::QED_DATA_LOADER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WriteInstructionData {
    discriminator: u8,
}

impl WriteInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 1 }
    }
}

impl Default for WriteInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WriteInstructionArgs {
    pub offset: u32,
    pub bytes: Vec<u8>,
}

/// Instruction builder for `Write`.
///
/// ### Accounts:
///
///   0. `[writable]` buffer_account
///   1. `[signer]` buffer_authority
#[derive(Clone, Debug, Default)]
pub struct WriteBuilder {
    buffer_account: Option<solana_program::pubkey::Pubkey>,
    buffer_authority: Option<solana_program::pubkey::Pubkey>,
    offset: Option<u32>,
    bytes: Option<Vec<u8>>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl WriteBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Buffer account.
    #[inline(always)]
    pub fn buffer_account(&mut self, buffer_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.buffer_account = Some(buffer_account);
        self
    }
    /// Buffer authority.
    #[inline(always)]
    pub fn buffer_authority(
        &mut self,
        buffer_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.buffer_authority = Some(buffer_authority);
        self
    }
    #[inline(always)]
    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);
        self
    }
    #[inline(always)]
    pub fn bytes(&mut self, bytes: Vec<u8>) -> &mut Self {
        self.bytes = Some(bytes);
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
        let accounts = Write {
            buffer_account: self.buffer_account.expect("buffer_account is not set"),
            buffer_authority: self.buffer_authority.expect("buffer_authority is not set"),
        };
        let args = WriteInstructionArgs {
            offset: self.offset.clone().expect("offset is not set"),
            bytes: self.bytes.clone().expect("bytes is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `write` CPI accounts.
pub struct WriteCpiAccounts<'a, 'b> {
    /// Buffer account.
    pub buffer_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Buffer authority.
    pub buffer_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `write` CPI instruction.
pub struct WriteCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Buffer account.
    pub buffer_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// Buffer authority.
    pub buffer_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: WriteInstructionArgs,
}

impl<'a, 'b> WriteCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: WriteCpiAccounts<'a, 'b>,
        args: WriteInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            buffer_account: accounts.buffer_account,
            buffer_authority: accounts.buffer_authority,
            __args: args,
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
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.buffer_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.buffer_authority.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&WriteInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::QED_DATA_LOADER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.buffer_account.clone());
        account_infos.push(self.buffer_authority.clone());
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

/// Instruction builder for `Write` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` buffer_account
///   1. `[signer]` buffer_authority
#[derive(Clone, Debug)]
pub struct WriteCpiBuilder<'a, 'b> {
    instruction: Box<WriteCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> WriteCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(WriteCpiBuilderInstruction {
            __program: program,
            buffer_account: None,
            buffer_authority: None,
            offset: None,
            bytes: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Buffer account.
    #[inline(always)]
    pub fn buffer_account(
        &mut self,
        buffer_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.buffer_account = Some(buffer_account);
        self
    }
    /// Buffer authority.
    #[inline(always)]
    pub fn buffer_authority(
        &mut self,
        buffer_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.buffer_authority = Some(buffer_authority);
        self
    }
    #[inline(always)]
    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.instruction.offset = Some(offset);
        self
    }
    #[inline(always)]
    pub fn bytes(&mut self, bytes: Vec<u8>) -> &mut Self {
        self.instruction.bytes = Some(bytes);
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
        let args = WriteInstructionArgs {
            offset: self.instruction.offset.clone().expect("offset is not set"),
            bytes: self.instruction.bytes.clone().expect("bytes is not set"),
        };
        let instruction = WriteCpi {
            __program: self.instruction.__program,

            buffer_account: self
                .instruction
                .buffer_account
                .expect("buffer_account is not set"),

            buffer_authority: self
                .instruction
                .buffer_authority
                .expect("buffer_authority is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct WriteCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    buffer_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    buffer_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    offset: Option<u32>,
    bytes: Option<Vec<u8>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
