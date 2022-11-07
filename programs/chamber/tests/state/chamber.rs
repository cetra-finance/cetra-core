use super::User;
use crate::utils;
use anchor_lang::{prelude::AccountMeta, AnchorDeserialize, InstructionData, ToAccountMetas};
use anchor_spl::{associated_token, token};
use cetra_program_test::{
    solana_sdk::{
        instruction::Instruction,
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        system_program,
        sysvar::{clock, rent},
        transaction::Transaction,
        transport,
    },
    TestContext,
};
use tulipv2_sdk_common::config::levfarm as tulip_levfarm_config;
use tulipv2_sdk_levfarm::accounts::{
    self as tulip_levfarm_accounts, derivations as tulip_levfarm_derivations,
};

pub struct Chamber {
    farm: Pubkey,
    farm_config: tulip_levfarm_config::LevFarmConfig,
    farm_type: tulip_levfarm_accounts::Farms,

    base_mint: Pubkey,
    quote_mint: Pubkey,
    shares_mint: Pubkey,

    base_ata: Pubkey,
    quote_ata: Pubkey,

    base_oracle: Pubkey,
    quote_oracle: Pubkey,

    chamber: Pubkey,
    authority: Pubkey,
    fee_manager: Pubkey,
    owner: Keypair,

    leverage: u64,
    is_base_volatile: bool,
    #[allow(unused)]
    bump: u8,
    authority_bump: u8,
    nonce: u8,
}

impl Chamber {
    pub async fn build_raydium_sol_usdc(
        test_context: &mut TestContext,
        payer: &Keypair,
        owner: &Keypair,
        fee_manager: &Pubkey,
        chamber_nonce: u8,
    ) -> transport::Result<Self> {
        let farm_config = tulip_levfarm_config::ray_solusdc::get_lev_farm_config();
        let farm = farm_config.account;

        let base_mint = farm_config.base_token_mint;
        let quote_mint = farm_config.quote_token_mint;

        let (chamber_pubkey, chamber_bump) = cetra_chamber::utils::derive_chamber_address(
            &farm,
            &base_mint,
            &quote_mint,
            chamber_nonce,
        );
        let (chamber_authority_pubkey, chamber_authority_bump) =
            cetra_chamber::utils::derive_chamber_authority_address(&chamber_pubkey);

        let base_ata =
            associated_token::get_associated_token_address(&chamber_authority_pubkey, &base_mint);
        let quote_ata =
            associated_token::get_associated_token_address(&chamber_authority_pubkey, &quote_mint);

        let base_oracle = farm_config.coin_price_account;
        let quote_oracle = farm_config.pc_price_account;

        let shares_mint_keypair = Keypair::new();
        utils::create_token_mint(test_context, payer, &shares_mint_keypair).await?;

        Ok(Chamber {
            farm,
            farm_config,
            farm_type: tulip_levfarm_accounts::Farms::SolUsdcRayVault,
            base_mint,
            quote_mint,
            shares_mint: shares_mint_keypair.pubkey(),
            base_ata,
            quote_ata,
            base_oracle,
            quote_oracle,
            chamber: chamber_pubkey,
            authority: chamber_authority_pubkey,
            fee_manager: *fee_manager,
            owner: utils::clone_keypair(owner),
            leverage: 3,
            is_base_volatile: true,
            bump: chamber_bump,
            authority_bump: chamber_authority_bump,
            nonce: chamber_nonce,
        })
    }

