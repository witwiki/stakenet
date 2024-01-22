use anchor_lang::prelude::*;

use crate::Config;
use crate::logs::LogInitializeConfig;

#[event_cpi]
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = signer,
        space = Config::SIZE,
        seeds = [Config::SEED],
        bump,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn handler(ctx: Context<InitializeConfig>, authority: Pubkey) -> Result<()> {
    ctx.accounts.config.oracle_authority = authority;
    ctx.accounts.config.admin = authority;
    ctx.accounts.config.bump = *ctx.bumps.get("config").unwrap();
    ctx.accounts.config.counter = 0;

    emit_cpi!(LogInitializeConfig {
        config: ctx.accounts.config.key(),
        signer: ctx.accounts.signer.owner.key(),
        oracle_authority: ctx.accounts.config.oracle_authority,
        admin: ctx.accounts.config.admin,
        bump: ctx.accounts.config.bump,
        counter: ctx.accounts.config.counter        
    });

    Ok(())
}

