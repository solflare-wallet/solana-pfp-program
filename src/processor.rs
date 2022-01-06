use solana_program::{
    entrypoint::ProgramResult,
    program::{ invoke, invoke_signed },
    program_error::ProgramError,
    program_pack::{ IsInitialized, Pack },

    account_info::{ next_account_info, AccountInfo },
    pubkey::Pubkey,
    
    sysvar::{ rent::Rent, Sysvar, clock::Clock },

    msg,
    system_instruction,
};

use std::convert::TryInto;

use spl_token::state::{ Account as TokenAccount, Mint as MintAccount };

use crate::{
    error::NFTError, state::NFTProfile, params::{ PREFIX, METADATA_PREFIX }
};

pub struct InstructionProcessor;
impl InstructionProcessor {
    pub fn process_set_profile_nft(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let signer_account = next_account_info(account_info_iter)?;

        if ! signer_account.is_signer {
            msg!("Invalid signer provided.");

            return Err(ProgramError::MissingRequiredSignature);
        }

        let nft_profile_account = next_account_info(account_info_iter)?;

        if ! nft_profile_account.is_writable {
            msg!("NFT profile account is not writable.");

            return Err(ProgramError::InvalidAccountData);
        }

        let nft_mint_account = next_account_info(account_info_iter)?;

        let nft_token_account = next_account_info(account_info_iter)?;

        let nft_metadata_account = next_account_info(account_info_iter)?;

        let clock = &Clock::from_account_info(next_account_info(account_info_iter)?)?;

        let system_program = next_account_info(account_info_iter)?;

        let rent = Rent::get()?;

        let nft_mint_account_info = MintAccount::unpack_from_slice(&nft_mint_account.data.borrow())?;

        if nft_mint_account_info.supply != 1u64 {
            msg!("Invalid NFT mint supply.");

            return Err(NFTError::InvalidMintSupply.into())
        }

        let nft_token_account_info = TokenAccount::unpack_from_slice(&nft_token_account.data.borrow())?;

        if nft_token_account_info.owner != * signer_account.key {
            msg!("Invalid NFT token account owner.");

            return Err(NFTError::InvalidOwner.into())
        }

        if nft_token_account_info.amount < 1u64 {
            msg!("Insufficient NFT token account balance.");

            return Err(NFTError::InsufficientTokenBalance.into())
        }

        if nft_token_account_info.mint != * nft_mint_account.key {
            msg!("Invalid NFT token account mint.");

            return Err(NFTError::InvalidMint.into())
        }

        let (nft_metadata_account_pda, _) = Pubkey::find_program_address(
            &[
                METADATA_PREFIX.as_bytes(),
                spl_token_metadata::id().as_ref(),
                nft_mint_account.key.as_ref(),
            ],
            &spl_token_metadata::id(),
        );

        if nft_metadata_account_pda != * nft_metadata_account.key {
            msg!("Invalid NFT metadata.");
        
            return Err(NFTError::InvalidMetadata.into())
        }

        let (nft_profile_pda, nft_profile_pda_bump_seed) = Pubkey::find_program_address(&[
            PREFIX.as_bytes(),
            &signer_account.key.to_bytes(),
        ], program_id);

        if nft_profile_pda != * nft_profile_account.key {
            msg!("NFT profile account PDA doesn't match.");

            return Err(ProgramError::InvalidAccountData);
        }

        let mut nft_profile_account_after = nft_profile_account.clone();

        if nft_profile_account.data_len() == 0 {
            let nft_profile_pda_signer_seeds: &[&[_]] = &[
                PREFIX.as_bytes(),
                &signer_account.key.to_bytes(),
                &[nft_profile_pda_bump_seed],
            ];
            
            if nft_profile_account.lamports() > 0 {
                let required_lamports = rent
                    .minimum_balance(NFTProfile::LEN)
                    .max(1)
                    .saturating_sub(nft_profile_account.lamports());
        
                if required_lamports > 0 {
                    invoke(
                        &system_instruction::transfer(signer_account.key, nft_profile_account.key, required_lamports),
                        &[
                            signer_account.clone(),
                            nft_profile_account.clone(),
                            system_program.clone(),
                        ],
                    )?;
                }

                let account_infos = [nft_profile_account.clone(), system_program.clone()];
        
                invoke_signed(
                    &system_instruction::allocate(nft_profile_account.key, NFTProfile::LEN as u64),
                    &account_infos,
                    &[nft_profile_pda_signer_seeds],
                )?;
        
                invoke_signed(
                    &system_instruction::assign(nft_profile_account.key, program_id),
                    &account_infos,
                    &[nft_profile_pda_signer_seeds],
                )?;

                nft_profile_account_after = account_infos[0].clone();
            } else {
                let account_infos = [
                    signer_account.clone(),
                    nft_profile_account.clone(),
                    system_program.clone(),
                ];

                invoke_signed(
                    &system_instruction::create_account(
                        signer_account.key,
                        nft_profile_account.key,
                        rent.minimum_balance(NFTProfile::LEN).max(1),
                        NFTProfile::LEN as u64,
                        program_id,
                    ),
                    &account_infos,
                    &[nft_profile_pda_signer_seeds],
                )?;

                nft_profile_account_after = account_infos[1].clone();
            }
        } else {
            if nft_profile_account.owner != program_id {
                msg!("Invalid NFT profile account program owner.");

                return Err(ProgramError::InvalidAccountData);
            }
        }

        let mut nft_profile_account_info = NFTProfile::unpack_unchecked(&nft_profile_account_after.data.borrow())?;

        nft_profile_account_info.is_initialized = true;
        nft_profile_account_info.version = 1u8;

        nft_profile_account_info.owner = * signer_account.key;
        nft_profile_account_info.nft_mint = * nft_mint_account.key;
        nft_profile_account_info.nft_token = * nft_token_account.key;

        nft_profile_account_info.updated_at = clock.unix_timestamp.try_into().unwrap();

        NFTProfile::pack(nft_profile_account_info, &mut nft_profile_account_after.data.borrow_mut())?;

        Ok(())
    }

