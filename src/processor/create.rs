
use {
    crate::{
        instruction::{DEFAULT_VALUE},
        state::{NameRecordHeader}, utils::get_seeds_and_key,
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
        system_instruction,
    },
};


pub fn process_create(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    hashed_name: Vec<u8>,
    lamports: u64,
    space: u32,
    custom_value: Option<u64>,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let system_program = next_account_info(accounts_iter)?;
    let payer_account = next_account_info(accounts_iter)?;
    let name_account = next_account_info(accounts_iter)?;
    let name_owner = next_account_info(accounts_iter)?;
    let name_class = next_account_info(accounts_iter)?;
    let parent_name_account = next_account_info(accounts_iter)?;
    let parent_name_owner = next_account_info(accounts_iter).ok();

    let (name_account_key, seeds) = get_seeds_and_key(
        program_id,
        hashed_name,
        Some(name_class.key),
        Some(parent_name_account.key),
    );
    if name_account_key != *name_account.key {
        msg!("The given name account is incorrect.");
        return Err(ProgramError::InvalidArgument);
    }

    if name_account.data.borrow().len() > 0 {
        let name_record_header =
            NameRecordHeader::unpack_from_slice(&name_account.data.borrow())?;
        if name_record_header.owner != Pubkey::default() {
            msg!("The given name account already exists.");
            return Err(ProgramError::InvalidArgument);
        }
    }

    if *name_class.key != Pubkey::default() && !name_class.is_signer {
        msg!("The given name class is not a signer.");
        return Err(ProgramError::InvalidArgument); 
    }

    // not the root domain
    if *parent_name_account.key != Pubkey::default() {
        if !parent_name_owner.unwrap().is_signer {
            msg!("The given parent name account owner is not a signer.");
            return Err(ProgramError::InvalidArgument);
        } else {
            let parent_name_record_header =
                NameRecordHeader::unpack_from_slice(&parent_name_account.data.borrow())?;
            if &parent_name_record_header.owner != parent_name_owner.unwrap().key {
                msg!("The given parent name account owner is not correct.");
                return Err(ProgramError::InvalidArgument);
            }
        }
    }
    
    if name_owner.key == &Pubkey::default() {
        msg!("The owner cannot be `Pubkey::default()`.");
        return Err(ProgramError::InvalidArgument);
    }

    if name_account.data.borrow().len() == 0 {
            invoke(
                &system_instruction::transfer(payer_account.key, &name_account_key, lamports),
                &[
                    payer_account.clone(),
                    name_account.clone(),
                    system_program.clone(),
                ],
            )?;

            invoke_signed(
                &system_instruction::allocate(
                    &name_account_key,
                    NameRecordHeader::LEN.saturating_add(space as usize) as u64,
                ),
                &[name_account.clone(), system_program.clone()],
                &[&seeds.chunks(32).collect::<Vec<&[u8]>>()],
            )?;

            invoke_signed(
                &system_instruction::assign(name_account.key, program_id),
                &[name_account.clone(), system_program.clone()],
                &[&seeds.chunks(32).collect::<Vec<&[u8]>>()],
            )?;
    }

    let name_state = NameRecordHeader {
        parent_name: *parent_name_account.key,
        owner: *name_owner.key,
        class: *name_class.key,
        custom_price: custom_value.unwrap_or_else(|| DEFAULT_VALUE),
    };

    name_state.pack_into_slice(&mut name_account.data.borrow_mut());

    Ok(())
}