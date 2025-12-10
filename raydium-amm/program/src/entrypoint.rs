//! ============================================================================
//! PROGRAM ENTRYPOINT - Where Solana Starts
//! ============================================================================
//!
//! This file defines the ENTRY POINT of the program - the first function
//! that Solana calls when someone sends a transaction to this program.
//!
//! THINK OF IT LIKE:
//! - A restaurant's front door (customers enter here)
//! - A website's homepage (all traffic starts here)
//! - main() in a regular program (execution starts here)
//!
//! THE FLOW:
//! 1. User sends transaction → Solana runtime
//! 2. Solana runtime → calls process_instruction() HERE
//! 3. process_instruction() → calls Processor::process()
//! 4. Processor::process() → does the actual work
//! 5. Result → returns back to Solana
//!
//! WHY A SEPARATE FILE?
//! - Keeps the entry logic simple and clean
//! - Allows disabling it for testing (see #[cfg] below)
//! - Standard Solana program pattern
//!
//! ============================================================================

// Only compile this file if we're NOT in "no-entrypoint" mode
// (Tests and benchmarks use "no-entrypoint" to call functions directly)
#![cfg(not(feature = "no-entrypoint"))]

use crate::{error::AmmError, processor::Processor};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    program_error::PrintProgramError, pubkey::Pubkey,
};

// ============================================================================
// THE ENTRYPOINT MACRO
// ============================================================================
// This macro tells Solana: "Hey, call THIS function when you execute my program!"
// It's like registering a callback function.

entrypoint!(process_instruction);

// ============================================================================
// THE ENTRY FUNCTION
// ============================================================================
// This is THE function Solana calls. Every transaction goes through here.
//
// PARAMETERS:
// - program_id: The ID of THIS program (our address)
// - accounts: All the accounts (data) this transaction needs
// - instruction_data: The raw bytes of the instruction (what to do)
//
// RETURNS:
// - Ok(()) if everything worked
// - Err(error) if something failed

fn process_instruction<'a>(
    program_id: &Pubkey,             // "Who am I?" (this program's ID)
    accounts: &'a [AccountInfo<'a>], // "What data do I have access to?"
    instruction_data: &[u8],         // "What should I do?" (raw bytes)
) -> ProgramResult {
    // Try to process the instruction
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // If it failed, print the error to the logs
        // (This helps with debugging - you can see the error in transaction logs)
        error.print::<AmmError>();

        // Return the error to Solana (transaction will fail)
        return Err(error);
    }

    // Success! Transaction will succeed
    Ok(())
}
