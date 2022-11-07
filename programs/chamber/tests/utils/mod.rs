use anchor_spl::token;
use cetra_program_test::{
    solana_sdk::{
        pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction,
        transaction::Transaction, transport,
    },
    TestContext,
};

#[allow(unused)]
pub async fn create_token_mint(
    test_context: &mut TestContext,
    payer: &Keypair,
    mint: &Keypair,
) -> transport::Result<()> {
    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::create_account(
            &payer.pubkey(),
            &mint.pubkey(),
            test_context
                .get_rent()
                .await
                .minimum_balance(token::Mint::LEN),
            token::Mint::LEN as u64,
            &token::ID,
        )],
        Some(&payer.pubkey()),
        &[payer, mint],
        test_context.context.last_blockhash,
    );

    test_context.process_transaction(tx).await
}

#[allow(unused)]
pub async fn transfer(
    test_context: &mut TestContext,
    from: &Keypair,
    to: &Pubkey,
    lamports: u64,
) -> transport::Result<()> {
    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(&from.pubkey(), to, lamports)],
        Some(&from.pubkey()),
        &[from],
        test_context.context.last_blockhash,
    );

    test_context.process_transaction(tx).await
}

#[allow(unused)]
pub fn clone_keypair(keypair: &Keypair) -> Keypair {
    Keypair::from_bytes(&keypair.to_bytes()).unwrap()
}
