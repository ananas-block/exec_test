import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ExecTest } from "../target/types/exec_test";
import { Keypair, Transaction, Signer } from "@solana/web3.js";
import * as solana from "@solana/web3.js";

describe("exec_test", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ExecTest as Program<ExecTest>;

  it("Is initialized!", async () => {
    const signer = Keypair.generate();
    const feePayerPda = solana.PublicKey.findProgramAddressSync([signer.publicKey.toBuffer()], program.programId)[0]; //deriveProgramAddress()
    const provider = anchor.getProvider();
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(feePayerPda, 1_000_000), "confirmed");
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(signer.publicKey, 1_000_000), "confirmed");
    console.log("signer ", signer.publicKey);
    console.log("feePayerPda ", feePayerPda);

    // Add your test here.
    const tx = await program.methods.initialize().accounts({
    }).rpc();
    console.log(tx);
    console.log("Your transaction signature", tx);
  });
});
