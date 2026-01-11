import * as anchor from '@coral-xyz/anchor';
import { Program, BN } from '@coral-xyz/anchor';
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';

import { BanksClient, ProgramTestContext, startAnchor } from 'solana-bankrun';

import IDL from '../target/idl/time_lock_vault.json';
import { TimeLockVault } from '../target/types/time_lock_vault';
import { SYSTEM_PROGRAM_ID } from '@coral-xyz/anchor/dist/cjs/native/system';

import { createMint, mintTo } from 'spl-token-bankrun';
import { TOKEN_PROGRAM_ID } from '@coral-xyz/anchor/dist/cjs/utils/token';
import NodeWallet from '@coral-xyz/anchor/dist/cjs/nodewallet';

describe('time lock vault tests', () => {
  const vaultName: string = 'vault';

  let recipient: Keypair;
  let authority: Keypair;
  let mint: PublicKey;
  let vaultAccountKey: PublicKey;
  let vaultAccountBump: number;

  let context: ProgramTestContext;
  let provider: BankrunProvider;
  let recipientProvider: BankrunProvider;
  let program: Program<TimeLockVault>;
  let recipientProgram: Program<TimeLockVault>;
  let banksClient: BanksClient;

  beforeAll(async () => {
    recipient = new anchor.web3.Keypair();

    context = await startAnchor(
      '',
      [{ name: 'time_lock_vault', programId: new PublicKey(IDL.address) }],
      [
        {
          address: recipient.publicKey,
          info: {
            lamports: LAMPORTS_PER_SOL,
            data: Buffer.alloc(0),
            owner: SYSTEM_PROGRAM_ID,
            executable: false,
          },
        },
      ],
    );

    provider = new BankrunProvider(context);
    anchor.setProvider(provider);

    program = new Program<TimeLockVault>(IDL, provider);

    banksClient = context.banksClient;

    authority = provider.wallet.payer;

    // Create mint
    // @ts-ignore
    mint = await createMint(banksClient, authority, authority.publicKey, null, 2);

    recipientProvider = new BankrunProvider(context);
    recipientProvider.wallet = new NodeWallet(recipient);

    recipientProgram = new Program<TimeLockVault>(IDL as TimeLockVault, recipientProvider);

    // Derive PDAs
    [vaultAccountKey] = PublicKey.findProgramAddressSync(
      [Buffer.from('treasury'), Buffer.from(vaultName)],
      program.programId,
    );
  });

  it('initialize vault', async () => {
    await program.methods
      .initializeVault(vaultName, new BN(1000), new BN(100_000_000))
      .accounts({
        authority: authority.publicKey,
        mint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc({ commitment: 'confirmed' });

    const vaultAccountData = await program.account.vault.fetch(vaultAccountKey, 'confirmed');
    console.log('Vault Account Data:', JSON.stringify(vaultAccountData, null, 2));
  });
});
