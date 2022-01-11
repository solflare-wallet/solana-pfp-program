use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum NFTError {
    /// 0 (0x0) Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction,

    /// 1 (0x1) Not rent exempt
    #[error("Not rent exempt")]
    NotRentExempt,

    /// 2 (0x2) Not initialized
    #[error("Not initialized")]
    NotInitialized,

    /// 3 (0x3) Invalid owner
    #[error("Invalid owner")]
    InvalidOwner,

    /// 4 (0x4) Invalid mint supply
    #[error("Invalid mint supply")]
    InvalidMintSupply,

    /// 5 (0x5) Insufficient token balance
    #[error("Insufficient token balance")]
    InsufficientTokenBalance,

    /// 6 (0x6) Invalid mint
    #[error("Invalid mint")]
    InvalidMint,

    /// 7 (0x7) Invalid metadata
    #[error("Invalid metadata")]
    InvalidMetadata,
}

impl From<NFTError> for ProgramError {
    fn from(e: NFTError) -> Self {
        ProgramError::Custom(e as u32)
    }
}