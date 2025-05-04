
#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;


pub use solana_program;

solana_program::declare_id!("HN9YwRE2coNQVDEvpSwd6XqZLfBTs1f2hkojBN5PSaBN");


//solana program deploy target/sbf-solana-solana/release/web3_name_service.so 
//--program-id HN9YwRE2coNQVDEvpSwd6XqZLfBTs1f2hkojBN5PSaBN --use-rpc

