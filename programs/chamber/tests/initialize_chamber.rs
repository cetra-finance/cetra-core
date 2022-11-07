mod state;
mod utils;

use cetra_program_test::{solana_program_test::*, *};
use solana_sdk::{signature::Keypair, signer::Signer};

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

    // 3. Fetch on-chain `Chamber`
    let _test_chain_chamber = test_chamber
        .fetch_chamber(&mut test_context)
        .await
        .expect("Unable to fetch chamber!");

    // TODO: Add more asserts
}
