use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey, msg,
};

use crate::{
    error::NFTError, instruction::NFTInstruction, processor::InstructionProcessor
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = NFTInstruction::unpack(instruction_data)?;

    match instruction {
        NFTInstruction::SetProfileNFT { } => {
            msg!("Instruction: SetProfileNFT");
            InstructionProcessor::process_set_profile_nft(accounts, program_id)
        },
        NFTInstruction::UnsetProfileNFT { } => {
            msg!("Instruction: UnsetProfileNFT");
            InstructionProcessor::process_unset_profile_nft(accounts, program_id)
        },
        #[allow(unreachable_patterns)]
        _ => return Err(NFTError::InvalidInstruction.into()),
    }
}