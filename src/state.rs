use solana_program::{
    program_pack::{ IsInitialized, Pack, Sealed },
    program_error::ProgramError,
    pubkey::Pubkey,
};

use arrayref::{ array_mut_ref, array_ref, array_refs, mut_array_refs };

pub struct NFTProfile {
    pub is_initialized: bool,
    pub version: u8,
    
    pub owner: Pubkey,
    pub nft_mint: Pubkey,
    pub nft_token: Pubkey,

    pub updated_at: u64,
}

impl Sealed for NFTProfile {}

impl IsInitialized for NFTProfile {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for NFTProfile {
    const LEN: usize = 106;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, NFTProfile::LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            is_initialized,
            version,
            owner,
            nft_mint,
            nft_token,
            updated_at,
        ) = array_refs![src, 1, 1, 32, 32, 32, 8];

        let result = NFTProfile {
            is_initialized: match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
            version: u8::from_le_bytes(*version),
            owner: Pubkey::new_from_array(*owner),
            nft_mint: Pubkey::new_from_array(*nft_mint),
            nft_token: Pubkey::new_from_array(*nft_token),
            updated_at: u64::from_le_bytes(*updated_at),
        };

        Ok(result)
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, NFTProfile::LEN];
        #[allow(clippy::ptr_offset_with_cast)]

        let (
            is_initialized_dst,
            version_dst,
            owner_dst,
            nft_mint_dst,
            nft_token_dst,
            updated_at_dst,
        ) = mut_array_refs![dst, 1, 1, 32, 32, 32, 8];

        let NFTProfile {
            is_initialized,
            version,
            owner,
            nft_mint,
            nft_token,
            updated_at,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        version_dst[0] = *version as u8;

        owner_dst.copy_from_slice(owner.as_ref());
        nft_mint_dst.copy_from_slice(nft_mint.as_ref());
        nft_token_dst.copy_from_slice(nft_token.as_ref());

        *updated_at_dst = updated_at.to_le_bytes();
    }
}
