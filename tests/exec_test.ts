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
  const program = new Program(IDL, new solana.PublicKey("DzLsmqhzHC3U5pFS1vbUXZFRuTeTC9ZTjNcc3STRX5Va"));

  it("Is initialized!", async () => {
    // program.programId = new solana.PublicKey("DUUot4WvRmSkRgKmpm7VWgQMkNbqcNa9QLkNr7ueTio4");
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

