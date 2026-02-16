use{
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program,
    },
};

#[cfg(not(feature="devnet"))]
pub const DEFAULT_VALUE: u64  = 999999999999;
#[cfg(feature="devnet")]
pub const DEFAULT_VALUE: u64  = 99999999;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum NameRegistryInstruction {
    Create {
        hashed_name: Vec<u8>,
        lamports: u64,
        space: u32,
        custom_value: Option<u64>
    },

    Update { 
        offset: u32, 
        data: Vec<u8> 
    },

    Transfer { 
        new_owner: Pubkey,
        custom_value: Option<u64>, 
    },

    Realloc {
        // New total number of bytes in addition to the `NameRecordHeader`.
        // can only be larger than last time 
        space: u32,
    },

    FreezeAccount,
}

#[allow(clippy::too_many_arguments)]
pub fn create(
    name_service_program_id: Pubkey,
    instruction_data: NameRegistryInstruction,
    name_account_key: Pubkey,
    payer_key: Pubkey,
    name_owner: Pubkey,
    name_class_opt: Option<Pubkey>,
    name_parent_opt: Option<Pubkey>,
    name_parent_owner_opt: Option<Pubkey>,
) -> Result<Instruction, ProgramError> {
    let data = borsh::to_vec(&instruction_data).unwrap();
    let mut accounts = vec![
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new(payer_key, true),
        AccountMeta::new(name_account_key, false),
        AccountMeta::new_readonly(name_owner, false),
    ];
    if let Some(name_class) = name_class_opt {
        accounts.push(AccountMeta::new_readonly(name_class, true));
    } else {
        accounts.push(AccountMeta::new_readonly(Pubkey::default(), false));
    }
    if let Some(name_parent) = name_parent_opt {
        accounts.push(AccountMeta::new_readonly(name_parent, false));
    } else {
        accounts.push(AccountMeta::new_readonly(Pubkey::default(), false));
    }
    if let Some(key) = name_parent_owner_opt {
        accounts.push(AccountMeta::new_readonly(key, true));
    }

    Ok(Instruction {
        program_id: name_service_program_id,
        accounts,
        data,
    })
}


pub fn update(
    name_service_program_id: Pubkey,
    offset: u32,
    data: Vec<u8>,
    name_account_key: Pubkey,
    name_update_signer: Pubkey,
) -> Result<Instruction, ProgramError> {
    let instruction_data = NameRegistryInstruction::Update { offset, data };
    let data = borsh::to_vec(&instruction_data).unwrap();
    let accounts = vec![
        AccountMeta::new(name_account_key, false),
        AccountMeta::new_readonly(name_update_signer, true),
    ];

    Ok(Instruction {
        program_id: name_service_program_id,
        accounts,
        data,
    })
}


pub fn transfer(
    name_service_program_id: Pubkey,
    new_owner: Pubkey,
    name_account_key: Pubkey,
    instruction_caller: Pubkey,
    root_name_account: Pubkey,
    new_custom_value: Option<u64>,
) -> Result<Instruction, ProgramError> {
    let instruction_data = NameRegistryInstruction::Transfer { 
        new_owner, custom_value: new_custom_value };
    let data = borsh::to_vec(&instruction_data).unwrap();
    let accounts = vec![
        AccountMeta::new(name_account_key, false),
        AccountMeta::new_readonly(instruction_caller, true),
        AccountMeta::new_readonly(root_name_account, false),
    ];

    Ok(Instruction {
        program_id: name_service_program_id,
        accounts,
        data,
    })
}


pub fn realloc(
    name_service_program_id: Pubkey,
    payer_key: Pubkey,
    name_account_key: Pubkey,
    name_owner_key: Pubkey,
    // the content length
    space: u32,
) -> Result<Instruction, ProgramError> {
    let instruction_data = NameRegistryInstruction::Realloc { space };
    let data = borsh::to_vec(&instruction_data).unwrap();
    let accounts = vec![
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new(payer_key, true),
        AccountMeta::new(name_account_key, false),
        AccountMeta::new_readonly(name_owner_key, true),
    ];

    Ok(Instruction {
        program_id: name_service_program_id,
        accounts,
        data,
    })
}


pub fn freeze_account(
    name_service_program_id: Pubkey,
    name_account_key: Pubkey,
    name_owner_key: Pubkey,
) -> Result<Instruction, ProgramError> {
    let instruction_data = NameRegistryInstruction::FreezeAccount;
    let data = borsh::to_vec(&instruction_data).unwrap();
    let accounts = vec![
        AccountMeta::new(name_account_key, false),
        AccountMeta::new_readonly(name_owner_key, true),
    ];

    Ok(Instruction {
        program_id: name_service_program_id,
        accounts,
        data,
    })
}
