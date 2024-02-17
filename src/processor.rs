use borsh::BorshSerialize;
use solana_program::{
    account_info::{
        next_account_info, 
        AccountInfo
    }, 
    borsh1::try_from_slice_unchecked, 
    entrypoint::ProgramResult, 
    msg, 
    program::invoke_signed, 
    program_error::ProgramError, 
    program_pack::IsInitialized, 
    pubkey::Pubkey, 
    system_instruction, 
    sysvar::{rent::Rent, Sysvar}
};
use crate::instruction::IntroInstruction;
use crate::state::{
    ReplyCounterState,
    ReplyState,
    StudentIntroState
};
use crate::error::StudentIntroError;

pub fn process_intruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    msg!("Processing instruction...");

    let instruction = IntroInstruction::unpack(instruction_data)?;

    match instruction {
        IntroInstruction::AddStudentIntro { 
            name, 
            message 
        } => add_student_intro(program_id, accounts, name, message),
        IntroInstruction::UpdateStudentIntro { 
            name, 
            message 
        } => update_student_intro(program_id, accounts, name, message),
        IntroInstruction::AddReplyToIntro { 
            name, 
            reply 
        } => add_reply_to_intro(program_id, accounts, name, reply)
    }
}

fn add_student_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String
) -> ProgramResult {
    msg!("Starting add_student_intro");

    let accounts_iter = &mut accounts.iter();

    let initializer = next_account_info(accounts_iter)?;
    let pda_intro_given = next_account_info(accounts_iter)?;
    let pda_counter_given = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if !initializer.is_signer {
        msg!("Missing initializer signature");
        return Err(ProgramError::IllegalOwner)
    }

    let (pda_intro_derived, intro_bump) = Pubkey::find_program_address(
       &[initializer.key.as_ref()], 
        program_id
    );

    if pda_intro_derived != *pda_intro_given.key {
        msg!("provided intro PDA does not match with the derived PDA");
        return Err(StudentIntroError::InvalidPDA.into())
    }

    let account_len: usize = 1000;
    let current_len = StudentIntroState::get_account_size(name.clone(), message.clone());

    if current_len > account_len {
        return Err(StudentIntroError::InvalidDataLength.into())
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key, 
            pda_intro_given.key, 
            rent_lamports, 
            account_len.try_into().unwrap(), 
            program_id
        ),
        &[
            initializer.clone(),
            pda_intro_given.clone(),
            system_program.clone()
        ],
        &[&[
            initializer.key.as_ref(),
            &[intro_bump]
        ]]
    )?;

    let mut intro_data = 
        try_from_slice_unchecked::<StudentIntroState>(&pda_intro_given.data.borrow())
        .unwrap();

    if intro_data.is_initialized() {
        msg!("Intro PDA account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized)
    }

    intro_data.discriminator = StudentIntroState::DISCRIMINATOR.to_string();
    intro_data.is_initialized = true;
    intro_data.name = name;
    intro_data.message = message;

    intro_data.serialize(&mut &mut pda_intro_given.data.borrow_mut()[..])?;

    msg!("student intro account serialized");

    let (pda_counter_derived, counter_bump) = Pubkey::find_program_address(
        &[pda_intro_derived.as_ref(), ReplyCounterState::DISCRIMINATOR.as_ref()], 
        program_id
    );

    if pda_counter_derived != *pda_counter_given.key {
        return Err(StudentIntroError::InvalidPDA.into())
    }

    let counter_len = ReplyCounterState::SIZE;
    let counter_lamports = rent.minimum_balance(counter_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key, 
            pda_counter_given.key, 
            counter_lamports, 
            counter_len.try_into().unwrap(), 
            program_id
        ),
        &[
            initializer.clone(),
            pda_counter_given.clone(),
            system_program.clone(),
        ],
        &[&[
            pda_counter_derived.as_ref(),
            ReplyCounterState::DISCRIMINATOR.as_ref(),
            &[counter_bump]
        ]]
    )?;

    let mut counter_data =
        try_from_slice_unchecked::<ReplyCounterState>(&pda_counter_given.data.borrow())
        .unwrap();

    if counter_data.is_initialized() {
        msg!("Counter PDA already initialized");
        return Err(ProgramError::AccountAlreadyInitialized)
    }

    counter_data.is_initialized = true;
    counter_data.discriminator = ReplyCounterState::DISCRIMINATOR.to_string();
    counter_data.counter = 0;

    counter_data.serialize(&mut &mut pda_counter_given.data.borrow_mut()[..])?;

    msg!("add_student_intro was succesfull!");

    Ok(())
}

