mod state;
mod utils;

use cetra_program_test::{solana_program_test::*, *};
use solana_sdk::{signature::Keypair, signer::Signer};

const USER_FUND_LAMPORTS: u64 = 1000000000;

#[tokio::test(flavor = "multi_thread")]
async fn success() {
    let rpc_accounts_loader = RpcAccountsLoader::default();
    let mut program_test_loader = ProgramTestLoader::default();

    program_test_loader
        .program_test
        .add_program("cetra_chamber", cetra_chamber::id(), None);
    program_test_loader
        .load()
        .expect("Unable to load accounts!");

    let mut test_context = program_test_loader
        .start_with_context(Box::new(rpc_accounts_loader))
        .await;

    let payer = utils::clone_keypair(&test_context.context.payer);
    let owner = Keypair::new();
    let fee_manager = Keypair::new();

    // 1. Build `Chamber`
    let test_chamber = state::Chamber::build_raydium_sol_usdc(
        &mut test_context,
        &payer,
        &owner,
        &fee_manager.pubkey(),
        0,
    )
    .await
    .expect("Unable to build raydium SOL/USDC chamber!");

    // 2. Initialize `Chamber`
    test_chamber
        .initialize_chamber(&mut test_context, &payer)
        .await
        .expect("Unable to initialize chamber!");

    // 3. Build user, which associated with `Chamber`
    let test_user = state::User::build_with_chamber(
        &mut test_context,
        &payer,
        &test_chamber,
        USER_FUND_LAMPORTS,
        0,
        0,
    )
    .await
    .expect("Unable to build user with chamber!");

    // 4. Create `UserAccount`
    test_user
        .create_user_account(&mut test_context)
        .await
        .expect("Unable to create user account!");

    // 5. Fetch on-chain `UserAccount`
    let test_chain_user_account = test_user
        .fetch_user_account(&mut test_context)
        .await
        .expect("Unable to fetch user account!");

    assert_eq!(test_chain_user_account.chamber, test_chamber.get_pubkey());
    assert_eq!(test_chain_user_account.user, test_user.get_pubkey());
    assert_eq!(
        test_chain_user_account.status,
        cetra_chamber::state::UserAccountStatus::Ready
    );
    assert_eq!(test_chain_user_account.locked_base_amount, 0);
    assert_eq!(test_chain_user_account.locked_quote_amount, 0);
    assert_eq!(test_chain_user_account.locked_shares_amount, 0);
}
