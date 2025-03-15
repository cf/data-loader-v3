//! Program processor.

use {
    crate::{
        instruction::QEDDataLoaderInstruction,
        state::UpgradeableLoaderState,
    }, borsh::BorshDeserialize, solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
        sysvar::Sysvar,
    }
};
/*
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
*/
/// Processes an
/// [InitializeBuffer](enum.QEDDataLoaderInstruction.html)
/// instruction.
fn process_initialize_buffer(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let source_info = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;

    // Ensure the buffer has not already been initialized.
    {
        let buffer_data = source_info.try_borrow_data()?;
        if UpgradeableLoaderState::Uninitialized
            != UpgradeableLoaderState::deserialize(&buffer_data)?
        {
            msg!("Buffer account already initialized");
            return Err(ProgramError::AccountAlreadyInitialized);
        }
    }

    let mut buffer_data = source_info.try_borrow_mut_data()?;
    bincode::serialize_into(
        &mut buffer_data[..],
        &UpgradeableLoaderState::Buffer {
            authority_address: Some(*authority_info.key),
        },
    )
    .map_err(|_| ProgramError::InvalidAccountData)?;

    Ok(())
}

/// Processes a
/// [Write](enum.QEDDataLoaderInstruction.html)
/// instruction.
fn process_write(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    offset: u32,
    bytes: Vec<u8>,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let buffer_info = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;

    // Run checks on the authority.
    {
        let buffer_data = buffer_info.try_borrow_data()?;
        if let UpgradeableLoaderState::Buffer { authority_address } =
            UpgradeableLoaderState::deserialize(&buffer_data)?
        {
            if authority_address.is_none() {
                msg!("Buffer is immutable");
                return Err(ProgramError::Immutable);
            }
            if authority_address != Some(*authority_info.key) {
                msg!("Incorrect buffer authority provided");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !authority_info.is_signer {
                msg!("Buffer authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
        } else {
            msg!("Invalid Buffer account");
            return Err(ProgramError::InvalidAccountData);
        }
    }

    // Ensure the buffer account is large enough.
    let programdata_offset =
        UpgradeableLoaderState::size_of_buffer_metadata().saturating_add(offset as usize);
    let end_offset = programdata_offset.saturating_add(bytes.len());
    if buffer_info.data_len() < end_offset {
        msg!(
            "Write overflow: {} < {}",
            buffer_info.data_len(),
            end_offset
        );
        return Err(ProgramError::AccountDataTooSmall);
    }

    // Write the data.
    buffer_info
        .try_borrow_mut_data()?
        .get_mut(programdata_offset..end_offset)
        .ok_or(ProgramError::AccountDataTooSmall)?
        .copy_from_slice(&bytes);

    Ok(())
}


/// Processes a
/// [SetAuthority](enum.QEDDataLoaderInstruction.html)
/// instruction.
fn process_set_authority(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let buffer_or_programdata_info = next_account_info(accounts_iter)?;
    let current_authority_info = next_account_info(accounts_iter)?;
    let new_authority_info = next_account_info(accounts_iter).ok();

    // Need this to avoid a double-borrow on account data.
    enum AuthorityType {
        Buffer,
        ProgramData { slot: u64 },
    }

    let authority_type = match UpgradeableLoaderState::deserialize(
        &buffer_or_programdata_info.try_borrow_data()?,
    )? {
        UpgradeableLoaderState::Buffer { authority_address } => {
            if new_authority_info.is_none() {
                msg!("Buffer authority is not optional");
                return Err(ProgramError::IncorrectAuthority);
            }
            if authority_address.is_none() {
                msg!("Buffer is immutable");
                return Err(ProgramError::Immutable);
            }
            if authority_address != Some(*current_authority_info.key) {
                msg!("Incorrect buffer authority provided");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !current_authority_info.is_signer {
                msg!("Buffer authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
            AuthorityType::Buffer
        }
        UpgradeableLoaderState::ProgramData {
            upgrade_authority_address,
            slot,
        } => {
            if upgrade_authority_address.is_none() {
                msg!("ProgramData is not upgradeable");
                return Err(ProgramError::Immutable);
            }
            if upgrade_authority_address != Some(*current_authority_info.key) {
                msg!("Incorrect upgrade authority provided");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !current_authority_info.is_signer {
                msg!("Upgrade authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
            AuthorityType::ProgramData { slot }
        }
        _ => {
            msg!("Account does not support authorities");
            return Err(ProgramError::InvalidArgument);
        }
    };

    match authority_type {
        AuthorityType::Buffer => {
            // This looks silly, but `bincode::serialize_into` will serialize 4
            // bytes for `None` and then stop, leaving the rest of the buffer
            // untouched.
            let mut buffer_data = buffer_or_programdata_info.try_borrow_mut_data()?;
            let buffer_metadata_offset = UpgradeableLoaderState::size_of_buffer_metadata();
            buffer_data
                .get_mut(..buffer_metadata_offset)
                .ok_or(ProgramError::AccountDataTooSmall)?
                .fill(0);
            bincode::serialize_into(
                &mut buffer_data[..],
                &UpgradeableLoaderState::Buffer {
                    authority_address: new_authority_info.map(|info| *info.key),
                },
            )
            .map_err(|_| ProgramError::InvalidAccountData)?;
        }
        AuthorityType::ProgramData { slot } => {
            // This looks silly, but `bincode::serialize_into` will serialize 4
            // bytes for `None` and then stop, leaving the rest of the buffer
            // untouched.
            let mut programdata_data = buffer_or_programdata_info.try_borrow_mut_data()?;
            let programdata_metadata_offset =
                UpgradeableLoaderState::size_of_programdata_metadata();
            programdata_data
                .get_mut(..programdata_metadata_offset)
                .ok_or(ProgramError::AccountDataTooSmall)?
                .fill(0);
            bincode::serialize_into(
                &mut programdata_data[..],
                &UpgradeableLoaderState::ProgramData {
                    upgrade_authority_address: new_authority_info.map(|info| *info.key),
                    slot,
                },
            )
            .map_err(|_| ProgramError::InvalidAccountData)?;
        }
    }

    msg!(
        "New authority: {:?}",
        new_authority_info.map(|info| info.key)
    );

    Ok(())
}

/// Processes a
/// [Close](enum.QEDDataLoaderInstruction.html)
/// instruction.
fn process_close(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let buffer_or_programdata_info = next_account_info(accounts_iter)?;
    let destination_info = next_account_info(accounts_iter)?;

    if buffer_or_programdata_info.key == destination_info.key {
        msg!("Recipient is the same as the account being closed");
        return Err(ProgramError::InvalidArgument);
    }

    // Need this to avoid a double-borrow on account data.
    // I guess it's only for a log message at this point?
    enum AuthorityType {
        Uninitialized,
        Buffer,
        ProgramData,
    }

    let authority_type = match UpgradeableLoaderState::deserialize(
        &buffer_or_programdata_info.try_borrow_data()?,
    )? {
        UpgradeableLoaderState::Uninitialized => AuthorityType::Uninitialized,
        UpgradeableLoaderState::Buffer { authority_address } => {
            let authority_info = next_account_info(accounts_iter)?;
            if authority_address.is_none() {
                msg!("Account is immutable");
                return Err(ProgramError::Immutable);
            }
            if authority_address != Some(*authority_info.key) {
                msg!("Incorrect authority provided");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !authority_info.is_signer {
                msg!("Authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }

            AuthorityType::Buffer
        }
        UpgradeableLoaderState::ProgramData {
            slot,
            upgrade_authority_address,
        } => {
            let authority_info = next_account_info(accounts_iter)?;
            let program_info = next_account_info(accounts_iter)?;

            if !program_info.is_writable {
                msg!("Program account is not writable");
                return Err(ProgramError::InvalidArgument);
            }
            if program_info.owner != program_id {
                msg!("Program account not owned by loader");
                return Err(ProgramError::IncorrectProgramId);
            }

            let clock = <Clock as Sysvar>::get()?;
            if clock.slot == slot {
                msg!("Program was deployed in this block already");
                return Err(ProgramError::InvalidArgument);
            }

            match UpgradeableLoaderState::deserialize(&program_info.try_borrow_data()?)? {
                UpgradeableLoaderState::Program {
                    programdata_address,
                } => {
                    if programdata_address != *buffer_or_programdata_info.key {
                        msg!("ProgramData account does not match ProgramData account");
                        return Err(ProgramError::InvalidArgument);
                    }

                    if upgrade_authority_address.is_none() {
                        msg!("Account is immutable");
                        return Err(ProgramError::Immutable);
                    }
                    if upgrade_authority_address != Some(*authority_info.key) {
                        msg!("Incorrect authority provided");
                        return Err(ProgramError::IncorrectAuthority);
                    }
                    if !authority_info.is_signer {
                        msg!("Authority did not sign");
                        return Err(ProgramError::MissingRequiredSignature);
                    }
                }
                _ => {
                    msg!("Invalid Program account");
                    return Err(ProgramError::InvalidArgument);
                }
            }

            AuthorityType::ProgramData
        }
        _ => {
            msg!("Account does not support closing");
            return Err(ProgramError::InvalidArgument);
        }
    };

    {
        let new_destination_lamports = destination_info
            .lamports()
            .checked_add(buffer_or_programdata_info.lamports())
            .ok_or::<ProgramError>(ProgramError::ArithmeticOverflow)?;

        **buffer_or_programdata_info.try_borrow_mut_lamports()? = 0;
        **destination_info.try_borrow_mut_lamports()? = new_destination_lamports;
    }

    buffer_or_programdata_info.realloc(UpgradeableLoaderState::size_of_uninitialized(), true)?;

    let mut buffer_or_programdata_data = buffer_or_programdata_info.try_borrow_mut_data()?;
    bincode::serialize_into(
        &mut buffer_or_programdata_data[..],
        &UpgradeableLoaderState::Uninitialized,
    )
    .map_err(|_| ProgramError::InvalidAccountData)?;

    // [CORE BPF]: Store modified entry in program cache.

    match authority_type {
        AuthorityType::Uninitialized => {
            msg!("Closed Uninitialized {}", buffer_or_programdata_info.key);
        }
        AuthorityType::Buffer => {
            msg!("Closed Buffer {}", buffer_or_programdata_info.key);
        }
        AuthorityType::ProgramData => {
            msg!("Closed Program {}", buffer_or_programdata_info.key);
        }
    }

    Ok(())
}


/// Processes a
/// [SetAuthorityChecked](enum.QEDDataLoaderInstruction.html)
/// instruction.
fn process_set_authority_checked(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let buffer_or_programdata_info = next_account_info(accounts_iter)?;
    let current_authority_info = next_account_info(accounts_iter)?;
    let new_authority_info = next_account_info(accounts_iter)?;

    // Need this to avoid a double-borrow on account data.
    enum AuthorityType {
        Buffer,
        ProgramData { slot: u64 },
    }

    let authority_type = match UpgradeableLoaderState::deserialize(
        &buffer_or_programdata_info.try_borrow_data()?,
    )? {
        UpgradeableLoaderState::Buffer { authority_address } => {
            if authority_address.is_none() {
                msg!("Buffer is immutable");
                return Err(ProgramError::Immutable);
            }
            if authority_address != Some(*current_authority_info.key) {
                msg!("Incorrect buffer authority provided");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !current_authority_info.is_signer {
                msg!("Buffer authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
            if !new_authority_info.is_signer {
                msg!("New authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
            AuthorityType::Buffer
        }
        UpgradeableLoaderState::ProgramData {
            upgrade_authority_address,
            slot,
        } => {
            if upgrade_authority_address.is_none() {
                msg!("Program not upgradeable");
                return Err(ProgramError::Immutable);
            }
            if upgrade_authority_address != Some(*current_authority_info.key) {
                msg!("Incorrect upgrade authority provided");
                return Err(ProgramError::IncorrectAuthority);
            }
            if !current_authority_info.is_signer {
                msg!("Upgrade authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
            if !new_authority_info.is_signer {
                msg!("New authority did not sign");
                return Err(ProgramError::MissingRequiredSignature);
            }
            AuthorityType::ProgramData { slot }
        }
        _ => {
            msg!("Account does not support authorities");
            return Err(ProgramError::InvalidArgument);
        }
    };

    match authority_type {
        AuthorityType::Buffer => {
            // No need to zero anything, since this instruction requires the
            // new authority to sign (cannot be `None`).
            let mut buffer_data = buffer_or_programdata_info.try_borrow_mut_data()?;
            bincode::serialize_into(
                &mut buffer_data[..],
                &UpgradeableLoaderState::Buffer {
                    authority_address: Some(*new_authority_info.key),
                },
            )
            .map_err(|_| ProgramError::InvalidAccountData)?;
        }
        AuthorityType::ProgramData { slot } => {
            // No need to zero anything, since this instruction requires the
            // new authority to sign (cannot be `None`).
            let mut programdata_data = buffer_or_programdata_info.try_borrow_mut_data()?;
            bincode::serialize_into(
                &mut programdata_data[..],
                &UpgradeableLoaderState::ProgramData {
                    upgrade_authority_address: Some(*new_authority_info.key),
                    slot,
                },
            )
            .map_err(|_| ProgramError::InvalidAccountData)?;
        }
    }

    msg!("New authority: {:?}", new_authority_info.key);

    Ok(())
}

/// Processes a
/// [QEDDataLoaderInstruction](enum.QEDDataLoaderInstruction.html).
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    match QEDDataLoaderInstruction::try_from_slice(input)? {
        QEDDataLoaderInstruction::InitializeBuffer => {
            msg!("Instruction: InitializeBuffer");
            process_initialize_buffer(program_id, accounts)
        }
        QEDDataLoaderInstruction::Write { offset, bytes } => {
            msg!("Instruction: Write");
            process_write(program_id, accounts, offset, bytes)
        }
        QEDDataLoaderInstruction::SetAuthority => {
            msg!("Instruction: SetAuthority");
            process_set_authority(program_id, accounts)
        }
        QEDDataLoaderInstruction::Close => {
            msg!("Instruction: Close");
            process_close(program_id, accounts)
        }
        QEDDataLoaderInstruction::SetAuthorityChecked => {
            msg!("Instruction: SetAuthorityChecked");
            process_set_authority_checked(program_id, accounts)
        }
    }
}