    pub fn process_unset_profile_nft(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let signer_account = next_account_info(account_info_iter)?;

        if ! signer_account.is_signer {
            msg!("Invalid signer provided.");

            return Err(ProgramError::MissingRequiredSignature);
        }

        let nft_profile_account = next_account_info(account_info_iter)?;

        if ! nft_profile_account.is_writable {
            msg!("NFT profile account is not writable.");

            return Err(ProgramError::InvalidAccountData);
        }

        if nft_profile_account.owner != program_id {
            msg!("Invalid NFT profile account program owner.");

            return Err(ProgramError::InvalidAccountData);
        }

        let (nft_profile_pda, _) = Pubkey::find_program_address(&[
            PREFIX.as_bytes(),
            &signer_account.key.to_bytes(),
        ], program_id);

        if nft_profile_pda != * nft_profile_account.key {
            msg!("NFT profile account PDA doesn't match.");

            return Err(ProgramError::InvalidAccountData);
        }

        let nft_profile_account_info = NFTProfile::unpack_from_slice(&nft_profile_account.data.borrow())?;

        if ! nft_profile_account_info.is_initialized() {
            msg!("NFT profile account is not initialized.");

            return Err(NFTError::NotInitialized.into())
        }

        if nft_profile_account_info.owner != * signer_account.key {
            msg!("Invalid NFT profile account owner.");

            return Err(NFTError::InvalidOwner.into())
        }

        let signer_account_lamports = signer_account.lamports();

        **signer_account.lamports.borrow_mut() = signer_account_lamports
            .checked_add(nft_profile_account.lamports())
            .unwrap();

        **nft_profile_account.lamports.borrow_mut() = 0;

        Ok(())
    }
}