import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';

import { startAnchor } from 'solana-bankrun';

import IDL from '../target/idl/vault.json';
import { Vault } from '../target/types/vault';

describe('vault tests', () => {
  const vaultName = 'vault';

  let context;
  let provider;
  let vaultProgram: anchor.Program<Vault>;

  let vaultAccountPda: PublicKey;

  beforeAll(async () => {
    context = await startAnchor('', [{ name: 'vault', programId: new PublicKey(IDL.address) }], []);
    provider = new BankrunProvider(context);

    vaultProgram = new Program<Vault>(IDL, provider);

    [vaultAccountPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(vaultName)],
      vaultProgram.programId,
    );
  });

  it('create vault', async () => {
    await vaultProgram.methods.createVault(vaultName).rpc({ commitment: 'confirmed' });

    const vault = await vaultProgram.account.vault.fetch(vaultAccountPda);

    expect(vault.vaultName).toEqual(vaultName);
    expect(vault.isLocked).toBeFalsy();
    expect(vault.unlockTimestamp.toNumber()).toEqual(0);
  });
});
