use anchor_lang::prelude::*;

declare_id!("FfpcmdLeYbLCdyECrfqsGePwp5D9SRoyoqTzr2XN1kDj");

#[error_code]
pub enum ErrorCode {
    #[msg("The task text is too long")]
    TextTooLong,
}

// size in bytes of the Task struct
const DISCRIMINATOR: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const BOOL_LENGTH: usize = 1;
const TEXT_LENGTH: usize = 4 + 400 * 4; // 400 characters + 4 bytes for prefix
const TIMESTAMP_LENGTH: usize = 8;

#[account] // helps to implement traits and be a Solana account
pub struct Task {
    pub author: Pubkey, // The account owner
    pub text: String,   // The task text
    pub is_done: bool,  // Whether the task is done or not
    pub created_at: i64, // The timestamp when the task was created
    pub updated_at: i64, // The timestamp when the task was updated
}

impl Task {
    const LEN: usize = DISCRIMINATOR
        + PUBLIC_KEY_LENGTH
        + TEXT_LENGTH
        + BOOL_LENGTH
        + TIMESTAMP_LENGTH
        + TIMESTAMP_LENGTH;
}


#[derive(Accounts)]
pub struct AddTaskAccounts<'info> { // lifetime parameter ('info) ensures accounts can be referenced throughout the instruction
    // accounts needed for tx
    // we initialize, author pays, space is LEN
    #[account(init, payer = author, space = Task::LEN)]
    pub task: Account<'info, Task>,
    // author account must be mutable to change balance
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>, // used to initialize accounts
}

#[derive(Accounts)]
pub struct UpdateTaskAccounts<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>
}

#[derive(Accounts)]
pub struct DeleteTaskAccounts<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>
}

#[program]
pub mod todo_list_dapp {
    use super::*;
        
    pub fn add_task(ctx: Context<AddTaskAccounts>, text: String) -> Result<()> {
        // mutable ref to task so it can be changed
        let task = &mut ctx.accounts.task;
        // borrow author
        let author = &ctx.accounts.author; // 'author' account
        let clock = Clock::get().unwrap(); // get current time from Solana Clock service
    
        // check the char limit
        if text.chars().count() > 400 {
            // initial error after this line because return statement is not of type Result
            // further code corrects this error
            return Err(ErrorCode::TextTooLong.into());
        }
    
        // assign values to task
        task.author = *author.key;
        task.is_done = false;
        task.created_at = clock.unix_timestamp;
        task.updated_at = clock.unix_timestamp;
        task.text = text;
        
        // return Ok(()) to indicate success
        Ok(())
    }

    pub fn update_task(ctx: Context<UpdateTaskAccounts>, is_done: bool) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let clock = Clock::get().unwrap();
        task.is_done = is_done;
        task.updated_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn delete_task(ctx: Context<DeleteTaskAccounts>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        task.author = *author.key;
        task.is_done = true;
        task.updated_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }
}