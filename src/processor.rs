use {
    crate::{
        instruction::NameRegistryInstruction,
    },
    borsh::BorshDeserialize,
    solana_program::{
        account_info::{ AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

pub mod create;
pub mod update;
pub mod transfer;
pub mod realloc;
pub mod freeze;
pub mod preview;
pub struct Processor {}

impl Processor {

    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Beginning processing");
        msg!("instruction: {:?}", instruction_data);
        let instruction = NameRegistryInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        msg!("Instruction unpacked");

        match instruction {
            NameRegistryInstruction::Create {
                hashed_name,
                lamports,
                space,
                custom_value
            } => {
                msg!("Instruction: Create");
                create::process_create(
                    program_id, accounts, hashed_name, lamports, space, custom_value
                )?;
            }
            NameRegistryInstruction::Update { 
                offset, 
                data 
            } => {
                msg!("Instruction: Update Data");
                update::process_update(accounts, offset, data)?;
            }
            NameRegistryInstruction::Transfer { 
                new_owner, custom_value 
            } => {
                msg!("Instruction: Transfer Ownership");
                transfer::process_transfer(accounts, new_owner, custom_value)?;
            }
            NameRegistryInstruction::Realloc { space } => {
                msg!("Instruction: Realloc Name Record");
                realloc::process_realloc(accounts, space)?;
            }
            NameRegistryInstruction::FreezeAccount => {
                msg!("Instruction: Freeze Account");
                freeze::process_freeze_account(program_id, accounts)?;
            }
            NameRegistryInstruction::ChangePreview { new_preview } => {
                msg!("Instruction: Set new Preview");
                preview::process_change_preview(program_id, accounts, new_preview)?;
            }
        }
        Ok(())
    }

    
}
