// #![deny(missing_docs)]

//! ============================================================================
//! RAYDIUM AMM - Automated Market Maker for Solana
//! ============================================================================
//!
//! This is the MAIN ENTRY POINT of the Raydium AMM program.
//!
//! WHAT IS THIS FILE?
//! This file (lib.rs) is like the "table of contents" for the entire program.
//! It declares all the modules (files) and makes them available to each other.
//!
//! THINK OF IT LIKE A RESTAURANT:
//! - lib.rs = The menu (lists all available modules)
//! - state.rs = The ingredients (what data we store)
//! - instruction.rs = The orders (what customers can request)
//! - processor.rs = The kitchen (where orders are executed)
//! - math.rs = The recipes (how to calculate prices)
//! - error.rs = The complaints (what can go wrong)
//!
//! HOW SOLANA PROGRAMS WORK:
//! 1. User sends a transaction with an instruction
//! 2. Solana calls our entrypoint.rs
//! 3. entrypoint.rs calls processor.rs
//! 4. processor.rs reads instruction.rs to understand what to do
//! 5. processor.rs uses state.rs to read/write data
//! 6. processor.rs uses math.rs to calculate prices
//! 7. If something fails, error.rs provides the error message
//!
//! ============================================================================

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================
// Each "mod" line includes a file from this directory.
// "pub mod" = public (other programs can use it)
// "mod" = private (only this program can use it)

/// Logging utilities - for debugging and monitoring
/// The #[macro_use] means we can use log macros anywhere
#[macro_use]
pub mod log;

/// The entry point - where Solana first calls our program
/// Private because only Solana needs to call this directly
mod entrypoint;

/// Error types - all the things that can go wrong
/// Public so other programs can understand our errors
pub mod error;

/// Instruction definitions - the commands users can send
/// Public so wallets/UIs can build transactions
pub mod instruction;

/// Cross-program invocation helpers - calling other programs
/// Public for advanced integrations
pub mod invokers;

/// Math utilities - price calculations, fee calculations, etc.
/// Public so others can verify our math
pub mod math;

/// The processor - the "brain" that executes instructions
/// Public for testing and verification
pub mod processor;

/// State definitions - the data structures we store on-chain
/// Public so others can read pool data
/// Public so others can read pool data
pub mod state;

// ============================================================================
// SOLANA SDK EXPORT
// ============================================================================
// Export current solana-sdk types for downstream users who may also be
// building with a different solana-sdk version.
// This prevents version conflicts when other programs use Raydium.
pub use solana_program;

// ============================================================================
// SECURITY INFORMATION
// ============================================================================
// This macro embeds security contact info directly in the compiled program.
// Bug bounty hunters and auditors can find this info on-chain!
//
// WHY THIS MATTERS:
// If someone finds a critical bug, they know where to report it.
// Raydium has a bug bounty program on Immunefi.

#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "raydium-amm",
    project_url: "https://raydium.io",
    contacts: "link:https://immunefi.com/bounty/raydium",
    policy: "https://immunefi.com/bounty/raydium",
    source_code: "https://github.com/raydium-io/raydium-amm",
    preferred_languages: "en",
    auditors: "https://github.com/raydium-io/raydium-docs/blob/master/audit/MadSheild%20Q2%202023/Raydium%20updated%20orderbook%20AMM%20program%20%26%20OpenBook%20migration.pdf"
}

// ============================================================================
// PROGRAM ID - The program's "address" on Solana
// ============================================================================
// Every Solana program has a unique ID (like a wallet address).
// This ID is used to identify the program on-chain.
//
// TWO VERSIONS:
// 1. Devnet (for testing): DRaya7Kj3aMWQSy19kSjvmuwq9docCHofyP9kanQGaav
// 2. Mainnet (for real money): 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
//
// The #[cfg] attributes choose which one to use based on build flags.

#[cfg(feature = "devnet")]
solana_program::declare_id!("DRaya7Kj3aMWQSy19kSjvmuwq9docCHofyP9kanQGaav");

#[cfg(not(feature = "devnet"))]
solana_program::declare_id!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
