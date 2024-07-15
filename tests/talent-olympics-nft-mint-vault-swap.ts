import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TalentOlympicsNftMintVaultSwap } from "../target/types/talent_olympics_nft_mint_vault_swap";
import { assert } from "chai";

describe("talent-olympics-nft-mint-vault-swap", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  // provider.opts.skipPreflight = true;
  // provider.opts.commitment = "confirmed";
  anchor.setProvider(provider);

  const program = anchor.workspace
    .TalentOlympicsNftMintVaultSwap as Program<TalentOlympicsNftMintVaultSwap>;

  const FEE = new anchor.BN(1_00_000_000);

  const [admin, user1, user2] = [
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
  ];

  console.table({
    poolAuthor: admin.publicKey.toBase58(),
    user1: user1.publicKey.toBase58(),
    user2: user2.publicKey.toBase58(),
  });

  const [protocolAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  const collection = anchor.web3.Keypair.generate();
  const collectionArgs = {
    name: "Solana Talent Olympics Collection 2024",
    uri: "https://ipfs.io/ipfs/QmQQYq41wkaAu5ekxv3xeDbSKyribYvHP8Pz7kPddYvvwB",
    plugins: [],
  };

  before(async () => {
    {
      const tx = await provider.connection.requestAirdrop(
        admin.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(tx);

      const tx2 = await provider.connection.requestAirdrop(
        user1.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(tx2);

      const tx3 = await provider.connection.requestAirdrop(
        user2.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(tx3);
    }
  });

  it("Should init protocol successfully", async () => {
    const tx = await program.methods
      .initialize(FEE)
      .accounts({
        signer: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    assert.ok(tx);

    console.log("Protocol initialized successfully at tx: ", tx);
  });

  it("Should update fee successfully", async () => {
    const tx = await program.methods
      .setFee(new anchor.BN(1_000_000_000))
      .accounts({
        signer: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    assert.ok(tx);

    console.log("Fee updated successfully at tx: ", tx);
  });

  it("Should create a collection successfully", async () => {
    const tx = await program.methods
      .createCollection(collectionArgs)
      .accountsPartial({
        payer: user1.publicKey,
        collection: collection.publicKey,
      })
      .signers([user1, collection])
      .rpc();

    assert.ok(tx);

    console.log("Collection created successfully at tx: ", tx);
  });
});
