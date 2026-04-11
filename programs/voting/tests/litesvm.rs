use ::voting::accounts::InitPoll;
use anchor_lang::{declare_program, Id};
use anchor_litesvm::{
    AnchorContext, AnchorLiteSVM, AssertionHelpers, Keypair, Pubkey, Signer, TestHelpers,
};

declare_program!(voting);

use self::voting::accounts::PollAccount;
use self::voting::client::{accounts, args};

const PROGRAM_BYTES: &[u8] = include_bytes!("../../../target/deploy/voting.so");

fn setup() -> AnchorContext {
    use anchor_lang::solana_program::clock::Clock;

    let mut ctx = AnchorLiteSVM::build_with_program(self::voting::ID, PROGRAM_BYTES);

    let clock = Clock {
        slot: 1000,
        epoch_start_timestamp: 0,
        epoch: 1,
        leader_schedule_epoch: 1,
        unix_timestamp: 1000,
    };
    ctx.svm.set_sysvar(&clock);
    ctx
}

fn get_poll_pda(signer: &Pubkey, poll_id: u32) -> Pubkey {
    Pubkey::find_program_address(
        &[b"poll", signer.as_ref(), &poll_id.to_le_bytes()],
        &self::voting::ID_CONST,
    )
    .0
}

fn init_poll(
    ctx: &mut AnchorContext,
    signer: &Keypair,
    name: &str,
    description: &str,
    start_time: u64,
    end_time: u64,
    poll_id: u32,
) {
    let poll_pda = get_poll_pda(&signer.pubkey(), poll_id);
    let ix = ctx
        .program()
        .accounts(accounts::InitPoll {
            signer: signer.pubkey(),
            post_account: poll_pda,
            system_program: anchor_lang::system_program::ID,
        })
        .args(args::InitPoll {
            _poll_id: poll_id,
            description: description.to_string(),
            name: name.to_string(),
            end_time,
            start_time,
        })
        .instruction()
        .unwrap();

    let result = ctx.execute_instruction(ix, &[signer]).unwrap();
    result.assert_success();
    ctx.svm.assert_account_exists(&poll_pda);
}

#[test]
fn test_init_poll() {
    let mut ctx = setup();
    let user = ctx.create_funded_account(10_000_000).unwrap();
    let poll_id = 1;
    let poll_pda = get_poll_pda(&user.pubkey(), poll_id);
    let start_time = 0;
    let end_time = u64::MAX;
    let poll_name = "Test Poll";
    let poll_description = "A test poll for voting";

    init_poll(
        &mut ctx,
        &user,
        poll_name,
        poll_description,
        start_time,
        end_time,
        poll_id,
    );
    let poll_account: PollAccount = ctx.get_account(&poll_pda).unwrap();
    assert_eq!(poll_account.poll_description, poll_description);
    assert_eq!(poll_account.poll_name, poll_name);
    assert_eq!(poll_account.poll_end_time, end_time);
    assert_eq!(poll_account.poll_start_time, start_time);
    assert_eq!(poll_account.poll_index, 0);
}
