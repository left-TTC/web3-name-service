
use {
    crate::{
        state::NameRecordHeader,
    },
    solana_program::{
        account_info::{AccountInfo, next_account_info},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
    },
};


pub fn process_change_preview(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    previewer: Pubkey,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let name_account = next_account_info(accounts_iter)?;
    let root_name_account = next_account_info(accounts_iter)?;
    let root_owner = next_account_info(accounts_iter)?;

    if name_account.owner != &crate::ID || root_name_account.owner != &crate::ID {
        msg!("not the program pda");
        return Err(ProgramError::InvalidArgument)
    }

    let mut name_record_header =
        NameRecordHeader::unpack_from_slice(&name_account.data.borrow())?;
    if name_record_header.previewer != Pubkey::default() || name_record_header.previewer == previewer {
        msg!("fault previewer or already changed");
        return Err(ProgramError::InvalidArgument);
    }

    let root_name_state = 
        NameRecordHeader::unpack_from_slice(&root_name_account.data.borrow())?;
    if root_owner.key != &root_name_state.owner || !root_owner.is_signer || root_name_state.class != Pubkey::default() || root_name_state.parent_name != Pubkey::default(){
        msg!("not the register central called the instruction");
        return Err(ProgramError::InvalidArgument);
    }

    name_record_header.previewer = previewer;
    name_record_header
        .pack_into_slice(&mut name_account.data.borrow_mut()[..NameRecordHeader::LEN]);

    Ok(())
}