use anchor_lang::prelude::*;

use crate::state::Config;
use crate::logs::LogSetNewAdmin;

#[event_cpi]
#[derive(Accounts)]
pub struct SetNewAdmin<'info> {
    #[account(
        mut,
        seeds = [Config::SEED],
        bump = config.bump,
        has_one = admin,
    )]
    pub config: Account<'info, Config>,
    /// CHECK: fine since we are not deserializing account
    pub new_admin: AccountInfo<'info>,
    pub admin: Signer<'info>,
}

pub fn handler(ctx: Context<SetNewAdmin>) -> Result<()> {
    ctx.accounts.config.admin = ctx.accounts.new_admin.key();

    emit_cpi!(LogSetNewAdmin {
        config: ctx.accounts.config.key(),
        new_admin: ctx.accounts.new_admin.key(),
        admin: ctx.accounts.admin.owner.key()
    });

    Ok(())
}
