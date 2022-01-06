use solana_program::program_error::ProgramError;

use crate::error::NFTError::{ InvalidInstruction };

pub enum NFTInstruction {
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person setting the NFT profile picture
    /// 1. `[writable]` NFT profile picture account (PDA)
    /// 2. `[]` NFT mint
    /// 3. `[]` NFT token account
    /// 4. `[]` NFT metadata
    /// 5. `[]` Clock sysvar
    /// 6. `[]` System program
    SetProfileNFT { },

    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person removing the NFT profile picture
    /// 1. `[writable]` NFT profile picture account (PDA)
    UnsetProfileNFT { },
}

impl NFTInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, _) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::SetProfileNFT { },
            1 => Self::UnsetProfileNFT { },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}