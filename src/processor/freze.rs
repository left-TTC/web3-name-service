use {
    crate::state::{NameRecordHeader},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        msg,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
        entrypoint::ProgramResult,
    },
};

pub fn process_freeze_account(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let name_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;

    // Verify owner is signer
    if !owner_account.is_signer {
        msg!("Owner account must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Unpack the name record header
    let mut name_record = NameRecordHeader::unpack(&name_account.data.borrow())?;

    // Verify the account owner matches
    if name_record.owner != *owner_account.key {
        msg!("Owner mismatch");
        return Err(ProgramError::InvalidArgument);
    }

    // Check if already frozen
    if name_record.is_frozen {
        msg!("Account is already frozen");
        return Err(ProgramError::InvalidAccountData);
    }

    // Set frozen status
    name_record.is_frozen = true;
    msg!("Account frozen successfully");

    // Pack updated record back into account
    NameRecordHeader::pack(name_record, &mut name_account.data.borrow_mut())?;

    Ok(())
}
