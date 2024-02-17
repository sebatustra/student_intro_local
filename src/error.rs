use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Debug, Error)]
pub enum StudentIntroError {
    #[error("PDA given does not match PDA derived!")]
    InvalidPDA,

    #[error("Account has not been initialized yet!")]
    UninitializedAccount,

    #[error("Data passed is too large!")]
    InvalidDataLength
}

impl From<StudentIntroError> for ProgramError {
    fn from(e: StudentIntroError) -> Self {
        ProgramError::Custom(e as u32)
    }
}