fn update_student_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let initializer = next_account_info(accounts_iter)?;
    let pda_given = next_account_info(accounts_iter)?;

    if !initializer.is_signer {
        msg!("signer is not the initializer");
        return Err(ProgramError::IllegalOwner)
    }

    let (pda_derived, _bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref()], 
        program_id
    );

    if pda_derived != *pda_given.key {
        msg!("PDA given does not match PDA derived");
        return Err(StudentIntroError::InvalidPDA.into())
    }

    let account_len: usize = 1000;
    let current_len = StudentIntroState::get_account_size(name, message.clone());

    if current_len > account_len {
        msg!("Account length too large");
        return Err(StudentIntroError::InvalidDataLength.into())
    }

    let mut account_data = 
        try_from_slice_unchecked::<StudentIntroState>(&pda_given.data.borrow())
        .unwrap();

    if account_data.is_initialized() {
        msg!("account not initialized");
        return Err(StudentIntroError::UninitializedAccount.into())
    }

    account_data.message = message;

    account_data.serialize(&mut &mut pda_given.data.borrow_mut()[..])?;

    Ok(())
}

fn add_reply_to_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    reply: String
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let initializer = next_account_info(accounts_iter)?;
    let pda_intro_given = next_account_info(accounts_iter)?;
    let pda_counter_given = next_account_info(accounts_iter)?;
    let pda_reply_given = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if !initializer.is_signer {
        msg!("initializer is not signer");
        return Err(ProgramError::IllegalOwner)
    }

    let mut counter_data = 
        try_from_slice_unchecked::<ReplyCounterState>(&pda_counter_given.data.borrow())
        .unwrap();

    let (pda_reply_derived, reply_bump) = Pubkey::find_program_address(
        &[
            pda_intro_given.key.as_ref(),
            counter_data.counter.to_be_bytes().as_ref()
        ], 
        program_id
    );

    if pda_reply_derived != * pda_reply_given.key {
        msg!("Reply PDA does not match passed reply PDA");
        return Err(StudentIntroError::InvalidPDA.into())
    }

    let rent = Rent::get()?;
    let account_len = ReplyState::get_account_size(name.clone(), reply.clone());

    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key, 
            pda_reply_given.key, 
            rent_lamports, 
            account_len.try_into().unwrap(), 
            program_id
        ),
        &[
            initializer.clone(),
            pda_counter_given.clone(),
            system_program.clone()
        ],
        &[&[
            pda_intro_given.key.as_ref(),
            counter_data.counter.to_be_bytes().as_ref(),
            &[reply_bump]
        ]]
    )?;

    let mut reply_data = 
        try_from_slice_unchecked::<ReplyState>(&pda_reply_given.data.borrow())
        .unwrap();

    if reply_data.is_initialized() {
        msg!("Reply account was already initialized");
        return Err(ProgramError::AccountAlreadyInitialized)
    }

    reply_data.is_initialized = true;
    reply_data.discriminator = ReplyState::DISCRIMINATOR.to_string();
    reply_data.name = name;
    reply_data.reply = reply;
    reply_data.replier = *initializer.key;
    reply_data.student_intro = *pda_intro_given.key;

    reply_data.serialize(&mut &mut pda_reply_given.data.borrow_mut()[..])?;

    counter_data.counter += 1;

    counter_data.serialize(&mut &mut pda_counter_given.data.borrow_mut()[..])?;

    Ok(())
}

