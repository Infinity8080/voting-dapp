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
        _candidate_id: u32,
        name: String,
    ) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate_account;
        candidate.candidate_name = name;
        candidate.canditate_votes = 0;

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
#[instruction(canidate_id:u32)]
pub struct InitCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer=signer,
        space=8+CandidateAccount::INIT_SPACE,
        seeds=[b"poll",signer.key().as_ref(),canidate_id.to_le_bytes().as_ref()],
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
