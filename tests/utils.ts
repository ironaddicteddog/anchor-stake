import * as anchor from '@project-serum/anchor';
import { createTokenAccountInstrs } from '@project-serum/common';
import { Keypair } from '@solana/web3.js';

export async function createBalanceSandbox(provider, r, registrySigner) {
  const spt = Keypair.generate();
  const vault = Keypair.generate();
  const vaultStake = Keypair.generate();
  const vaultPw = Keypair.generate();

  const lamports = await provider.connection.getMinimumBalanceForRentExemption(
    165
  );

  const createSptIx = await createTokenAccountInstrs(
    provider,
    spt.publicKey,
    r.poolMint,
    registrySigner,
    lamports
  );
  const createVaultIx = await createTokenAccountInstrs(
    provider,
    vault.publicKey,
    r.mint,
    registrySigner,
    lamports
  );
  const createVaultStakeIx = await createTokenAccountInstrs(
    provider,
    vaultStake.publicKey,
    r.mint,
    registrySigner,
    lamports
  );
  const createVaultPwIx = await createTokenAccountInstrs(
    provider,
    vaultPw.publicKey,
    r.mint,
    registrySigner,
    lamports
  );
  let tx0 = new anchor.web3.Transaction();
  tx0.add(
    ...createSptIx,
    ...createVaultIx,
    ...createVaultStakeIx,
    ...createVaultPwIx
  );
  let signers0 = [spt, vault, vaultStake, vaultPw];

  const tx = { tx: tx0, signers: signers0 };

  return [
    tx,
    {
      spt: spt.publicKey,
      vault: vault.publicKey,
      vaultStake: vaultStake.publicKey,
      vaultPw: vaultPw.publicKey,
    },
  ];
}
