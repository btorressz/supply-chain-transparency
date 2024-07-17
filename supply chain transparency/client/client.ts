import * as web3 from "@solana/web3.js";
import * as anchor from '@coral-xyz/anchor';
import { Program, AnchorProvider, Wallet, web3 } from '@coral-xyz/anchor';
import { SupplyChainTransparency } from '../target/types/supply_chain_transparency';
import type { SupplyChainTransparency } from "../target/types/supply_chain_transparency";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SupplyChainTransparency as anchor.Program<SupplyChainTransparency>;


const { SystemProgram } = web3;

export async function initializeTraceability(
  provider: AnchorProvider,
  program: Program<SupplyChainTransparency>,
  product_name: string,
  traceabilityAccount: web3.Keypair
) {
  await program.methods
    .initializeTraceability(product_name)
    .accounts({
      traceabilityAccount: traceabilityAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([traceabilityAccount])
    .rpc();
}

export async function updateTraceability(
  provider: AnchorProvider,
  program: Program<SupplyChainTransparency>,
  traceabilityAccount: web3.PublicKey,
  stage: string
) {
  await program.methods
    .updateTraceability(stage)
    .accounts({
      traceabilityAccount: traceabilityAccount,
      owner: provider.wallet.publicKey,
    })
    .rpc();
}

export async function addCertification(
  provider: AnchorProvider,
  program: Program<SupplyChainTransparency>,
  traceabilityAccount: web3.PublicKey,
  certification: string
) {
  await program.methods
    .addCertification(certification)
    .accounts({
      traceabilityAccount: traceabilityAccount,
      owner: provider.wallet.publicKey,
    })
    .rpc();
}

export async function closeTraceability(
  provider: AnchorProvider,
  program: Program<SupplyChainTransparency>,
  traceabilityAccount: web3.PublicKey,
  recipient: web3.PublicKey
) {
  await program.methods
    .closeTraceability()
    .accounts({
      traceabilityAccount: traceabilityAccount,
      recipient: recipient,
      owner: provider.wallet.publicKey,
    })
    .rpc();
}

export async function getStages(
  program: Program<SupplyChainTransparency>,
  traceabilityAccount: web3.PublicKey,
  start: number,
  end: number
) {
  return await program.methods
    .getStages(start, end)
    .accounts({
      traceabilityAccount: traceabilityAccount,
    })
    .rpc();
}

export async function getCertifications(
  program: Program<SupplyChainTransparency>,
  traceabilityAccount: web3.PublicKey,
  start: number,
  end: number
) {
  return await program.methods
    .getCertifications(start, end)
    .accounts({
      traceabilityAccount: traceabilityAccount,
    })
    .rpc();
}
