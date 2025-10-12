
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

// the case we will use the update function:
// case 1: create domain, we will use it to add the reverse name
// case 2: revise custom price 


// NameRecordHeader::LEN - 8 means the owner can only change the customPrice
pub fn process_update(accounts: &[AccountInfo], offset: u32, data: Vec<u8>) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let name_account = next_account_info(accounts_iter)?;
    // always be two central states or the owner
    let name_update_signer = next_account_info(accounts_iter)?;

    let name_record_header = NameRecordHeader::unpack_from_slice(&name_account.data.borrow())?;

    if !name_update_signer.is_signer {
        msg!("The given name class or owner is not a signer.");
        return Err(ProgramError::InvalidArgument);
    }

    // means reverse account
    // only the 
    if name_record_header.class != Pubkey::default()                
        && *name_update_signer.key != name_record_header.class {    
        msg!("The given name class account is incorrect.");
        return Err(ProgramError::InvalidArgument);
    }

    // means common domain key
    // only the domain owner can update the custom price
    if name_record_header.class == Pubkey::default()
        && *name_update_signer.key != name_record_header.owner{
        msg!("The given name owner account is incorrect.");
        return Err(ProgramError::InvalidArgument);
    }

    write_data(
        name_account,
        &data,
        // only two cases:
        // 1. common domain: offset = 0 and usr will revise the custom price
        // 2. reverse domain: owner is cenatral state => offset = 8 and only update the domain reverse name
        (NameRecordHeader::LEN - 8).saturating_add(offset as usize),
    );

    Ok(())
}