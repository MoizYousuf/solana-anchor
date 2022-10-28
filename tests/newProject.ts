import assert from "assert";
import {
  AnchorProvider,
  setProvider,
  web3
} from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";
const { SystemProgram } = web3;

const program = anchor.workspace.NewProject;
const [vault] = web3.PublicKey.findProgramAddressSync([
  Buffer.from("vault")
], program.programId)

describe("basic-2", () => {
  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  // Counter for the tests.
  const counter = anchor.web3.Keypair.generate();

  // Program for the tests.




  it("Creates a counter", async () => {
    await program.rpc.initialize({
      accounts: {
        counter: counter.publicKey,
        user: provider.wallet.publicKey,
        vault,
        systemProgram: SystemProgram.programId,
      },
      signers: [counter],
    });

    let counterAccount = await program.account.counter.fetch(counter.publicKey);

    assert.ok(counterAccount.authority.equals(provider.wallet.publicKey));
    assert.ok(counterAccount.count.toNumber() === 0);
  });

  it("Updates a counter", async () => {
    await program.rpc.increment({
      accounts: {
        counter: counter.publicKey,
        payer: provider.wallet.publicKey,
        vault
      },
    });

    const counterAccount = await program.account.counter.fetch(
      counter.publicKey
    );

    assert.ok(counterAccount.authority.equals(provider.wallet.publicKey));
    assert.ok(counterAccount.count.toNumber() == 1);
  });


  it("decrement in a counter", async () => {
    await program.rpc.decrement({
      accounts: {
        counter: counter.publicKey,
        payer: provider.wallet.publicKey,
        vault
      },
    });


    const counterAccount = await program.account.counter.fetch(
      counter.publicKey
    );

    assert.ok(counterAccount.authority.equals(provider.wallet.publicKey));
    assert.ok(counterAccount.count.toNumber() == 0);
  });
});