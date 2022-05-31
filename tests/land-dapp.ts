import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LandDapp } from "../target/types/land_dapp";
const assert = require("assert");
const { SystemProgram } = anchor.web3;
describe("land-dapp", () => {
  // Configure the client to use the local cluster.
  

  const program = anchor.workspace.LandDapp as Program<LandDapp>;
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.rpc.initialize({});
  //   console.log("Your transaction signature", tx);
  // });
  it("register a new user", async () => {
    const user = anchor.web3.Keypair.generate();

    await program.rpc.registerUser("zunaira", "https://img.link", {
      accounts: {
        authority: provider.wallet.publicKey,
        userAccount: user.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [user],
    });
  
    const userAccount = await program.account.user.fetch(user.publicKey);
    console.log("register",userAccount);
    assert.equal(userAccount.name, "zunaira");
    assert.equal(userAccount.profile, "https://img.link");

    assert.equal(
      userAccount.authority.toString(),
      provider.wallet.publicKey.toString()
    );
  });
  it("update user", async () => {
    const user = anchor.web3.Keypair.generate();

    await program.rpc.registerUser("zunaira javed", "https://img.link", {
      accounts: {
        authority: provider.wallet.publicKey,
        userAccount: user.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [user],
    });
  
    const userAccount = await program.account.user.fetch(user.publicKey);
    console.log("update",userAccount);
    assert.equal(userAccount.name, "zunaira javed");
    assert.equal(userAccount.profile, "https://img.link");

    assert.equal(
      userAccount.authority.toString(),
      provider.wallet.publicKey.toString()
    );
  });


});
