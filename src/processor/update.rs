
use {
    crate::{
        state::{write_data, NameRecordHeader}, 
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
    },
};


// can only update the reverse data
// the reverse owner can only be record central state and register central state
// so the update can only be called by CPI
pub fn process_update(
    accounts: &[AccountInfo], 
    data: Vec<u8>
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let name_account = next_account_info(accounts_iter)?;
    let name_update_signer = next_account_info(accounts_iter)?;
    let parent_name = next_account_info(accounts_iter).ok();

    let name_record_header = 
        NameRecordHeader::unpack_from_slice(&name_account.data.borrow())?;

    // Verifications: 
    let is_parent_owner = if let Some(parent_name) = parent_name {
        if name_record_header.parent_name != *parent_name.key {
            msg!("Invalid parent name account");
            return Err(ProgramError::InvalidArgument);
        }
        let parent_name_record_header =
            NameRecordHeader::unpack_from_slice(&parent_name.data.borrow())?;
        parent_name_record_header.owner == *name_update_signer.key
    } else {
        false
    };
    if !name_update_signer.is_signer {
        msg!("The given name class or owner is not a signer.");
        return Err(ProgramError::InvalidArgument);
    }
    if name_record_header.class != Pubkey::default()
        && *name_update_signer.key != name_record_header.class
    {
        msg!("The given name class account is incorrect.");
        return Err(ProgramError::InvalidArgument);
    }
    if name_record_header.class == Pubkey::default()
        && *name_update_signer.key != name_record_header.owner
        && !is_parent_owner
    {
        msg!("The given name owner account is incorrect.");
        return Err(ProgramError::InvalidArgument);
    }

    write_data(
        name_account,
        &data,
        NameRecordHeader::LEN,
    );

    Ok(())
}