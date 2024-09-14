use crate::instructions::{LZ_RECEIVE_TYPES_SEED, OAPP_SEED, OWNER_SEED};
use crate::state::{OAppConfig, OAppLzReceiveTypesAccounts, VaultOwner};
use anchor_lang::prelude::*;

/// This instruction should always be in the same transaction as InitializeMint.
/// Otherwise, it is possible for your settings to be front-run by another transaction.
/// If such a case did happen, you should initialize another mint for this oapp.
#[derive(Accounts)]
#[instruction(params: InitOAppParams)]
pub struct InitOApp<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + OAppConfig::INIT_SPACE,
        seeds = [OAPP_SEED],
        bump
    )]
    pub oapp_config: Account<'info, OAppConfig>,
    #[account(
        init,
        payer = payer,
        space = 8 + OAppLzReceiveTypesAccounts::INIT_SPACE,
        seeds = [LZ_RECEIVE_TYPES_SEED, &oapp_config.key().as_ref()],
        bump
    )]
    pub lz_receive_types_accounts: Account<'info, OAppLzReceiveTypesAccounts>,
    #[account(
        init,
        payer = payer,
        space = 8 + VaultOwner::INIT_SPACE,
        seeds = [OWNER_SEED],
        bump
    )]
    pub vault_owner: Account<'info, VaultOwner>,
    pub system_program: Program<'info, System>,
}

impl InitOApp<'_> {
    pub fn apply(ctx: &mut Context<InitOApp>, params: &InitOAppParams) -> Result<()> {
        ctx.accounts.oapp_config.bump = ctx.bumps.oapp_config;

        ctx.accounts.lz_receive_types_accounts.oapp_config = ctx.accounts.oapp_config.key();

        ctx.accounts.oapp_config.usdc_hash = params.usdc_hash;
        ctx.accounts.oapp_config.usdc_mint = params.usdc_mint;
        ctx.accounts.vault_owner.owner = params.admin;
        ctx.accounts.vault_owner.bump = ctx.bumps.vault_owner;
        let oapp_signer = ctx.accounts.oapp_config.key();
        ctx.accounts.oapp_config.init(
            params.endpoint_program,
            params.admin,
            ctx.remaining_accounts,
            oapp_signer,
        )
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitOAppParams {
    pub admin: Pubkey,
    pub endpoint_program: Option<Pubkey>,
    pub usdc_hash: [u8; 32],
    pub usdc_mint: Pubkey,
}
