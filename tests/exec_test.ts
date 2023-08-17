import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
// import { ExecTest } from "../target/types/exec_test";
import { Keypair, Transaction, Signer } from "@solana/web3.js";
import * as solana from "@solana/web3.js";
describe("exec_test", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const IDL: ExecTest = {
    "version": "0.1.0",
    "name": "exec_test",
    "instructions": [
      {
        "name": "initialize",
        "accounts": [],
        "args": []
      }
    ]
  };
  
  type ExecTest = {
    "version": "0.1.0",
    "name": "exec_test",
    "instructions": [
      {
        "name": "initialize",
        "accounts": [],
        "args": []
      }
    ]
  };
  const program = new Program(IDL, new solana.PublicKey("Esqmi51Gc2EGie1b3uGqZyqY86rakzCLk1gv4D9x2Lwp"));

  it("Is initialized!", async () => {
    // program.programId = new solana.PublicKey("DUUot4WvRmSkRgKmpm7VWgQMkNbqcNa9QLkNr7ueTio4");
    const signer = Keypair.generate();
    const feePayerPda = solana.PublicKey.findProgramAddressSync([signer.publicKey.toBuffer()], program.programId)[0]; //deriveProgramAddress()
    const provider = anchor.getProvider();
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(provider.publicKey, 1_000_000), "confirmed");
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(feePayerPda, 1_000_000), "confirmed");
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(signer.publicKey, 1_000_000), "confirmed");
    console.log("signer ", signer.publicKey);
    console.log("feePayerPda ", feePayerPda);

    // Add your test here.
    try {
      const tx = await program.methods.initialize().preInstructions([
        solana.ComputeBudgetProgram.setComputeUnitLimit({ units: 1_200_000 }),
      ]).accounts({
      }).rpc();
      console.log(tx);
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log("error ", error);
    }


  });
});

