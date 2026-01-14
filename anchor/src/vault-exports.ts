// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import TimeLockVaultIDL from '../target/idl/time_lock_vault.json'
import type { TimeLockVault } from '../target/types/time_lock_vault'

// Re-export the generated IDL and type
export { TimeLockVault, TimeLockVaultIDL }

// The programId is imported from the program IDL.
export const TIME_LOCK_VAULT_PROGRAM_ID = new PublicKey(TimeLockVaultIDL.address)

// This is a helper function to get the TimeLockVault Anchor program.
export function getTimeLockVaultProgram(provider: AnchorProvider, address?: PublicKey): Program<TimeLockVault> {
  return new Program(
    { ...TimeLockVaultIDL, address: address ? address.toBase58() : TimeLockVaultIDL.address } as TimeLockVault,
    provider,
  )
}

// This is a helper function to get the program ID for the TimeLockVault program depending on the cluster.
export function getTimeLockVaultProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the TimeLockVault program on devnet and testnet.
      return new PublicKey('Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe')
    case 'mainnet-beta':
    default:
      return TIME_LOCK_VAULT_PROGRAM_ID
  }
}
