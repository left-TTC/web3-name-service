
use {
    crate::{instruction::DEFAULT_VALUE, state::NameRecordHeader}, solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
    }, 
};


// only one case we will use the transfer function:
// when we buy a domain that owned by others
// so the function will only called when CPI
pub fn process_transfer(
    accounts: &[AccountInfo], 
    new_owner: Pubkey,
    new_custom_value: Option<u64>
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let name_account = next_account_info(accounts_iter)?;
    // This account must be registra program
    let instruction_caller = next_account_info(accounts_iter)?;
    let root_name_account = next_account_info(accounts_iter)?;

    if name_account.owner != &crate::ID || root_name_account.owner != &crate::ID {
        msg!("not the program pda");
        return Err(ProgramError::InvalidArgument)
    }

    let mut name_record_header =
        NameRecordHeader::unpack_from_slice(&name_account.data.borrow())?;
    if &name_record_header.parent_name != root_name_account.key {
        msg!("root domain account error");
        return Err(ProgramError::InvalidArgument);
    }
    
    let root_name_state = 
        NameRecordHeader::unpack_from_slice(&root_name_account.data.borrow())?;
    if instruction_caller.key != &root_name_state.owner || !instruction_caller.is_signer || root_name_state.class != Pubkey::default() || root_name_state.parent_name != Pubkey::default(){
        msg!("not the register central called the instruction");
        return Err(ProgramError::InvalidArgument);
    }

    if let Some(value) = new_custom_value {
        name_record_header.custom_price = value
    }else {
        name_record_header.custom_price = DEFAULT_VALUE
    }

    msg!("new value: {} dollar", name_record_header.custom_price / 1000000 );

    name_record_header.owner = new_owner;
    name_record_header
        .pack_into_slice(&mut name_account.data.borrow_mut()[..NameRecordHeader::LEN]);

    Ok(())
}