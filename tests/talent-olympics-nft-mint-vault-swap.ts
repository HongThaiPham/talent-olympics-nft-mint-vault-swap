import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TalentOlympicsNftMintVaultSwap } from "../target/types/talent_olympics_nft_mint_vault_swap";
import { assert } from "chai";

import { fetchAssetV1, fetchCollection } from "@metaplex-foundation/mpl-core";
import { describe, it } from "node:test";
import {
  createNoopSigner,
  createSignerFromKeypair,
  keypairIdentity,
  publicKey,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

describe("talent-olympics-nft-mint-vault-swap", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  provider.opts.skipPreflight = true;
  provider.opts.commitment = "confirmed";
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
    admin: admin.publicKey.toBase58(),
    user1: user1.publicKey.toBase58(),
    user2: user2.publicKey.toBase58(),
  });

  const umi = createUmi(provider.connection.rpcEndpoint, "confirmed").use(
    mplTokenMetadata()
  );

  // const mySigner = createNoopSigner(admin.publicKey);

  const adminUmiKeypair = umi.eddsa.createKeypairFromSecretKey(admin.secretKey);
  umi.use(keypairIdentity(adminUmiKeypair));

  const [protocolAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  const [vaultAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault")],
    program.programId
  );

  const collection = anchor.web3.Keypair.generate();

  const aNft = anchor.web3.Keypair.generate();

  const collectionArgs = {
    name: "Solana Talent Olympics Collection 2024",
    uri: "https://ipfs.io/ipfs/QmQQYq41wkaAu5ekxv3xeDbSKyribYvHP8Pz7kPddYvvwB",
    plugins: [],
  };

  const assetArgs = {
    name: "Solana Talent Olympics NFT 2024",
    uri: "https://ipfs.io/ipfs/QmQQYq41wkaAu5ekxv3xeDbSKyribYvHP8Pz7kPddYvvwB",
    plugins: [],
  };

  it("Init test successfully", async () => {
    const tx = await provider.connection.requestAirdrop(
      admin.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(tx);

    const tx2 = await provider.connection.requestAirdrop(
      user1.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(tx2);

    const tx3 = await provider.connection.requestAirdrop(
      user2.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(tx3);
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
        updateAuthority: user1.publicKey,
      })
      .signers([user1, collection])
      .rpc();

    assert.ok(tx);

    const collectionData = await fetchCollection(
      umi,
      collection.publicKey.toString()
    );

    assert.equal(collectionData.name, collectionArgs.name);
    assert.equal(collectionData.uri, collectionArgs.uri);
    assert.equal(collectionData.numMinted, 0);
    assert.equal(collectionData.updateAuthority, user1.publicKey.toString());

    console.log("Collection created successfully at tx: ", tx);
  });

  it("Should mint a token successfully", async () => {
    const tx = await program.methods
      .mintNft(assetArgs)
      .accountsPartial({
        authority: user1.publicKey,
        asset: aNft.publicKey,
        collection: collection.publicKey,
        owner: null,
        updateAuthority: null,
        logWrapper: null,
      })
      .signers([user1, aNft])
      .rpc();

    assert.ok(tx);

    const assetData = await fetchAssetV1(
      umi,
      publicKey(aNft.publicKey.toString())
    );

    assert.equal(assetData.name, assetArgs.name);
    assert.equal(assetData.uri, assetArgs.uri);
    assert.equal(assetData.updateAuthority.type, "Collection");
    assert.equal(
      assetData.updateAuthority.address,
      collection.publicKey.toString()
    );

    const collectionData = await fetchCollection(
      umi,
      collection.publicKey.toString()
    );
    assert.equal(collectionData.numMinted, 1);

    console.log("Token minted successfully at tx: ", tx);
  });

  it("Should lock a nft successfully", async () => {
    const [lockerAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("locker"),
        aNft.publicKey.toBuffer(),
        user1.publicKey.toBuffer(),
      ],
      program.programId
    );
    const tx = await program.methods
      .lockNft()
      .accountsPartial({
        signer: user1.publicKey,
        asset: aNft.publicKey,
        collection: collection.publicKey,
        locker: lockerAccount,
        authority: null,
        logWrapper: null,
      })
      .signers([user1])
      .rpc();

    assert.ok(tx);

    const assetData = await fetchAssetV1(
      umi,
      publicKey(aNft.publicKey.toString())
    );

    assert.equal(assetData.owner.toString(), vaultAccount.toString());

    const vaultBalance = await provider.connection.getBalance(vaultAccount);
    assert.equal(vaultBalance, 1_000_000_000);

    console.log("Token locked successfully at tx: ", tx);
  });

  it("Should unlock a nft successfully", async () => {
    const [lockerAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("locker"),
        aNft.publicKey.toBuffer(),
        user1.publicKey.toBuffer(),
      ],
      program.programId
    );
    const tx = await program.methods
      .unlockNft()
      .accountsPartial({
        signer: user1.publicKey,
        asset: aNft.publicKey,
        collection: collection.publicKey,
        locker: lockerAccount,
        authority: null,
        logWrapper: null,
      })
      .signers([user1])
      .rpc();

    assert.ok(tx);

    const assetData = await fetchAssetV1(
      umi,
      publicKey(aNft.publicKey.toString())
    );

    assert.equal(assetData.owner.toString(), user1.publicKey.toString());

    console.log("Token unlocked successfully at tx: ", tx);
  });
});
