use solana_program::{
    program_pack::{Sealed, IsInitialized},
    pubkey::Pubkey
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct StudentIntroState {
    pub discriminator: String,
    pub is_initialized: bool,
    pub name: String,
    pub message: String
}

impl StudentIntroState {
    pub const DISCRIMINATOR: &'static str = "intro";

    pub fn get_account_size(name: String, message: String) -> usize {
        (4 + Self::DISCRIMINATOR.len())
        + 1
        + (4 + name.len())
        + (4 + message.len())
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ReplyCounterState {
    pub discriminator: String,
    pub is_initialized: bool,
    pub counter: u64
}

impl ReplyCounterState {
    pub const DISCRIMINATOR: &'static str = "counter";

    pub const SIZE: usize = (4 + Self::DISCRIMINATOR.len()) + 1 + 8;
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ReplyState {
    pub discriminator: String,
    pub is_initialized: bool,
    pub name: String,
    pub reply: String,
    pub replier: Pubkey,
    pub student_intro: Pubkey
}

impl ReplyState {
    pub const DISCRIMINATOR: &'static str = "reply";

    pub fn get_account_size(name: String, reply: String) -> usize {
        (4 + Self::DISCRIMINATOR.len())
        + 1
        + 32
        + 32
        + (4 + name.len())
        + (4 + reply.len())
    }
}

    /////////////////

impl Sealed for StudentIntroState {}

impl Sealed for ReplyCounterState {}

impl Sealed for ReplyState {}

impl IsInitialized for StudentIntroState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for ReplyCounterState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for ReplyState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}