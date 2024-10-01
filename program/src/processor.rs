//! Program processor.

use {
    crate::instruction::LoaderV3Instruction,
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
        pubkey::Pubkey,
    },
};

// [Core BPF]: Locally-implemented
// `solana_sdk::program_utils::limited_deserialize`.
fn limited_deserialize<T>(input: &[u8]) -> Result<T, ProgramError>
where
    T: serde::de::DeserializeOwned,
{
    solana_program::program_utils::limited_deserialize(
        input, 1232, // [Core BPF]: See `solana_sdk::packet::PACKET_DATA_SIZE`
    )
    .map_err(|_| ProgramError::InvalidInstructionData)
}

/// Processes an
/// [InitializeBuffer](enum.LoaderV3Instruction.html)
/// instruction.
fn process_initialize_buffer(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [Write](enum.LoaderV3Instruction.html)
/// instruction.
fn process_write(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _offset: u32,
    _bytes: Vec<u8>,
) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [DeployWithMaxDataLen](enum.LoaderV3Instruction.html)
/// instruction.
fn process_deploy_with_max_data_len(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _max_data_len: usize,
) -> ProgramResult {
    Ok(())
}

/// Processes an
/// [Upgrade](enum.LoaderV3Instruction.html)
/// instruction.
fn process_upgrade(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [SetAuthority](enum.LoaderV3Instruction.html)
/// instruction.
fn process_set_authority(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [Close](enum.LoaderV3Instruction.html)
/// instruction.
fn process_close(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes an
/// [ExtendProgram](enum.LoaderV3Instruction.html)
/// instruction.
fn process_extend_program(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _additional_bytes: u32,
) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [SetAuthorityChecked](enum.LoaderV3Instruction.html)
/// instruction.
fn process_set_authority_checked(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [LoaderV3Instruction](enum.LoaderV3Instruction.html).
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    match limited_deserialize(input)? {
        LoaderV3Instruction::InitializeBuffer => {
            msg!("Instruction: InitializeBuffer");
            process_initialize_buffer(program_id, accounts)
        }
        LoaderV3Instruction::Write { offset, bytes } => {
            msg!("Instruction: Write");
            process_write(program_id, accounts, offset, bytes)
        }
        LoaderV3Instruction::DeployWithMaxDataLen { max_data_len } => {
            msg!("Instruction: DeployWithMaxDataLen");
            process_deploy_with_max_data_len(program_id, accounts, max_data_len)
        }
        LoaderV3Instruction::Upgrade => {
            msg!("Instruction: Upgrade");
            process_upgrade(program_id, accounts)
        }
        LoaderV3Instruction::SetAuthority => {
            msg!("Instruction: SetAuthority");
            process_set_authority(program_id, accounts)
        }
        LoaderV3Instruction::Close => {
            msg!("Instruction: Close");
            process_close(program_id, accounts)
        }
        LoaderV3Instruction::ExtendProgram { additional_bytes } => {
            msg!("Instruction: ExtendProgram");
            process_extend_program(program_id, accounts, additional_bytes)
        }
        LoaderV3Instruction::SetAuthorityChecked => {
            msg!("Instruction: SetAuthorityChecked");
            process_set_authority_checked(program_id, accounts)
        }
    }
}
