use anchor_lang::prelude::*;

declare_id!("A6a7ZHwc5cvh1yevgdPzZpvQB7eGEzt9E4CZQvTDPDXY");

#[program]
pub mod voting {
    use super::*;

    pub fn init_poll(
        ctx: Context<InitPoll>,
        _poll_id: u32,
        name: String,
        description: String,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.post_account;
        poll.poll_description = description;
        poll.poll_name = name;
        poll.poll_start_time = start_time;
        poll.poll_end_time = end_time;

        Ok(())
    }
    pub fn init_candidate(
        ctx: Context<InitCandidate>,
        _poll_id: u32,
        candidate: String,
    ) -> Result<()> {
        let cdt = &mut ctx.accounts.candidate_account;
        ctx.accounts.poll_account.poll_index += 1;
        cdt.candidate_name = candidate;
        cdt.canditate_votes = 0;

        Ok(())
    }
    pub fn vote(ctx: Context<Vote>, _poll_id: u32, _candidate: String) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;

        if current_time > ctx.accounts.poll_account.poll_end_time as i64 {
            return err!(VoteError::VotingFinished);
        } else if current_time < ctx.accounts.poll_account.poll_start_time as i64 {
            return err!(VoteError::VoteNotStarted);
        }

        ctx.accounts.candidate_account.canditate_votes += 1;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id:u32)]
pub struct InitPoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer=signer,
        space=8+PollAccount::INIT_SPACE,
        seeds=[b"poll",signer.key().as_ref(),poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub post_account: Account<'info, PollAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id:u32, candidate:String)]
pub struct InitCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut,
        seeds=[b"poll",signer.key().as_ref(),poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,
    #[account(
        init,
        payer=signer,
        space=8+CandidateAccount::INIT_SPACE,
        seeds=[poll_id.to_le_bytes().as_ref(),candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id:u32, candidate:String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut,
        seeds=[b"poll",signer.key().as_ref(),poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,
    #[account(
        init,
        payer=signer,
        space=8+CandidateAccount::INIT_SPACE,
        seeds=[poll_id.to_le_bytes().as_ref(),candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(200)]
    pub poll_description: String,
    pub poll_start_time: u64,
    pub poll_end_time: u64,
    pub poll_index: u64,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name: String,
    pub canditate_votes: u64,
}

#[error_code]
pub enum VoteError {
    #[msg("Voting has not started yet!")]
    VoteNotStarted,
    #[msg("Time up! Voting is finished!")]
    VotingFinished,
}
