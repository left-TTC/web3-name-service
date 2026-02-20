use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::AccountInfo,
        msg,
        program_error::ProgramError,
        program_pack::{IsInitialized, Pack, Sealed},
        pubkey::Pubkey,
    },
};

/// The data for a Name Registry account is always prefixed a `NameRecordHeader`
/// structure.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct NameRecordHeader {
    pub parent_name: Pubkey,
    pub owner: Pubkey,
    pub class: Pubkey,
    pub is_frozen: bool,
    pub custom_price: u64,
}

impl Sealed for NameRecordHeader {}

impl Pack for NameRecordHeader {
    const LEN: usize = 105;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut slice = dst;
        self.serialize(&mut slice).unwrap()
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let mut p = src;
        NameRecordHeader::deserialize(&mut p).map_err(|_| {
            msg!("Failed to deserialize name record");
            ProgramError::InvalidAccountData
        })
    }
}

impl IsInitialized for NameRecordHeader {
    fn is_initialized(&self) -> bool {
        self.owner == Pubkey::default()
    }
}

pub fn write_data(account: &AccountInfo, input: &[u8], offset: usize) {
    let mut account_data = account.data.borrow_mut();
    account_data[offset..offset.saturating_add(input.len())].copy_from_slice(input);
}


