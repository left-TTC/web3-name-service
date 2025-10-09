use {
    crate::{
        state::NameRecordHeader,
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo}, 
        entrypoint::ProgramResult, msg, program::invoke, 
        program_error::ProgramError, program_pack::Pack, rent::Rent, sysvar::Sysvar
    }, 
    std::cmp::Ordering,
};

use solana_system_interface::instruction as system_instruction;


// In my design, this instruction will only come from record
pub fn process_realloc(
    accounts: &[AccountInfo],
    space: u32,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let system_program = next_account_info(accounts_iter)?;
    let payer_account = next_account_info(accounts_iter)?;
    // record account
    let name_account = next_account_info(accounts_iter)?;
    // record central
    let name_owner = next_account_info(accounts_iter)?;

    let name_record_header = 
        NameRecordHeader::unpack_from_slice(&name_account.data.borrow())?;

    if !name_owner.is_signer || name_record_header.owner != *name_owner.key {
        msg!("The given name owner is incorrect or not a signer.");
        return Err(ProgramError::InvalidArgument);
    }

    // currently can only be the cid length
    let new_space = NameRecordHeader::LEN.saturating_add(space as usize);
    let required_lamports = Rent::get()?.minimum_balance(new_space);

    let now_lamports = name_account.lamports();

    match now_lamports.cmp(&required_lamports) {
        Ordering::Less => {
            // the only correspond condition
            #[allow(clippy::arithmetic_side_effects)]
            let lamports_to_add = required_lamports.saturating_sub(now_lamports);
            invoke(
                &system_instruction::transfer(
                    payer_account.key,
                    name_account.key,
                    lamports_to_add,
                ),
                &[
                    payer_account.clone(),
                    name_account.clone(),
                    system_program.clone(),
                ],
            )?;
        }
        Ordering::Equal => {}
        Ordering::Greater => {}
    }

    name_account.realloc(new_space, false)?;
    Ok(())
}