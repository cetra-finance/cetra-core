use super::Chamber;
use crate::utils;
use anchor_lang::{AnchorDeserialize, InstructionData, ToAccountMetas};
use anchor_spl::{associated_token, token};
use cetra_program_test::{
    solana_sdk::{
        instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer,
        system_program, sysvar::rent, transaction::Transaction, transport,
    },
    TestContext,
};

#[derive(Debug)]
pub struct User {
    keypair: Keypair,
    user_account: Pubkey,
    user_shares: Pubkey,
    base_ata: Pubkey,
    quote_ata: Pubkey,
    chamber: Pubkey,
    shares_mint: Pubkey,
}

impl User {
    #[allow(unused)]
    pub async fn build_with_chamber(
        test_context: &mut TestContext,
        payer: &Keypair,
        chamber: &Chamber,
        lamports: u64,
        base_amount: u64,
        quote_amount: u64,
    ) -> transport::Result<Self> {
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey();

        let chamber_pubkey = chamber.get_pubkey();
        let shares_mint = chamber.get_shares_mint();
        let base_mint = chamber.get_base_mint();
        let quote_mint = chamber.get_quote_mint();

        let (user_account, _) =
            cetra_chamber::utils::derive_user_account_address(&chamber_pubkey, &pubkey);

        let user_shares = associated_token::get_associated_token_address(&pubkey, &shares_mint);

        utils::transfer(test_context, payer, &pubkey, lamports).await?;

        test_context
            .create_ata(&pubkey, &base_mint, base_amount)
            .await?;

        test_context
            .create_ata(&pubkey, &quote_mint, quote_amount)
            .await?;

        let base_ata = associated_token::get_associated_token_address(&pubkey, &base_mint);
        let quote_ata = associated_token::get_associated_token_address(&pubkey, &quote_mint);

        Ok(User {
            keypair,
            user_account,
            user_shares,
            base_ata,
            quote_ata,
            chamber: chamber_pubkey,
            shares_mint,
        })
    }

    #[allow(unused)]
    pub async fn create_user_account(
        &self,
        test_context: &mut TestContext,
    ) -> transport::Result<()> {
        let accounts = cetra_chamber::accounts::CreateUserAccount {
            chamber: self.chamber,
            user_account: self.user_account,
            user_shares: self.user_shares,
            shares_mint: self.shares_mint,
            user: self.keypair.pubkey(),
            rent_sysvar: rent::id(),
            token_program: token::ID,
            associated_token_program: associated_token::ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = cetra_chamber::instruction::CreateUserAccount {}.data();

        let tx = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: cetra_chamber::id(),
                data,
                accounts,
            }],
            Some(&self.keypair.pubkey()),
            &[&self.keypair],
            test_context.context.last_blockhash,
        );

        test_context.process_transaction(tx).await
    }

    #[allow(unused)]
    pub fn get_keypair(&self) -> Keypair {
        utils::clone_keypair(&self.keypair)
    }

    #[allow(unused)]
    pub fn get_shares(&self) -> Pubkey {
        self.user_shares
    }

    #[allow(unused)]
    pub fn get_pubkey(&self) -> Pubkey {
        self.keypair.pubkey()
    }

    #[allow(unused)]
    pub fn get_base_ata(&self) -> Pubkey {
        self.base_ata
    }

    #[allow(unused)]
    pub fn get_quote_ata(&self) -> Pubkey {
        self.quote_ata
    }

    #[allow(unused)]
    pub async fn fetch_user_account(
        &self,
        test_context: &mut TestContext,
    ) -> transport::Result<cetra_chamber::state::UserAccount> {
        let Some(account) = test_context
            .context
            .banks_client
            .get_account(self.user_account)
            .await? else {
                return Err(transport::TransportError::Custom("UserAccount is not found!".to_string()));
            };

        let mut account_data = &account.data[8..];

        let user_account = cetra_chamber::state::UserAccount::deserialize(&mut account_data)
            .expect("Unexpected invalid UserAccount layout!");

        Ok(user_account)
    }

    #[allow(unused)]
    pub fn get_user_account_pubkey(&self) -> Pubkey {
        self.user_account
    }
}
