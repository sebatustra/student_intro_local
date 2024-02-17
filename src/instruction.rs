use solana_program::program_error::ProgramError;
use borsh::BorshDeserialize;

pub enum IntroInstruction {
    AddStudentIntro {
        name: String,
        message: String
    },
    UpdateStudentIntro {
        name: String,
        message: String
    },
    AddReplyToIntro {
        name: String,
        reply: String
    }
}

#[derive(BorshDeserialize)]
struct IntroPayload {
    name: String,
    message: String
}

#[derive(BorshDeserialize)]
struct ReplyPayload {
    name: String,
    reply: String
}

impl IntroInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let payload = IntroPayload::try_from_slice(rest).unwrap();

                IntroInstruction::AddStudentIntro { name: payload.name, message: payload.message }
            },
            1 => {
                let payload = IntroPayload::try_from_slice(rest).unwrap();

                IntroInstruction::UpdateStudentIntro { name: payload.name, message: payload.message }
            },
            2 => {
                let payload = ReplyPayload::try_from_slice(rest).unwrap();

                IntroInstruction::AddReplyToIntro { name: payload.name, reply: payload.reply }
            },
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}