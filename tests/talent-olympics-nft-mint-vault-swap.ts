import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TalentOlympicsNftMintVaultSwap } from "../target/types/talent_olympics_nft_mint_vault_swap";

describe("talent-olympics-nft-mint-vault-swap", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TalentOlympicsNftMintVaultSwap as Program<TalentOlympicsNftMintVaultSwap>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