    pub async fn initialize_chamber(
        &self,
        test_context: &mut TestContext,
        payer: &Keypair,
    ) -> transport::Result<()> {
        let mut accounts = cetra_chamber::accounts::InitializeChamber {
            chamber: self.chamber,
            farm: self.farm,
            base_token: self.base_ata,
            quote_token: self.quote_ata,
            base_mint: self.base_mint,
            quote_mint: self.quote_mint,
            shares_mint: self.shares_mint,
            base_oracle: self.base_oracle,
            quote_oracle: self.quote_oracle,
            authority: self.authority,
            fee_manager: self.fee_manager,
            farm_program: tulipv2_sdk_levfarm::ID,
            payer: payer.pubkey(),
            owner: self.owner.pubkey(),
            clock_sysvar: clock::id(),
            rent_sysvar: rent::id(),
            token_program: token::ID,
            associated_token_program: associated_token::ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let (user_farm, _) = self.derive_user_farm(0);
        let (user_farm_obligation, _) = self.derive_user_farm_obligation(0, 0);
        let (user_farm_obligation_1, _) = self.derive_user_farm_obligation(0, 1);
        let (user_farm_obligation_vault, _) = self.derive_user_farm_obligation_vault(0, 0);
        let (user_farm_obligation_vault_1, _) = self.derive_user_farm_obligation_vault(0, 1);

        accounts.extend(vec![
            AccountMeta::new(user_farm, false),
            AccountMeta::new(user_farm_obligation, false),
            AccountMeta::new(user_farm_obligation_1, false),
            AccountMeta::new(self.farm_config.lending_market, false),
            AccountMeta::new(user_farm_obligation_vault, false),
            AccountMeta::new(user_farm_obligation_vault_1, false),
            AccountMeta::new_readonly(self.farm_config.global, false),
            AccountMeta::new_readonly(self.farm_config.lending_program, false),
            AccountMeta::new_readonly(self.farm_config.solfarm_vault_program, false),
        ]);

        let data = cetra_chamber::instruction::InitializeChamber {
            market: cetra_chamber::state::ChamberMarket::Tulip,
            leverage: self.leverage,
            is_base_volatile: self.is_base_volatile,
            chamber_nonce: self.nonce,
            authority_bump: self.authority_bump,
        }
        .data();

        let tx = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: cetra_chamber::id(),
                data,
                accounts,
            }],
            Some(&payer.pubkey()),
            &[payer, &self.owner],
            test_context.context.last_blockhash,
        );

        test_context.process_transaction(tx).await
    }

    /// TODO: Implement with address lookup table support.
    #[allow(unused)]
    pub async fn deposit_chamber(
        &self,
        _test_context: &mut TestContext,
        user: &User,
        base_amount: u64,
        quote_amount: u64,
    ) -> transport::Result<()> {
        let _user_keypair = user.get_keypair();

        let _accounts = cetra_chamber::accounts::DepositChamber {
            chamber: self.chamber,
            user_account: user.get_user_account_pubkey(),
            user_shares: user.get_shares(),
            user_base_token: user.get_base_ata(),
            user_quote_token: user.get_quote_ata(),
            chamber_shares_mint: self.shares_mint,
            chamber_base_token: self.base_ata,
            chamber_quote_token: self.quote_ata,
            chamber_base_oracle: self.base_oracle,
            chamber_quote_oracle: self.quote_oracle,
            chamber_authority: self.authority,
            chamber_farm_program: tulipv2_sdk_levfarm::ID,
            user: user.get_pubkey(),
            clock_sysvar: clock::id(),
            rent_sysvar: rent::id(),
            token_program: token::ID,
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let _data = cetra_chamber::instruction::DepositChamber {
            base_amount,
            quote_amount,
        }
        .data();

        Ok(())
    }

    /// TODO: Implement with address lookup table support.
    #[allow(unused)]
    pub async fn withdraw_chamber(
        &self,
        _test_context: &mut TestContext,
        _user: &User,
        _base_amount: u64,
        _quote_amount: u64,
    ) -> transport::Result<()> {
        Ok(())
    }

    #[allow(unused)]
    pub async fn fetch_chamber(
        &self,
        test_context: &mut TestContext,
    ) -> transport::Result<cetra_chamber::state::Chamber> {
        let Some(account) = test_context
            .context
            .banks_client
            .get_account(self.chamber)
            .await? else {
                return Err(transport::TransportError::Custom("Chamber is not found!".to_string()));
            };

        let mut account_data = &account.data[8..];

        let user_account = cetra_chamber::state::Chamber::deserialize(&mut account_data)
            .expect("Unexpected invalid Chamber layout!");

        Ok(user_account)
    }

    #[allow(unused)]
    pub fn get_pubkey(&self) -> Pubkey {
        self.chamber
    }

    #[allow(unused)]
    pub fn get_base_mint(&self) -> Pubkey {
        self.base_mint
    }

    #[allow(unused)]
    pub fn get_quote_mint(&self) -> Pubkey {
        self.quote_mint
    }

    #[allow(unused)]
    pub fn get_shares_mint(&self) -> Pubkey {
        self.shares_mint
    }

    fn derive_user_farm(&self, index: u64) -> (Pubkey, u8) {
        tulip_levfarm_derivations::derive_user_farm_address(
            self.authority,
            tulipv2_sdk_levfarm::ID,
            index,
            self.farm_type,
        )
    }

    fn derive_user_farm_obligation(&self, index: u64, obligation_index: u64) -> (Pubkey, u8) {
        tulip_levfarm_derivations::derive_user_farm_obligation_address(
            self.authority,
            self.derive_user_farm(index).0,
            tulipv2_sdk_levfarm::ID,
            obligation_index as u8,
        )
    }

    fn derive_user_farm_obligation_vault(&self, index: u64, obligation_index: u64) -> (Pubkey, u8) {
        tulip_levfarm_derivations::derive_user_farm_obligation_vault_address(
            self.derive_user_farm(index).0,
            tulipv2_sdk_levfarm::ID,
            obligation_index as u8,
        )
    }
}
