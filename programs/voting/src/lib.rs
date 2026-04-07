use anchor_lang::prelude::*;

declare_id!("A6a7ZHwc5cvh1yevgdPzZpvQB7eGEzt9E4CZQvTDPDXY");

#[program]
pub mod voting {
    use super::*;

    pub fn init_poll() -> Result<(ctx:Context<>)> {
        Ok(())
    }
}




#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(32)]
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
    #[max_len(32)]
    pub canditate_votes: u64,
}
