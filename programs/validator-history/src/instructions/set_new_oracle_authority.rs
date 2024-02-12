use anchor_lang::prelude::*;

use crate::state::Config;
use crate::logs::LogSetNewOracleAuthority;

#[event_cpi]
#[derive(Accounts)]
pub struct SetNewOracleAuthority<'info> {
    #[account(
        mut,
        seeds = [Config::SEED],
        bump = config.bump,
        has_one = admin,
    )]
    pub config: Account<'info, Config>,
    /// CHECK: fine since we are not deserializing account
    pub new_oracle_authority: AccountInfo<'info>,
    pub admin: Signer<'info>,
}

pub fn handle_set_new_oracle_authority(ctx: Context<SetNewOracleAuthority>) -> Result<()> {
    ctx.accounts.config.oracle_authority = ctx.accounts.new_oracle_authority.key();

    emit_cpi!(LogSetNewOracleAuthority {
        config: ctx.accounts.config.key(),
        new_oracle_authority: ctx.accounts.new_oracle_authority.key(),
        admin: ctx.accounts.admin.owner.key()
    });

    Ok(())
}
