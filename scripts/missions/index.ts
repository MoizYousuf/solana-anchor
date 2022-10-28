import fs from "fs";
import {
  Program,
  AnchorProvider,
  setProvider,
  web3,
  Wallet,
  BN,
} from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import key from "/Users/arbazyousuf/.config/solana/id.json";
import { NewProject, IDL } from "../../target/types/new_project";
import { MerkleTree } from "../helpers/merkleTree";
import { buildLeaves, getDeployments, setDeployments } from "../helpers";

const devnetConfig = {
  network: "devnet",
  endpoint: "https://api.devnet.solana.com",
  programId: new PublicKey(
    "3dY4fgav22wK1PvFY6BadcAqX8NrYQH4CPMRLdRhVyQV"
  ),
};
const mainnetConfig = {
  network: "mainnet-beta",
  endpoint: "https://api.mainnet-beta.solana.com",
  programId: new PublicKey(
    "HkoqNP2KcgiH5bzWWaSUFA8AsGyb7M9zRyJEhKnezSrv"
  ),
};

export const getDependencies = (network: "mainnet" | "devnet") => {
  const config = network === "mainnet" ? mainnetConfig : devnetConfig;

  const wallet = new Wallet(web3.Keypair.fromSecretKey(Uint8Array.from(key)));
  const provider = new AnchorProvider(
    new web3.Connection(config.endpoint),
    wallet,
    {
      preflightCommitment: "confirmed",
    }
  );
  setProvider(provider);


  return {
    config,
    provider,
    payer: wallet.payer,
    program: new Program<NewProject>(
      IDL,
      config.programId,
      provider
    ),
    getDeployments: () => getDeployments("missions", network),
    setDeployments: (deployments) =>
      setDeployments("missions", network, deployments),
  };
};
