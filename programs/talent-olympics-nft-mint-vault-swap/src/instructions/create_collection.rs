use anchor_lang::prelude::*;
use mpl_core::types::PluginAuthorityPair;

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub collection: Signer<'info>,
    /// The MPL Core program.
    /// CHECK: Checked in mpl-core.
    #[account(address = mpl_core::ID)]
    pub mpl_core: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateCollectionV1Args {
    pub name: String,
    pub uri: String,
    pub plugins: Option<Vec<PluginAuthorityPair>>,
}

impl<'info> CreateCollection<'info> {
    pub fn handler(&mut self, args: CreateCollectionV1Args) -> Result<()> {
        mpl_core::instructions::CreateCollectionV1Cpi {
            collection: &self.collection.to_account_info(),
            payer: &self.payer.to_account_info(),
            update_authority: Some(&self.payer.to_account_info()),
            system_program: &self.system_program.to_account_info(),
            __program: &self.mpl_core,
            __args: mpl_core::instructions::CreateCollectionV1InstructionArgs {
                name: args.name,
                uri: args.uri,
                plugins: args.plugins,
            },
        }
        .invoke()?;
        Ok(())
    }
}
