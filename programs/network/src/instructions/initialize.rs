use {
    crate::state::*,
    anchor_lang::{
        prelude::*, 
        solana_program::{system_program, sysvar}
    },
    anchor_spl::token::{Mint, Token},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    config_bump: u8,
    pool_bump: u8,
    registry_bump: u8,
    snapshot_bump: u8,
)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        init,
        seeds = [SEED_CONFIG],
        bump,
        payer = admin,
        space = 8 + size_of::<Config>(),
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 7,
        mint::authority = admin,
        mint::freeze_authority = admin
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        seeds = [SEED_POOL],
        bump,
        payer = admin,
        space = 8 + size_of::<Pool>(),
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        seeds = [SEED_REGISTRY],
        bump,
        payer = admin,
        space = 8 + size_of::<Registry>(),
    )]
    pub registry: Account<'info, Registry>,
 
    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT, 
            (0 as u64).to_be_bytes().as_ref()
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<Snapshot>(),
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<Initialize>,
    config_bump: u8,
    pool_bump: u8,
    registry_bump: u8,
    snapshot_bump: u8,
) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let clock = &ctx.accounts.clock;
    let config = &mut ctx.accounts.config;
    let mint = &ctx.accounts.mint;
    let pool = &mut ctx.accounts.pool;
    let registry = &mut ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;

    config.new(admin.key(), config_bump, mint.key())?;
    pool.new(pool_bump)?;
    registry.new(registry_bump)?;
    registry.new_snapshot(snapshot, snapshot_bump)?;
    registry.rotate_snapshot(clock, None, snapshot)?;

    Ok(())
}
