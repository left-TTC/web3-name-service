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
            } => {
                msg!("Instruction: Create");
                create::process_create(
                    program_id, accounts, hashed_name, lamports, space
                )?;
            }
            NameRegistryInstruction::Update { 
                data 
            } => {
                msg!("Instruction: Update Data");
                update::process_update(accounts, data)?;
            }
            NameRegistryInstruction::Transfer { 
                new_owner 
            } => {
                msg!("Instruction: Transfer Ownership");
                transfer::process_transfer(accounts, new_owner)?;
            }
        }
        Ok(())
    }








    



    

    
}
