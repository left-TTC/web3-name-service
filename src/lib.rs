
#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

pub use solana_program;

solana_program::declare_id!("29CkJByNom4XprPhyntVis1jqjDzHHx43do4oYeDDQRL");


//solana program deploy target/sbf-solana-solana/release/web3_name_service.so 
//--program-id HN9YwRE2coNQVDEvpSwd6XqZLfBTs1f2hkojBN5PSaBN --use-rpc

