import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { createMintAndVault, getTokenAccount, createTokenAccountInstrs, createMint, sleep, createTokenAccount } from '@project-serum/common';
import { Lockup } from '../target/types/lockup';
import { Registry } from '../target/types/registry';
import { RegistryRealizor } from '../target/types/registry_realizor';
import { PublicKey, SystemProgram, Keypair, Commitment, Connection } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
// import { assert } from 'chai';
import assert from 'assert';
import { createBalanceSandbox } from "./utils";
import { SendTxRequest } from '@project-serum/anchor/dist/cjs/provider';

describe("Lockup and Registry", () => {
  // const commitment: Commitment = 'processed';
  // const connection = new Connection('https://rpc-mainnet-fork.dappio.xyz', { commitment, wsEndpoint: 'wss://rpc-mainnet-fork.dappio.xyz/ws' });
  // const options = anchor.Provider.defaultOptions();
  // const wallet = NodeWallet.local();
  // const provider = new anchor.Provider(connection, wallet, options);

  const provider = anchor.Provider.env();

  anchor.setProvider(provider);

  const lockup = anchor.workspace.Lockup as Program<Lockup>;
  const registry = anchor.workspace.Registry as Program<Registry>;
  const registryRealizor = anchor.workspace.RegistryRealizor as Program<RegistryRealizor>;

  const WHITELIST_SIZE = 10;

  let lockupAddress = null as PublicKey;
  let lockupNonce = null as number;

  let mint = null;
  let god = null;

  it("Sets up initial test state", async () => {
    const [_mint, _god] = await createMintAndVault(
      provider,
      new anchor.BN(1000000)
    );
    mint = _mint;
    god = _god;
  });

  it("Is initialized!", async () => {
    [
      lockupAddress,
      lockupNonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("lockup"))],
      lockup.programId
    );

    await lockup.rpc.whitelistNew(lockupNonce, {
      accounts: {
        authority: provider.wallet.publicKey,
        lockup: lockupAddress,
        systemProgram: SystemProgram.programId
      },
    });

    const lockupAccount = await lockup.account.lockup.fetch(lockupAddress);

    assert.ok(lockupAccount.authority.equals(provider.wallet.publicKey));
    assert.ok(lockupAccount.whitelist.length === WHITELIST_SIZE);
    lockupAccount.whitelist.forEach((e) => {
      assert.ok(e.programId.equals(anchor.web3.PublicKey.default));
    });
  });

  it("Deletes the default whitelisted addresses", async () => {
    const defaultEntry = { programId: anchor.web3.PublicKey.default };
    await lockup.rpc.whitelistDelete(lockupNonce, defaultEntry, {
      accounts: {
        authority: provider.wallet.publicKey,
        lockup: lockupAddress,
      },
    });
  });

  it("Sets a new authority", async () => {
    const newAuthority = Keypair.generate();
    await lockup.rpc.setAuthority(lockupNonce, newAuthority.publicKey, {
      accounts: {
        authority: provider.wallet.publicKey,
        lockup: lockupAddress,
      },
    });

    let lockupAccount = await lockup.account.lockup.fetch(lockupAddress);
    assert.ok(lockupAccount.authority.equals(newAuthority.publicKey));

    await lockup.rpc.setAuthority(lockupNonce, provider.wallet.publicKey, {
      accounts: {
        authority: newAuthority.publicKey,
        lockup: lockupAddress,
      },
      signers: [newAuthority],
    });

    lockupAccount = await lockup.account.lockup.fetch(lockupAddress);
    assert.ok(lockupAccount.authority.equals(provider.wallet.publicKey));
  });

  const entries = [];

  it("Adds to the whitelist", async () => {
    const generateEntry = async () => {
      let programId = Keypair.generate().publicKey;
      return {
        programId,
      };
    };

    for (let k = 0; k < WHITELIST_SIZE; k += 1) {
      entries.push(await generateEntry());
    }

    const accounts = {
      authority: provider.wallet.publicKey,
      lockup: lockupAddress,
    };

    await lockup.rpc.whitelistAdd(lockupNonce, entries[0], { accounts });

    let lockupAccount = await lockup.account.lockup.fetch(lockupAddress);

    assert.ok(lockupAccount.whitelist.length === 1);
    assert.deepEqual(lockupAccount.whitelist, [entries[0]]);

    for (let k = 1; k < WHITELIST_SIZE; k += 1) {
      await lockup.rpc.whitelistAdd(lockupNonce, entries[k], { accounts });
    }

    lockupAccount = await lockup.account.lockup.fetch(lockupAddress);

    assert.deepEqual(lockupAccount.whitelist, entries);

    await assert.rejects(
      async () => {
        const e = await generateEntry();
        await lockup.rpc.whitelistAdd(lockupNonce, e, { accounts });
      },
      (err) => {
        assert.equal(err.code, 6008);
        assert.equal(err.msg, "Whitelist is full");
        return true;
      }
    );
  });

  it("Removes from the whitelist", async () => {
    await lockup.rpc.whitelistDelete(lockupNonce, entries[0], {
      accounts: {
        authority: provider.wallet.publicKey,
        lockup: lockupAddress,
      },
    });
    let lockupAccount = await lockup.account.lockup.fetch(lockupAddress);
    assert.deepEqual(lockupAccount.whitelist, entries.slice(1));
  });

  const vesting = Keypair.generate();
  let vestingAccount = null;
  let vestingSigner = null as PublicKey;

  it("Creates a vesting account", async () => {
    const startTs = new anchor.BN(Date.now() / 1000);
    const endTs = new anchor.BN(startTs.toNumber() + 5);
    const periodCount = new anchor.BN(2);
    const beneficiary = provider.wallet.publicKey;
    const depositAmount = new anchor.BN(100);

    const vault = Keypair.generate();
    let [
      _vestingSigner,
      vestingNonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
      [vesting.publicKey.toBuffer()],
      lockup.programId
    );
    vestingSigner = _vestingSigner;

    const sig1 = await lockup.rpc.createVesting(
      beneficiary,
      depositAmount,
      vestingNonce,
      startTs,
      endTs,
      periodCount,
      null, // Lock realizor is None.
      {
        accounts: {
          vesting: vesting.publicKey,
          vault: vault.publicKey,
          depositor: god,
          depositorAuthority: provider.wallet.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [vesting, vault],
        instructions: [
          await lockup.account.vesting.createInstruction(vesting),
          ...(await createTokenAccountInstrs(
            provider,
            vault.publicKey,
            mint,
            vestingSigner
          )),
        ],
      }
    );

    vestingAccount = await lockup.account.vesting.fetch(vesting.publicKey);

    assert.ok(vestingAccount.beneficiary.equals(provider.wallet.publicKey));
    assert.ok(vestingAccount.mint.equals(mint));
    assert.ok(vestingAccount.grantor.equals(provider.wallet.publicKey));
    assert.ok(vestingAccount.outstanding.eq(depositAmount));
    assert.ok(vestingAccount.startBalance.eq(depositAmount));
    assert.ok(vestingAccount.whitelistOwned.eq(new anchor.BN(0)));
    assert.equal(vestingAccount.nonce, vestingNonce);
    assert.ok(vestingAccount.createdTs.gt(new anchor.BN(0)));
    assert.ok(vestingAccount.startTs.eq(startTs));
    assert.ok(vestingAccount.endTs.eq(endTs));
    assert.ok(vestingAccount.realizor === null);
  });

  it("Fails to withdraw from a vesting account before vesting", async () => {
    await assert.rejects(
      async () => {
        await lockup.rpc.withdraw(new anchor.BN(100), {
          accounts: {
            vesting: vesting.publicKey,
            beneficiary: provider.wallet.publicKey,
            token: god,
            vault: vestingAccount.vault,
            vestingSigner: vestingSigner,
            tokenProgram: TOKEN_PROGRAM_ID,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          },
        });
      },
      (err) => {
        assert.equal(err.code, 6007);
        assert.equal(err.msg, "Insufficient withdrawal balance.");
        return true;
      }
    );
  });

  it("Waits for a vesting period to pass", async () => {
    await sleep(10 * 1000);
  });

  it("Withdraws from the vesting account", async () => {
    const token = await createTokenAccount(
      provider,
      mint,
      provider.wallet.publicKey
    );

    await lockup.rpc.withdraw(new anchor.BN(100), {
      accounts: {
        vesting: vesting.publicKey,
        beneficiary: provider.wallet.publicKey,
        token,
        vault: vestingAccount.vault,
        vestingSigner,
        tokenProgram: TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      },
    });

    vestingAccount = await lockup.account.vesting.fetch(vesting.publicKey);
    assert.ok(vestingAccount.outstanding.eq(new anchor.BN(0)));

    const vaultAccount = await getTokenAccount(
      provider,
      vestingAccount.vault
    );
    assert.ok(vaultAccount.amount.eq(new anchor.BN(0)));

    const tokenAccount = await getTokenAccount(provider, token);
    assert.ok(tokenAccount.amount.eq(new anchor.BN(100)));
  });

  const registrar = Keypair.generate();
  const rewardQ = Keypair.generate();
  const withdrawalTimelock = new anchor.BN(4);
  const stakeRate = new anchor.BN(2);
  const rewardQLen = 170;
  let registrarAccount = null;
  let registrarSigner = null;
  let registrarNonce = null;
  let poolMint = null;
  let registryAddress = null as PublicKey;
  let registryNonce = null as number;

  it("Creates registry genesis", async () => {
    [
      registryAddress,
      registryNonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("registry"))],
      registry.programId
    );

    const [
      _registrarSigner,
      _nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
      [registrar.publicKey.toBuffer()],
      registry.programId
    );
    registrarSigner = _registrarSigner;
    registrarNonce = _nonce;
    poolMint = await createMint(provider, registrarSigner);
  });

  it("Initializes registry's global state", async () => {
    let accounts = {
      authority: provider.wallet.publicKey,
      lockupProgram: lockup.programId,
      realizorProgram: registryRealizor.programId,
      registry: registryAddress,
      systemProgram: SystemProgram.programId,
    };
    await registry.rpc.newRegistry(registryNonce, { accounts });

    const registryAccount = await registry.account.registry.fetch(registryAddress);
    assert.ok(registryAccount.lockupProgram.equals(lockup.programId));

    // Should not allow a second initializatoin.
    await assert.rejects(
      async () => {
        await registry.rpc.newRegistry(registryNonce, { accounts });
      },
      (err) => {
        return true;
      }
    );
  });

  it("Initializes the registrar", async () => {
    await registry.rpc.initialize(
      mint,
      provider.wallet.publicKey,
      registrarNonce,
      withdrawalTimelock,
      stakeRate,
      rewardQLen,
      {
        accounts: {
          registrar: registrar.publicKey,
          poolMint,
          rewardEventQ: rewardQ.publicKey,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [registrar, rewardQ],
        instructions: [
          await registry.account.registrar.createInstruction(registrar),
          await registry.account.rewardQueue.createInstruction(rewardQ, 8250),
        ],
      }
    );

    registrarAccount = await registry.account.registrar.fetch(registrar.publicKey);

    assert.ok(registrarAccount.authority.equals(provider.wallet.publicKey));
    assert.equal(registrarAccount.nonce, registrarNonce);
    assert.ok(registrarAccount.mint.equals(mint));
    assert.ok(registrarAccount.poolMint.equals(poolMint));
    assert.ok(registrarAccount.stakeRate.eq(stakeRate));
    assert.ok(registrarAccount.rewardEventQ.equals(rewardQ.publicKey));
    assert.ok(registrarAccount.withdrawalTimelock.eq(withdrawalTimelock));
  });

  const member = Keypair.generate();
  let memberAccount = null;
  let memberSigner = null;
  let balances = null;
  let balancesLocked = null;

  it("Creates a member", async () => {
    const [
      _memberSigner,
      memberNonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
      [registrar.publicKey.toBuffer(), member.publicKey.toBuffer()],
      registry.programId
    );
    memberSigner = _memberSigner;

    const [mainTx, _balances] = await createBalanceSandbox(
      provider,
      registrarAccount,
      memberSigner
    );
    const [lockedTx, _balancesLocked] = await createBalanceSandbox(
      provider,
      registrarAccount,
      memberSigner
    );

    balances = _balances;
    balancesLocked = _balancesLocked;

    const txCreate = registry.transaction.createMember(memberNonce, {
      accounts: {
        registrar: registrar.publicKey,
        member: member.publicKey,
        beneficiary: provider.wallet.publicKey,
        memberSigner,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      instructions: [await registry.account.member.createInstruction(member)],
    });

    const txUpdateBalances = registry.transaction.updateMemberBalances(memberNonce, {
      accounts: {
        registrar: registrar.publicKey,
        member: member.publicKey,
        memberSigner,
        balances,
      },
    });

    const txUpdateBalancesLock = registry.transaction.updateMemberBalancesLock(memberNonce, {
      accounts: {
        registrar: registrar.publicKey,
        member: member.publicKey,
        memberSigner,
        balancesLocked,
      },
    });

    const wallet = provider.wallet as NodeWallet

    const signers = [member, wallet.payer];

    const allTxs: SendTxRequest[] = [
      mainTx as SendTxRequest,
      lockedTx as SendTxRequest,
      { tx: txCreate, signers },
      { tx: txUpdateBalances, signers: [wallet.payer] },
      { tx: txUpdateBalancesLock, signers: [wallet.payer] },
    ];

    await provider.sendAll(allTxs);

    memberAccount = await registry.account.member.fetch(member.publicKey);

    assert.ok(memberAccount.registrar.equals(registrar.publicKey));
    assert.ok(memberAccount.beneficiary.equals(provider.wallet.publicKey));
    assert.ok(memberAccount.metadata.equals(anchor.web3.PublicKey.default));
    assert.equal(
      JSON.stringify(memberAccount.balances),
      JSON.stringify(balances)
    );
    assert.equal(
      JSON.stringify(memberAccount.balancesLocked),
      JSON.stringify(balancesLocked)
    );
    assert.ok(memberAccount.rewardsCursor === 0);
    assert.ok(memberAccount.lastStakeTs.eq(new anchor.BN(0)));
  });

  it("Deposits (unlocked) to a member", async () => {
    const depositAmount = new anchor.BN(120);
    await registry.rpc.deposit(depositAmount, {
      accounts: {
        depositor: god,
        depositorAuthority: provider.wallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        vault: memberAccount.balances.vault,
        beneficiary: provider.wallet.publicKey,
        member: member.publicKey,
      },
    });

    const memberVault = await getTokenAccount(
      provider,
      memberAccount.balances.vault
    );
    assert.ok(memberVault.amount.eq(depositAmount));
  });

  it("Stakes to a member (unlocked)", async () => {
    const stakeAmount = new anchor.BN(10);
    await registry.rpc.stake(stakeAmount, {
      accounts: {
        // Stake instance.
        registrar: registrar.publicKey,
        rewardEventQ: rewardQ.publicKey,
        poolMint,
        // Member.
        member: member.publicKey,
        beneficiary: provider.wallet.publicKey,
        balances,
        // balancesLocked,
        // Program signers.
        memberSigner,
        registrarSigner,
        // Misc.
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    });

    const vault = await getTokenAccount(
      provider,
      memberAccount.balances.vault
    );
    const vaultStake = await getTokenAccount(
      provider,
      memberAccount.balances.vaultStake
    );
    const spt = await getTokenAccount(
      provider,
      memberAccount.balances.spt
    );

    assert.ok(vault.amount.eq(new anchor.BN(100)));
    assert.ok(vaultStake.amount.eq(new anchor.BN(20)));
    assert.ok(spt.amount.eq(new anchor.BN(10)));
  });

  const unlockedVendor = Keypair.generate();
  const unlockedVendorVault = Keypair.generate();
  let unlockedVendorSigner = null;

  it("Drops an unlocked reward", async () => {
    const rewardKind = {
      unlocked: {},
    };
    const rewardAmount = new anchor.BN(200);
    const expiry = new anchor.BN(Date.now() / 1000 + 5);
    const [
      _vendorSigner,
      nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
      [registrar.publicKey.toBuffer(), unlockedVendor.publicKey.toBuffer()],
      registry.programId
    );
    unlockedVendorSigner = _vendorSigner;

    await registry.rpc.dropReward(
      rewardKind,
      rewardAmount,
      expiry,
      provider.wallet.publicKey,
      nonce,
      {
        accounts: {
          registrar: registrar.publicKey,
          rewardEventQ: rewardQ.publicKey,
          poolMint,

          vendor: unlockedVendor.publicKey,
          vendorVault: unlockedVendorVault.publicKey,

          depositor: god,
          depositorAuthority: provider.wallet.publicKey,

          tokenProgram: TOKEN_PROGRAM_ID,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [unlockedVendorVault, unlockedVendor],
        instructions: [
          ...(await createTokenAccountInstrs(
            provider,
            unlockedVendorVault.publicKey,
            mint,
            unlockedVendorSigner
          )),
          await registry.account.rewardVendor.createInstruction(unlockedVendor),
        ],
      }
    );

    const vendorAccount = await registry.account.rewardVendor.fetch(
      unlockedVendor.publicKey
    );

    assert.ok(vendorAccount.registrar.equals(registrar.publicKey));
    assert.ok(vendorAccount.vault.equals(unlockedVendorVault.publicKey));
    assert.ok(vendorAccount.nonce === nonce);
    assert.ok(vendorAccount.poolTokenSupply.eq(new anchor.BN(10)));
    assert.ok(vendorAccount.expiryTs.eq(expiry));
    assert.ok(vendorAccount.expiryReceiver.equals(provider.wallet.publicKey));
    assert.ok(vendorAccount.total.eq(rewardAmount));
    assert.ok(vendorAccount.expired === false);
    assert.ok(vendorAccount.rewardEventQCursor === 0);
    assert.deepEqual(vendorAccount.kind, rewardKind);

    const rewardQAccount = await registry.account.rewardQueue.fetch(
      rewardQ.publicKey
    );
    assert.ok(rewardQAccount.head === 1);
    assert.ok(rewardQAccount.tail === 0);
    const e = rewardQAccount.events[0];
    assert.ok(e.vendor.equals(unlockedVendor.publicKey));
    assert.equal(e.locked, false);
  });

  it("Collects an unlocked reward", async () => {
    const token = await createTokenAccount(
      provider,
      mint,
      provider.wallet.publicKey
    );
    await registry.rpc.claimReward({
      accounts: {
        to: token,
        cmn: {
          registrar: registrar.publicKey,

          member: member.publicKey,
          beneficiary: provider.wallet.publicKey,
          balancesSpt: balances.spt,
          balancesLockedSpt: balancesLocked.spt,
          vendor: unlockedVendor.publicKey,
          vault: unlockedVendorVault.publicKey,
          vendorSigner: unlockedVendorSigner,

          tokenProgram: TOKEN_PROGRAM_ID,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
      },
    });

    let tokenAccount = await getTokenAccount(provider, token);
    assert.ok(tokenAccount.amount.eq(new anchor.BN(200)));

    const memberAccount = await registry.account.member.fetch(member.publicKey);
    assert.ok(memberAccount.rewardsCursor == 1);
  });

  const lockedVendor = Keypair.generate();
  const lockedVendorVault = Keypair.generate();
  let lockedVendorSigner = null;
  let lockedRewardAmount = null;
  let lockedRewardKind = null;

  it("Drops a locked reward", async () => {
    lockedRewardKind = {
      locked: {
        startTs: new anchor.BN(Date.now() / 1000),
        endTs: new anchor.BN(Date.now() / 1000 + 6),
        periodCount: new anchor.BN(2),
      },
    };
    lockedRewardAmount = new anchor.BN(200);
    const expiry = new anchor.BN(Date.now() / 1000 + 5);
    const [
      _vendorSigner,
      nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
      [registrar.publicKey.toBuffer(), lockedVendor.publicKey.toBuffer()],
      registry.programId
    );
    lockedVendorSigner = _vendorSigner;

    await registry.rpc.dropReward(
      lockedRewardKind,
      lockedRewardAmount,
      expiry,
      provider.wallet.publicKey,
      nonce,
      {
        accounts: {
          registrar: registrar.publicKey,
          rewardEventQ: rewardQ.publicKey,
          poolMint,

          vendor: lockedVendor.publicKey,
          vendorVault: lockedVendorVault.publicKey,

          depositor: god,
          depositorAuthority: provider.wallet.publicKey,

          tokenProgram: TOKEN_PROGRAM_ID,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [lockedVendorVault, lockedVendor],
        instructions: [
          ...(await createTokenAccountInstrs(
            provider,
            lockedVendorVault.publicKey,
            mint,
            lockedVendorSigner
          )),
          await registry.account.rewardVendor.createInstruction(lockedVendor),
        ],
      }
    );

    const vendorAccount = await registry.account.rewardVendor.fetch(
      lockedVendor.publicKey
    );

    assert.ok(vendorAccount.registrar.equals(registrar.publicKey));
    assert.ok(vendorAccount.vault.equals(lockedVendorVault.publicKey));
    assert.ok(vendorAccount.nonce === nonce);
    assert.ok(vendorAccount.poolTokenSupply.eq(new anchor.BN(10)));
    assert.ok(vendorAccount.expiryTs.eq(expiry));
    assert.ok(vendorAccount.expiryReceiver.equals(provider.wallet.publicKey));
    assert.ok(vendorAccount.total.eq(lockedRewardAmount));
    assert.ok(vendorAccount.expired === false);
    assert.ok(vendorAccount.rewardEventQCursor === 1);
    assert.equal(
      JSON.stringify(vendorAccount.kind),
      JSON.stringify(lockedRewardKind)
    );

    const rewardQAccount = await registry.account.rewardQueue.fetch(
      rewardQ.publicKey
    );
    assert.ok(rewardQAccount.head === 2);
    assert.ok(rewardQAccount.tail === 0);
    const e = rewardQAccount.events[1];
    assert.ok(e.vendor.equals(lockedVendor.publicKey));
    assert.ok(e.locked === true);
  });

  let vendoredVesting = null;
  let vendoredVestingVault = null;
  let vendoredVestingSigner = null;

  it("Claims a locked reward", async () => {
    vendoredVesting = Keypair.generate();
    vendoredVestingVault = Keypair.generate();
    let [
      _vendoredVestingSigner,
      nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
      [vendoredVesting.publicKey.toBuffer()],
      lockup.programId
    );
    vendoredVestingSigner = _vendoredVestingSigner;
    const remainingAccounts = lockup.instruction.createVesting
      .accounts({
        vesting: vendoredVesting.publicKey,
        vault: vendoredVestingVault.publicKey,
        depositor: lockedVendorVault.publicKey,
        depositorAuthority: lockedVendorSigner,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      // Change the signer status on the vendor signer since it's signed by the program, not the
      // client.
      .map((meta) =>
        meta.pubkey === lockedVendorSigner ? { ...meta, isSigner: false } : meta
      );

    const sig = await registry.rpc.claimRewardLocked(registryNonce, nonce, {
      accounts: {
        registry: registryAddress,
        lockupProgram: lockup.programId,
        realizorProgram: registryRealizor.programId,
        cmn: {
          registrar: registrar.publicKey,

          member: member.publicKey,
          beneficiary: provider.wallet.publicKey,
          balancesSpt: balances.spt,
          balancesLockedSpt: balancesLocked.spt,

          vendor: lockedVendor.publicKey,
          vault: lockedVendorVault.publicKey,
          vendorSigner: lockedVendorSigner,

          tokenProgram: TOKEN_PROGRAM_ID,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
      },
      remainingAccounts,
      signers: [vendoredVesting, vendoredVestingVault],
      instructions: [
        await lockup.account.vesting.createInstruction(vendoredVesting),
        ...(await createTokenAccountInstrs(
          provider,
          vendoredVestingVault.publicKey,
          mint,
          vendoredVestingSigner
        )),
      ],
    });

    const lockupAccount = await lockup.account.vesting.fetch(
      vendoredVesting.publicKey
    );

    assert.ok(lockupAccount.beneficiary.equals(provider.wallet.publicKey));
    assert.ok(lockupAccount.mint.equals(mint));
    assert.ok(lockupAccount.vault.equals(vendoredVestingVault.publicKey));
    assert.ok(lockupAccount.outstanding.eq(lockedRewardAmount));
    assert.ok(lockupAccount.startBalance.eq(lockedRewardAmount));
    assert.ok(lockupAccount.endTs.eq(lockedRewardKind.locked.endTs));
    assert.ok(
      lockupAccount.periodCount.eq(lockedRewardKind.locked.periodCount)
    );
    assert.ok(lockupAccount.whitelistOwned.eq(new anchor.BN(0)));
    assert.ok(lockupAccount.realizor.program.equals(registryRealizor.programId));
    assert.ok(lockupAccount.realizor.metadata.equals(member.publicKey));
  });

  it("Waits for the lockup period to pass", async () => {
    await sleep(10 * 1000);
  });

  it("Should fail to unlock an unrealized lockup reward", async () => {
    // Get Member account
    const memberAccount = await registry.account.member.fetch(
      member.publicKey
    );

    const token = await createTokenAccount(
      provider,
      mint,
      provider.wallet.publicKey
    );
    await assert.rejects(
      async () => {
        const withdrawAmount = new anchor.BN(10);
        await lockup.rpc.withdraw(withdrawAmount, {
          accounts: {
            vesting: vendoredVesting.publicKey,
            beneficiary: provider.wallet.publicKey,
            token,
            vault: vendoredVestingVault.publicKey,
            vestingSigner: vendoredVestingSigner,
            tokenProgram: TOKEN_PROGRAM_ID,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          },
          // TODO: trait methods generated on the client. Until then, we need to manually
          //       specify the account metas here.
          remainingAccounts: [
            { pubkey: registryRealizor.programId, isWritable: false, isSigner: false },
            { pubkey: registry.programId, isWritable: false, isSigner: false },
            // is_realized
            { pubkey: member.publicKey, isWritable: false, isSigner: false },
            { pubkey: balances.spt, isWritable: false, isSigner: false },
            { pubkey: balancesLocked.spt, isWritable: false, isSigner: false },
            // MemberData
            { pubkey: memberAccount.registrar, isWritable: false, isSigner: false },
            { pubkey: memberAccount.beneficiary, isWritable: false, isSigner: false },
            { pubkey: memberAccount.metadata, isWritable: false, isSigner: false },
            { pubkey: memberAccount.balances.spt, isWritable: false, isSigner: false },
            { pubkey: memberAccount.balancesLocked.spt, isWritable: false, isSigner: false },
          ],
        });
      },
      (err) => {
        // Solana doesn't propagate errors across CPI. So we receive the registry's error code,
        // not the lockup's.
        // const errorCode = "custom program error: 0x65";
        // assert.ok(err.toString().split(errorCode).length === 2);
        assert.equal(err.code, 6000);
        return true;
      }
    );
  });

  const pendingWithdrawal = Keypair.generate();

  it("Unstakes (unlocked)", async () => {
    const unstakeAmount = new anchor.BN(10);

    await registry.rpc.startUnstake(unstakeAmount, false, {
      accounts: {
        registrar: registrar.publicKey,
        rewardEventQ: rewardQ.publicKey,
        poolMint,

        pendingWithdrawal: pendingWithdrawal.publicKey,
        member: member.publicKey,
        beneficiary: provider.wallet.publicKey,
        balances,

        memberSigner,

        tokenProgram: TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [pendingWithdrawal],
      instructions: [
        await registry.account.pendingWithdrawal.createInstruction(
          pendingWithdrawal
        ),
      ],
    });

    const vaultPw = await getTokenAccount(
      provider,
      memberAccount.balances.vaultPw
    );
    const vaultStake = await getTokenAccount(
      provider,
      memberAccount.balances.vaultStake
    );
    const spt = await getTokenAccount(
      provider,
      memberAccount.balances.spt
    );

    assert.ok(vaultPw.amount.eq(new anchor.BN(20)));
    assert.ok(vaultStake.amount.eq(new anchor.BN(0)));
    assert.ok(spt.amount.eq(new anchor.BN(0)));
  });

  const tryEndUnstake = async () => {
    await registry.rpc.endUnstake({
      accounts: {
        registrar: registrar.publicKey,

        member: member.publicKey,
        beneficiary: provider.wallet.publicKey,
        pendingWithdrawal: pendingWithdrawal.publicKey,

        vault: balances.vault,
        vaultPw: balances.vaultPw,

        memberSigner,

        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    });
  };

  it("Fails to end unstaking before timelock", async () => {
    await assert.rejects(
      async () => {
        await tryEndUnstake();
      },
      (err) => {
        assert.equal(err.code, 6009);
        assert.equal(err.msg, "The unstake timelock has not yet expired.");
        return true;
      }
    );
  });

  it("Waits for the unstake period to end", async () => {
    await sleep(5000);
  });

  it("Unstake finalizes (unlocked)", async () => {
    await tryEndUnstake();

    const vault = await getTokenAccount(
      provider,
      memberAccount.balances.vault
    );
    const vaultPw = await getTokenAccount(
      provider,
      memberAccount.balances.vaultPw
    );

    assert.ok(vault.amount.eq(new anchor.BN(120)));
    assert.ok(vaultPw.amount.eq(new anchor.BN(0)));
  });

  it("Withdraws deposits (unlocked)", async () => {
    const token = await createTokenAccount(
      provider,
      mint,
      provider.wallet.publicKey
    );
    const withdrawAmount = new anchor.BN(100);
    await registry.rpc.withdraw(withdrawAmount, {
      accounts: {
        registrar: registrar.publicKey,
        member: member.publicKey,
        beneficiary: provider.wallet.publicKey,
        vault: memberAccount.balances.vault,
        memberSigner,
        depositor: token,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    });

    const tokenAccount = await getTokenAccount(provider, token);
    assert.ok(tokenAccount.amount.eq(withdrawAmount));
  });

  it("Should succesfully unlock a locked reward after unstaking", async () => {
    // Get Member account
    const memberAccount = await registry.account.member.fetch(
      member.publicKey
    );

    const token = await createTokenAccount(
      provider,
      mint,
      provider.wallet.publicKey
    );

    const withdrawAmount = new anchor.BN(7);
    await lockup.rpc.withdraw(withdrawAmount, {
      accounts: {
        vesting: vendoredVesting.publicKey,
        beneficiary: provider.wallet.publicKey,
        token,
        vault: vendoredVestingVault.publicKey,
        vestingSigner: vendoredVestingSigner,
        tokenProgram: TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      },
      // TODO: trait methods generated on the client. Until then, we need to manually
      //       specify the account metas here.
      remainingAccounts: [
        { pubkey: registryRealizor.programId, isWritable: false, isSigner: false },
        { pubkey: registry.programId, isWritable: false, isSigner: false },
        // is_realized
        { pubkey: member.publicKey, isWritable: false, isSigner: false },
        { pubkey: balances.spt, isWritable: false, isSigner: false },
        { pubkey: balancesLocked.spt, isWritable: false, isSigner: false },
        // MemberData
        { pubkey: memberAccount.registrar, isWritable: false, isSigner: false },
        { pubkey: memberAccount.beneficiary, isWritable: false, isSigner: false },
        { pubkey: memberAccount.metadata, isWritable: false, isSigner: false },
        { pubkey: memberAccount.balances.spt, isWritable: false, isSigner: false },
        { pubkey: memberAccount.balancesLocked.spt, isWritable: false, isSigner: false },
      ],
    });
    const tokenAccount = await getTokenAccount(provider, token);
    assert.ok(tokenAccount.amount.eq(withdrawAmount));
  });
});
