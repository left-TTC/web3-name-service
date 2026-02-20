
#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

pub use solana_program;

solana_program::declare_id!("88WyWKRnvUqJqAX15Xj1nmiykqeBTwbaST6QvJD1jPBJ");


