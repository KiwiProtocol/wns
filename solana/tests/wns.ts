import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { assert } from "chai";
import { Wns } from "../target/types/wns";

describe("Wormhole Name Service", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.Wns as Program<Wns>;

  const owner = Keypair.generate();
  const snsDomain = "alice.sol";
  const wnsDomain = "alice.wns";

  it("Initializes the program", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Transaction signature", tx);
  });

  it("Registers a new WNS domain", async () => {
    const tx = await program.methods.registerName(wnsDomain, owner.publicKey).rpc();
    console.log("Transaction signature", tx);

    const registryAccount = await program.account.nameRegistry.fetch(
      new PublicKey(wnsDomain)
    );
    assert.equal(registryAccount.owner.toString(), owner.publicKey.toString());
  });

  it("Renews an existing WNS domain", async () => {
    const tx = await program.methods.renewName(wnsDomain).rpc();
    console.log("Transaction signature", tx);

    const registryAccount = await program.account.nameRegistry.fetch(
      new PublicKey(wnsDomain)
    );
    assert.isAbove(registryAccount.expiration, Date.now() / 1000);
  });

  it("Automatically registers a WNS domain from an SNS domain", async () => {
    const tx = await program.methods.registerWnsFromSns(snsDomain).rpc();
    console.log("Transaction signature", tx);

    const registryAccount = await program.account.nameRegistry.fetch(
      new PublicKey(`${snsDomain}.wns`)
    );
    assert.equal(registryAccount.owner.toString(), owner.publicKey.toString());
  });

  it("Resolves a WNS domain", async () => {
    const resolvedOwner = await program.methods.resolveName(wnsDomain).rpc();
    assert.equal(resolvedOwner.toString(), owner.publicKey.toString());
  });
});