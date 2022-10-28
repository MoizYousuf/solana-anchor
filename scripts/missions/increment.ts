import { getDependencies } from ".";
import * as web3 from "@solana/web3.js";
import * as spl_token from "@solana/spl-token";

const fn = async (network: string) => {
  if (network !== "mainnet" && network !== "devnet")
    throw new Error("Unknown network!");

  const {
    provider,
    config,
    payer,
    program,
    getDeployments,
    setDeployments,
  } = getDependencies(network);

  let deployments: any = {};
  try {
    deployments = getDeployments();
  } catch (e) {}

  const counterKey = new web3.PublicKey(deployments.counter)
    const [counter] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("counter", "utf8"),
        counterKey.toBuffer(),
      ],
      program.programId
    );

  const [vault] = await web3.PublicKey.findProgramAddress(
    [
      Buffer.from("vault", "utf8"),
      counterKey.toBuffer(),
    ],
    program.programId
  );

  const accounts = {
    counter,
    vault,
    payer: provider.wallet.publicKey,
  }

  Object.entries(accounts).forEach(([key, value]) => console.log(`${key}: ${value.toString()}`))

    const txId = await program.methods
      .increment()
      .accounts(accounts).rpc()
    console.log("======= Project Incrmented =======");
    console.log(`Transaction Signature: ${txId}`);

  setDeployments({
    program: program.programId.toString(),
    counter: counterKey.toString(),
  });
};

fn(process.argv[2]);
