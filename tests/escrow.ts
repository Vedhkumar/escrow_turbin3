import * as anchor from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import {
  Account,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { assert } from "chai";
describe("test", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Escrow as anchor.Program<Escrow>;

  const maker = anchor.web3.Keypair.generate();
  let makerAATA: Account;

  const mintAAuthority = anchor.web3.Keypair.generate();
  let mintA: anchor.web3.PublicKey;
  const mintADecimals = 9;

  const vault = anchor.web3.Keypair.generate();

  const mintBAuthority = anchor.web3.Keypair.generate();
  let mintB: anchor.web3.PublicKey;
  const mintBDecimals = 9;

  const escrow_id = new anchor.BN(1);
  const [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("escrow"),
      maker.publicKey.toBuffer(),
      new anchor.BN(escrow_id).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );
  before(async () => {
    // Airdrop SOL to the maker account
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        maker.publicKey,
        anchor.web3.LAMPORTS_PER_SOL * 10
      )
    );

    // Airdrop SOL to the mint A authority account
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        mintAAuthority.publicKey,
        anchor.web3.LAMPORTS_PER_SOL * 10
      )
    );
    // Airdrop SOL to the mint B authority account
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        mintBAuthority.publicKey,
        anchor.web3.LAMPORTS_PER_SOL * 10
      )
    );

    // Create mint accounts
    mintA = await createMint(
      provider.connection,
      mintAAuthority,
      mintAAuthority.publicKey,
      null,
      mintADecimals,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );
    makerAATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      maker,
      mintA,
      maker.publicKey
    );
    await mintTo(
      provider.connection,
      mintAAuthority,
      mintA,
      makerAATA.address,
      mintAAuthority,
      100 * 10 ** mintADecimals
    );
    mintB = await createMint(
      provider.connection,
      mintBAuthority,
      mintBAuthority.publicKey,
      null,
      mintBDecimals,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );
  });

  it("Initialize make", async () => {
    const amountA = new anchor.BN(10 * 10 ** mintADecimals);
    const amountBWanted = new anchor.BN(100 * 10 ** mintBDecimals);

    const makerAATABalance = makerAATA.amount;
    console.log("Maker AATA Balance before: ", makerAATABalance);
    // const ata = await provider.connection.getAccountInfo(TOKEN_PROGRAM_ID);
    // console.log(ata);
    try {
      await program.methods
        .make(escrow_id, amountA, amountBWanted)
        .accountsPartial({
          maker: maker.publicKey,
          mintA: mintA,
          mintB: mintB,
          makerTokenAAccount: makerAATA.address,
          // vault: vault.publicKey, //? why we should not pass the vault?
          escrow: escrow,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([maker])
        .rpc();
    } catch (error) {
      console.log("ERROR");
      console.log(await error.getLogs());
    }

    const escrow_account = await program.account.escrow.fetch(escrow);
    console.log(escrow_account);

    assert(escrow_account.amountA == amountA, "Invalid amountA");
    assert(
      escrow_account.amountBWanted == amountBWanted,
      "Invalid amountBWanted"
    );
    assert(escrow_account.escrowId == escrow_id, "Invalid escrowId");
    assert(escrow_account.maker == maker.publicKey, "Invalid maker");
    assert(escrow_account.mintA == mintA, "Invalid mintA");
    assert(escrow_account.mintB == mintB, "Invalid mintB");
  });
});
