pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2K2jn1PU9TjxmL3PkmRRCMs1pvaqZEbGjSyHbpqdRLdT");

#[program]
pub mod talent_olympics_nft_mint_vault_swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u64) -> Result<()> {
        ctx.accounts.handler(fee)
    }

    pub fn set_fee(ctx: Context<SetFee>, fee: u64) -> Result<()> {
        ctx.accounts.handler(fee)
    }

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        args: CreateCollectionV1Args,
    ) -> Result<()> {
        ctx.accounts.handler(args)
    }

    pub fn mint_nft(ctx: Context<MintNft>, args: CreateAssetArgs) -> Result<()> {
        ctx.accounts.handler(args)
    }

    pub fn lock_nft(ctx: Context<LockNft>) -> Result<()> {
        ctx.accounts.handler()
    }
}
