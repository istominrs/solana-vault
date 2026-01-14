import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';

import { TOKEN_PROGRAM_ID } from '@solana/spl-token';

import { BanksClient, startAnchor } from 'solana-bankrun';
import { createMint } from 'spl-token-bankrun';
import IDL from '../target/idl/vault.json';
import { Vault } from '../target/types/vault';
import { SYSTEM_PROGRAM_ID } from '@coral-xyz/anchor/dist/cjs/native/system';

describe('vault tests', () => {
  const vaultName = 'vault';

  let context;
  let provider: BankrunProvider;
  let vaultProgram: anchor.Program<Vault>;
  let mint: PublicKey;
  let user: Keypair;
  let authority: Keypair;
  let vaultAccountPda: PublicKey;
  let vaultTokenAccountPda: PublicKey;
  let banksClient: BanksClient;

  beforeAll(async () => {
    mint = Keypair.generate().publicKey;
    user = Keypair.generate();

    context = await startAnchor(
      '',
      [{ name: 'vault', programId: new PublicKey(IDL.address) }],
      [
        {
          address: user.publicKey,
          info: {
            lamports: 10 * LAMPORTS_PER_SOL,
            data: Buffer.alloc(0),
            owner: SYSTEM_PROGRAM_ID,
            executable: false,
          },
        },
      ],
    );
    provider = new BankrunProvider(context);
    anchor.setProvider(provider);

    vaultProgram = new Program<Vault>(IDL, provider);

    banksClient = context.banksClient;

    authority = provider.wallet.payer;

    // Create mint
    // @ts-ignore
    mint = await createMint(banksClient, authority, authority.publicKey, null, 2);

    // Derive PDAs
    [vaultAccountPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(vaultName)],
      vaultProgram.programId,
    );

    [vaultTokenAccountPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('treasury'), Buffer.from(vaultName)],
      vaultProgram.programId,
    );
  });

  it('create vault', async () => {
    await vaultProgram.methods
      .createVault(vaultName)
      .accounts({
        authority: authority.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        mint: mint,
      })
      .rpc({ commitment: 'confirmed' });

    const vault = await vaultProgram.account.vault.fetch(vaultAccountPda);

    expect(vault.vaultName).toEqual(vaultName);
    expect(vault.isLocked).toBeFalsy();
    expect(vault.unlockTimestamp.toNumber()).toEqual(0);
  });
});
