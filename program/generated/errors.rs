// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::msg;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Error, FromPrimitive, Debug, Clone)]
pub enum PortfolioNftError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Invalid Signer Permission")]
    InvalidSignerPermission,

    #[error("Not The Expected Account Address")]
    NotExpectedAddress,

    #[error("Wrong Account Owner")]
    WrongAccountOwner,

    #[error("Invalid Account Len")]
    InvalidAccountLen,

    #[error("Executable Account Expected")]
    ExecutableAccountExpected,

 
}

impl From<PortfolioNftError> for ProgramError {
    fn from(e: PortfolioNftError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for PortfolioNftError {
    fn type_of() -> &'static str {
        "PortfolioNftError"
    }
}

impl PrintProgramError for PortfolioNftError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            PortfolioNftError::InvalidInstruction => msg!("Error: Invalid instruction"),
            PortfolioNftError::InvalidSignerPermission => msg!("Error: The account is_signer value is not the expected one"),
            PortfolioNftError::NotExpectedAddress => {
                msg!("Error: Not the expected account address")
            }
            PortfolioNftError::WrongAccountOwner => msg!("Error: Wrong account owner"),
            PortfolioNftError::InvalidAccountLen => msg!("Error: Invalid account length"),
            PortfolioNftError::ExecutableAccountExpected => msg!("Error: Executable account expected"),
 
        }
    }
}