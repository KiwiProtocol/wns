import * as anchor from "@project-serum/anchor";
import { Connection, Keypair } from "@solana/web3.js";
import { readFileSync } from "fs";

async function main() {
  // Set up provider and connection
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Read program ID from deployment
  const idl = JSON.parse(readFileSync("./target/idl/wns.json", "utf8"));
  const programId = new anchor.web3.PublicKey("YourProgramID"); // Replace with your program ID

  const program = new anchor.Program(idl, programId, provider);

  // Initialize the program (you can add custom initialization logic here)
  await program.rpc.initialize({});
  console.log("Program deployed successfully!");
}

main().catch((err) => {
  console.error(err);
});
