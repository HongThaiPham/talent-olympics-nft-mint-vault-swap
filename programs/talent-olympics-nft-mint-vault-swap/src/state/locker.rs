use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Locker {
    pub owner: Pubkey,
    pub asset: Pubkey,
    pub locked_at: i64,
}
