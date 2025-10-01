
use {
    crate::state::NameRecordHeader, solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
    }
};


// same as update, can only be called by CPI
pub fn process_transfer(
    accounts: &[AccountInfo], 
    new_owner: Pubkey
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let name_account = next_account_info(accounts_iter)?;
    let name_owner = next_account_info(accounts_iter)?;
    let name_class_opt = next_account_info(accounts_iter).ok();
    let parent_name = next_account_info(accounts_iter).ok();

    let mut name_record_header =
        NameRecordHeader::unpack_from_slice(&name_account.data.borrow())?;

    // Verifications
    let is_parent_owner = if let Some(parent_name) = parent_name {
        if name_record_header.parent_name != *parent_name.key {
            msg!("Invalid parent name account");
            return Err(ProgramError::InvalidArgument);
        }
        let parent_name_record_header =
            NameRecordHeader::unpack_from_slice(&parent_name.data.borrow())?;
        parent_name_record_header.owner == *name_owner.key
    } else {
        false
    };

    if !name_owner.is_signer
        || (name_record_header.owner != *name_owner.key && !is_parent_owner)
    {
        msg!("The given name owner is incorrect or not a signer.");
        return Err(ProgramError::InvalidArgument);
    }
    if name_record_header.class != Pubkey::default()
        && (name_class_opt.is_none()
            || name_record_header.class != *name_class_opt.unwrap().key
            || !name_class_opt.unwrap().is_signer)
    {
        msg!("The given name class account is incorrect or not a signer.");
        return Err(ProgramError::InvalidArgument);
    }

    name_record_header.owner = new_owner;
    name_record_header
        .pack_into_slice(&mut name_account.data.borrow_mut()[..NameRecordHeader::LEN]);

    Ok(())
}