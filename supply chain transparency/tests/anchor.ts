import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { SupplyChainTransparency } from '../target/types/supply_chain_transparency';
import { expect } from 'chai';
import type { SupplyChainTransparency } from "../target/types/supply_chain_transparency";

describe('supply_chain_transparency', () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SupplyChainTransparency as anchor.Program<SupplyChainTransparency>;
  
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SupplyChainTransparency as Program<SupplyChainTransparency>;

  let traceabilityAccount = anchor.web3.Keypair.generate();

  it('Initializes a new traceability account', async () => {
    await program.methods
      .initializeTraceability('Apples')
      .accounts({
        traceabilityAccount: traceabilityAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([traceabilityAccount])
      .rpc();

    const account = await program.account.traceability.fetch(traceabilityAccount.publicKey);
    expect(account.productName).to.equal('Apples');
    expect(account.origin).to.equal('Farm');
    expect(account.stages).to.be.an('array').that.is.empty;
    expect(account.certifications).to.be.an('array').that.is.empty;
  });

  it('Updates the traceability stages', async () => {
    await program.methods
      .updateTraceability('Packaging')
      .accounts({
        traceabilityAccount: traceabilityAccount.publicKey,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.traceability.fetch(traceabilityAccount.publicKey);
    expect(account.stages).to.include('Packaging');
  });

  it('Adds certifications', async () => {
    await program.methods
      .addCertification('Organic')
      .accounts({
        traceabilityAccount: traceabilityAccount.publicKey,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.traceability.fetch(traceabilityAccount.publicKey);
    expect(account.certifications).to.include('Organic');
  });

  it('Fetches stages with pagination', async () => {
    const stages = await program.methods
      .getStages(0, 1)
      .accounts({
        traceabilityAccount: traceabilityAccount.publicKey,
      })
      .rpc();

    expect(stages).to.be.an('array').that.includes('Packaging');
  });

  it('Fetches certifications with pagination', async () => {
    const certifications = await program.methods
      .getCertifications(0, 1)
      .accounts({
        traceabilityAccount: traceabilityAccount.publicKey,
      })
      .rpc();

    expect(certifications).to.be.an('array').that.includes('Organic');
  });

  it('Closes the traceability account', async () => {
    const recipient = provider.wallet.publicKey;
    await program.methods
      .closeTraceability()
      .accounts({
        traceabilityAccount: traceabilityAccount.publicKey,
        recipient: recipient,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    try {
      await program.account.traceability.fetch(traceabilityAccount.publicKey);
      throw new Error('Account should be closed');
    } catch (error) {
      expect(error.message).to.include('Account does not exist');
    }
  });
});
