use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    errors::CustomError,
    state::{CurveConfiguration, LiquidityPool, LiquidityPoolAccount},
};

pub fn swap(ctx: Context<Swap>, amount: u64, style: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    let token_one_accounts = (
        &mut *ctx.accounts.mint_token_one.clone(),
        &mut *ctx.accounts.pool_token_account_one,
        &mut *ctx.accounts.user_token_account_one,
    );

    let token_two_accounts = (
        &mut *ctx.accounts.mint_token_one.clone(),
        &mut pool.to_account_info().clone(),
        &mut ctx.accounts.user.clone()
    );

    pool.swap(
        &*ctx.accounts.dex_configuration_account,
        token_one_accounts,
        token_two_accounts,
        amount,
        style,
        &ctx.accounts.user,
        &ctx.accounts.token_program,
        &ctx.accounts.system_program,
    )?;

     // Emit the event
     emit!(TokenSwap {
        user: ctx.accounts.user.key(),
        pool: ctx.accounts.pool.key(),
        token_mint: ctx.accounts.mint_token_one.key(),
        amount,
        style,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(
        mut,
        seeds = [CurveConfiguration::SEED.as_bytes()],
        bump,
    )]
    pub dex_configuration_account: Box<Account<'info, CurveConfiguration>>,

    #[account(
        mut,
        seeds = [LiquidityPool::POOL_SEED_PREFIX.as_bytes(), mint_token_one.key().as_ref()],
        bump = pool.bump
    )]
    pub pool: Box<Account<'info, LiquidityPool>>,

    #[account(mut)]
    pub mint_token_one: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_token_one,
        associated_token::authority = pool
    )]
    pub pool_token_account_one: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_token_one,
        associated_token::authority = user,
    )]
    pub user_token_account_one: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


#[event]
pub struct TokenSwap {
    pub user: Pubkey,
    pub pool: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub style: u64,
}