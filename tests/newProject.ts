import assert from "assert";
import {
  AnchorProvider,
  setProvider,
  web3
} from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";
const { SystemProgram } = web3;
const userAccount = anchor.web3.Keypair.generate();
describe("basic-1", () => {
  // Use a local provider.
  const provider = AnchorProvider.env();

  // Configure the client to use the local cluster.
  setProvider(provider);

  it("Creates and initializes an account in a single atomic transaction (simplified)", async () => {
    // #region code-simplified
    // The program to execute.
    const program = anchor.workspace.NewProject;

    // The Account to create.

    // Create the new account and initialize it with the program.
    // #region code-simplified

    // await program.methods
    // .initialize(new anchor.BN(1234))
    // .accounts({
    //   userAccount: userAccount.publicKey,
    //   user: provider.wallet.publicKey,
    //   systemProgram: SystemProgram.programId,
    // })
    // .signers([userAccount])
    // .rpc();

    await program.rpc.initialize(new anchor.BN(1234), {
      accounts: {
        userAccount: userAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [userAccount],
    });
    // #endregion code-simplified

    // Fetch the newly created account from the cluster.
    const account = await program.account.userData.fetch(userAccount.publicKey);

    // Check it's state was initialized.
    assert.ok(account.data.eq(new anchor.BN(1234)));
  });

  it("Updates a previously created account", async () => {

    // #region update-test
    // The program to execute.
    const program = anchor.workspace.NewProject;

    // Invoke the update rpc.
    await program.rpc.update(new anchor.BN(4321), {
      accounts: {
        userAccount: userAccount.publicKey,
      },
    });

    // Fetch the newly updated account.
    const account = await program.account.userData.fetch(userAccount.publicKey);

    // Check it's state was mutated.
    assert.ok(account.data.eq(new anchor.BN(4321)));

    // #endregion update-test
  });
});