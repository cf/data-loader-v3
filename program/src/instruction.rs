//! Program instruction types.

use {
    borsh::{BorshDeserialize, BorshSerialize}, serde::{Deserialize, Serialize}, shank::ShankInstruction, solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    }
};

/// Instructions supported by the Solana BPF Loader v3 program.
#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ShankInstruction, BorshSerialize, BorshDeserialize)]
pub enum QEDDataLoaderInstruction {
    /// Initialize a Buffer account.
    ///
    /// A Buffer account is an intermediary that once fully populated is used
    /// with the `DeployWithMaxDataLen` instruction to populate the program's
    /// ProgramData account.
    ///
    /// The `InitializeBuffer` instruction requires no signers and MUST be
    /// included within the same Transaction as the system program's
    /// `CreateAccount` instruction that creates the account being initialized.
    /// Otherwise another party may initialize the account.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Source account to initialize.
    /// 1. `[ ]` Buffer authority.
    #[account(
        0,
        writable,
        name = "source_account",
        desc = "Source account to initialize."
    )]
    #[account(
        1,
        name = "buffer_authority",
        desc = "Buffer authority."
    )]
    InitializeBuffer,

    /// Write program data into a Buffer account.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Buffer account.
    /// 1. `[s]` Buffer authority.
    #[account(
        0,
        writable,
        name = "buffer_account",
        desc = "Buffer account."
    )]
    #[account(
        1,
        signer,
        name = "buffer_authority",
        desc = "Buffer authority."
    )]
    Write {
        /// Offset at which to write the given bytes.
        offset: u32,
        /// Serialized program data.
        bytes: Vec<u8>,
    },

    /// Set a new authority that is allowed to write the buffer or upgrade the
    /// program. To permanently make the buffer immutable or disable program
    /// updates, omit the new authority.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Buffer or ProgramData account.
    /// 1. `[s]` Current authority.
    /// 2. `[ ]` New authority (optional).
    #[account(
        0,
        writable,
        name = "buffer_or_program_data_account",
        desc = "Buffer or ProgramData account."
    )]
    #[account(
        1,
        signer,
        name = "current_authority",
        desc = "Current authority."
    )]
    #[account(
        2,
        optional,
        name = "new_authority",
        desc = "New authority (optional)."
    )]
    SetAuthority,

    /// Closes an account owned by the upgradeable loader of all lamports and
    /// withdraws all the lamports.
    ///
    /// Note: The authority is only required to close an initialized account.
    /// Note: The program account is required to close an initialized
    /// ProgramData account.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Buffer or ProgramData account to close.
    /// 1. `[w]` Destination account for reclaimed lamports.
    /// 2. `[s]` Authority (optional).
    /// 3. `[w]` Program account (optional).
    #[account(
        0,
        writable,
        name = "buffer_or_program_data_account",
        desc = "Buffer or ProgramData account to close."
    )]
    #[account(
        1,
        writable,
        name = "destination_account",
        desc = "Destination account for reclaimed lamports."
    )]
    #[account(
        2,
        optional,
        signer,
        name = "authority",
        desc = "Authority (optional)."
    )]
    #[account(
        3,
        optional,
        name = "program_account",
        desc = "Program account (optional)."
    )]
    Close,

    /// Set a new authority that is allowed to write the buffer or upgrade the
    /// program.
    ///
    /// This instruction differs from SetAuthority in that the new authority is
    /// a required signer.
    ///
    /// Accounts expected by this instruction:
    /// 0. `[w]` Buffer or ProgramData account to change the authority of.
    /// 1. `[s]` Current authority.
    /// 2. `[s]` New authority.
    #[account(
        0,
        writable,
        name = "buffer_or_program_data_account",
        desc = "Buffer or ProgramData account to change the authority of."
    )]
    #[account(
        1,
        signer,
        name = "current_authority",
        desc = "Current authority."
    )]
    #[account(
        2,
        signer,
        name = "new_authority",
        desc = "New authority."
    )]
    SetAuthorityChecked,
}

/// Creates an
/// [InitializeBuffer](enum.QEDDataLoaderInstruction.html)
/// instruction.
pub fn initialize_buffer(source_address: &Pubkey, authority_address: &Pubkey) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*source_address, false),
        AccountMeta::new(*authority_address, false),
    ];
    Instruction::new_with_borsh(
        crate::id(),
        &QEDDataLoaderInstruction::InitializeBuffer,
        accounts,
    )
}

/// Creates a
/// [Write](enum.QEDDataLoaderInstruction.html)
/// instruction.
pub fn write(
    buffer_address: &Pubkey,
    authority_address: &Pubkey,
    offset: u32,
    bytes: Vec<u8>,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*buffer_address, false),
        AccountMeta::new_readonly(*authority_address, true),
    ];
    Instruction::new_with_borsh(
        crate::id(),
        &QEDDataLoaderInstruction::Write { offset, bytes },
        accounts,
    )
}

/// Creates a
/// [SetAuthority](enum.QEDDataLoaderInstruction.html)
/// instruction.
pub fn set_authority(
    buffer_or_program_data_address: &Pubkey,
    current_authority_address: &Pubkey,
    new_authority_address: Option<&Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(*buffer_or_program_data_address, false),
        AccountMeta::new_readonly(*current_authority_address, true),
    ];
    if let Some(new_authority_address) = new_authority_address {
        accounts.push(AccountMeta::new_readonly(*new_authority_address, false));
    }
    Instruction::new_with_borsh(crate::id(), &QEDDataLoaderInstruction::SetAuthority, accounts)
}

/// Creates a
/// [Close](enum.QEDDataLoaderInstruction.html)
/// instruction.
pub fn close(
    buffer_or_program_data_address: &Pubkey,
    destination_address: &Pubkey,
    authority_address: Option<&Pubkey>,
    program_address: Option<&Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(*buffer_or_program_data_address, false),
        AccountMeta::new(*destination_address, false),
    ];
    if let Some(authority_address) = authority_address {
        accounts.push(AccountMeta::new_readonly(*authority_address, true));
    }
    if let Some(program_address) = program_address {
        accounts.push(AccountMeta::new(*program_address, false));
    }
    Instruction::new_with_borsh(crate::id(), &QEDDataLoaderInstruction::Close, accounts)
}

/// Creates a
/// [SetAuthorityChecked](enum.QEDDataLoaderInstruction.html)
/// instruction.
pub fn set_authority_checked(
    buffer_or_program_data_address: &Pubkey,
    current_authority_address: &Pubkey,
    new_authority_address: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*buffer_or_program_data_address, false),
        AccountMeta::new_readonly(*current_authority_address, true),
        AccountMeta::new_readonly(*new_authority_address, true),
    ];
    Instruction::new_with_borsh(
        crate::id(),
        &QEDDataLoaderInstruction::SetAuthorityChecked,
        accounts,
    )
}
