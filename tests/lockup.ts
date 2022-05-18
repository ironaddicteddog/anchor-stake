import * as anchor from "@project-serum/anchor";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { IDL as lockupIDL } from "../target/types/lockup";
import { IDL as registryIDL } from "../target/types/registry";
import { IDL as realizorIDL } from "../target/types/registry_realizor";
import {
  PublicKey,
  SystemProgram,
  Keypair,
  Commitment,
  Connection,
} from "@solana/web3.js";
import * as splToken from "@solana/spl-token";
// import { assert } from 'chai';
import assert from "assert";
import {
  createBalanceSandbox,
  createMint,
  createMintAndVault,
  createTokenAccount,
  createTokenAccountInstrs,
  sleep,
} from "./utils";
import { SendTxRequest } from "@project-serum/anchor/dist/cjs/provider";
import { TypeDef } from "@project-serum/anchor/dist/cjs/program/namespace/types";

const LOCKUP_PROGRAM_ID = new anchor.web3.PublicKey(
  "DCrDvbtuqxFiAgTz7JrDy5c1PiuZkgfphyGUTpq2L2eW"
);
const REGISTRY_PROGRAM_ID = new anchor.web3.PublicKey(
  "E9a8yDKJMRDGS7SNJpJGF9mJAK1K2knvqHcWriBK6JRZ"
);
const REALIZOR_PROGRAM_ID = new anchor.web3.PublicKey(
  "6YHjQ62tRTRho5tkFdtMX9rQod2RkwZMz2VfSiGwK9v7"
);

describe("Lockup and Registry", () => {
  const commitment: Commitment = "confirmed";
  const connection = new Connection("https://rpc-mainnet-fork.epochs.studio", {
    commitment,
    wsEndpoint: "wss://rpc-mainnet-fork.epochs.studio/ws",
  });
  const options = anchor.AnchorProvider.defaultOptions();
  const wallet = NodeWallet.local();
  const provider = new anchor.AnchorProvider(connection, wallet, options);

  // const provider = anchor.Provider.env();

  anchor.setProvider(provider);

  const lockup = new anchor.Program(lockupIDL, LOCKUP_PROGRAM_ID, provider);
  const registry = new anchor.Program(
    registryIDL,
    REGISTRY_PROGRAM_ID,
    provider
  );
  const realizor = new anchor.Program(
    realizorIDL,
    REALIZOR_PROGRAM_ID,
    provider
  );

  const WHITELIST_SIZE = 10;

  let lockupAddress = null as PublicKey;
  let _lockupBump = null as number;
  const lockupNonce = new anchor.BN(Math.floor(Math.random() * 100000000));

  let mint = null;
  let god = null;

  it("Sets up initial test state", async () => {
    const [_mint, _god] = await createMintAndVault(
      provider as anchor.AnchorProvider,
      new anchor.BN(1000000)
    );
    mint = _mint;
    god = _god;
  });

  it("Is initialized!", async () => {
    [lockupAddress, _lockupBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from(anchor.utils.bytes.utf8.encode("lockup")),
          lockupNonce.toBuffer("le", 8),
        ],
        lockup.programId
      );

    await lockup.methods
      .whitelistNew(lockupNonce)
      .accounts({
        authority: provider.wallet.publicKey,
        lockup: lockupAddress,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const lockupAccount = await lockup.account.lockup.fetch(lockupAddress);
    const whitelist = lockupAccount.whitelist as Array<
      TypeDef<
        {
          name: "WhitelistEntry";
          type: {
            kind: "struct";
            fields: [
              {
                name: "programId";
                type: "publicKey";
              }
            ];
          };
        },
        Record<string, anchor.web3.PublicKey>
      >
    >;

    assert.ok(lockupAccount.authority.equals(provider.wallet.publicKey));
    assert.ok(whitelist.length === WHITELIST_SIZE);
    whitelist.forEach((e) => {
      assert.ok(e.programId.equals(anchor.web3.PublicKey.default));
    });
  });

  it("Deletes the default whitelisted addresses", async () => {
    const defaultEntry = { programId: anchor.web3.PublicKey.default };
    await lockup.methods
      .whitelistDelete(lockupNonce, defaultEntry)
      .accounts({
        authority: provider.wallet.publicKey,
        lockup: lockupAddress,
      })
      .rpc();
  });

  it("Sets a new authority", async () => {
    const newAuthority = Keypair.generate();
    await lockup.methods
      .setAuthority(lockupNonce, newAuthority.publicKey)
      .accounts({
        authority: provider.wallet.publicKey,
        lockup: lockupAddress,
      })
      .rpc();

    let lockupAccount = await lockup.account.lockup.fetch(lockupAddress);
    assert.ok(lockupAccount.authority.equals(newAuthority.publicKey));

    await lockup.methods
      .setAuthority(lockupNonce, provider.wallet.publicKey)
      .accounts({
        authority: newAuthority.publicKey,
        lockup: lockupAddress,
      })
      .signers([newAuthority])
      .rpc();

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

    await lockup.methods
      .whitelistAdd(lockupNonce, entries[0])
      .accounts(accounts)
      .rpc();

    let lockupAccount = await lockup.account.lockup.fetch(lockupAddress);

    const whitelist = lockupAccount.whitelist as Array<
      TypeDef<
        {
          name: "WhitelistEntry";
          type: {
            kind: "struct";
            fields: [
              {
                name: "programId";
                type: "publicKey";
              }
            ];
          };
        },
        Record<string, anchor.web3.PublicKey>
      >
    >;

    assert.ok(whitelist.length === 1);
    assert.deepEqual(whitelist, [entries[0]]);

    for (let k = 1; k < WHITELIST_SIZE; k += 1) {
      await lockup.methods
        .whitelistAdd(lockupNonce, entries[k])
        .accounts(accounts)
        .rpc();
    }

    lockupAccount = await lockup.account.lockup.fetch(lockupAddress);

    const whitelist2 = lockupAccount.whitelist as Array<
      TypeDef<
        {
          name: "WhitelistEntry";
          type: {
            kind: "struct";
            fields: [
              {
                name: "programId";
                type: "publicKey";
              }
            ];
          };
        },
        Record<string, anchor.web3.PublicKey>
      >
    >;

    assert.deepEqual(whitelist2, entries);

    await assert.rejects(
      async () => {
        const e = await generateEntry();
        await lockup.methods
          .whitelistAdd(lockupNonce, e)
          .accounts(accounts)
          .rpc();
      },
      (err: anchor.AnchorError) => {
        assert.equal(err.error.errorCode.code, "WhitelistFull");
        assert.equal(err.error.errorCode.number, 6008);
        assert.equal(err.error.errorMessage, "Whitelist is full");
        return true;
      }
    );
  });

  it("Removes from the whitelist", async () => {
    await lockup.methods
      .whitelistDelete(lockupNonce, entries[0])
      .accounts({
        authority: provider.wallet.publicKey,
        lockup: lockupAddress,
      })
      .rpc();
    let lockupAccount = await lockup.account.lockup.fetch(lockupAddress);
    assert.deepEqual(lockupAccount.whitelist, entries.slice(1));
  });

  const vesting = Keypair.generate();
  let vestingAccount = null;
  let vestingSigner = null as PublicKey;

  it("Creates a vesting account", async () => {
    const slot = await connection.getSlot();
    const blocktime = await connection.getBlockTime(slot);
    const startTs = new anchor.BN(blocktime);
    // const startTs = new anchor.BN(Date.now() / 1000);

    const endTs = new anchor.BN(startTs.toNumber() + 5);
    const periodCount = new anchor.BN(2);
    const beneficiary = provider.wallet.publicKey;
    const depositAmount = new anchor.BN(100);

    const vault = Keypair.generate();
    let [_vestingSigner, vestingNonce] =
      await anchor.web3.PublicKey.findProgramAddress(
        [vesting.publicKey.toBuffer()],
        lockup.programId
      );
    vestingSigner = _vestingSigner;

    const sig = await lockup.methods
      .createVesting(
        beneficiary,
        depositAmount,
        vestingNonce,
        startTs,
        endTs,
        periodCount,
        null // Lock realizor is None.
      )
      .accounts({
        vesting: vesting.publicKey,
        vault: vault.publicKey,
        depositor: god,
        depositorAuthority: provider.wallet.publicKey,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .signers([vesting, vault])
      .preInstructions([
        await lockup.account.vesting.createInstruction(vesting),
        ...(await createTokenAccountInstrs(
          provider,
          vault.publicKey,
          mint,
          vestingSigner
        )),
      ])
      .rpc();

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

    const vaultAccount = await splToken.getAccount(
      provider.connection,
      vestingAccount.vault
    );
    console.log(`vaultAccount amount: ${vaultAccount.amount}`);
  });

  it("Fails to withdraw from a vesting account before vesting", async () => {
    await assert.rejects(
      async () => {
        await lockup.methods
          .withdraw(new anchor.BN(100))
          .accounts({
            vesting: vesting.publicKey,
            beneficiary: provider.wallet.publicKey,
            token: god,
            vault: vestingAccount.vault,
            vestingSigner: vestingSigner,
            tokenProgram: splToken.TOKEN_PROGRAM_ID,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          })
          .rpc();
      },
      (err: anchor.AnchorError) => {
        assert.equal(err.error.errorCode.code, "InsufficientWithdrawalBalance");
        assert.equal(err.error.errorCode.number, 6007);
        assert.equal(
          err.error.errorMessage,
          "Insufficient withdrawal balance."
        );
        return true;
      }
    );
  });

  it("Waits for a vesting period to pass", async () => {
    await sleep(10 * 1000);

    const vaultAccount = await splToken.getAccount(
      provider.connection,
      vestingAccount.vault
    );
    console.log(`vaultAccount amount: ${vaultAccount.amount}`);
  });

  it("Withdraws from the vesting account", async () => {
    const token = await createTokenAccount(
      provider,
      mint,
      provider.wallet.publicKey
    );

    await lockup.methods
      .withdraw(new anchor.BN(100))
      .accounts({
        vesting: vesting.publicKey,
        beneficiary: provider.wallet.publicKey,
        token,
        vault: vestingAccount.vault,
        vestingSigner,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .rpc();

    vestingAccount = await lockup.account.vesting.fetch(vesting.publicKey);
    assert.ok(vestingAccount.outstanding.eq(new anchor.BN(0)));

    const vaultAccount = await splToken.getAccount(
      provider.connection,
      vestingAccount.vault
    );
    assert.ok(
      new anchor.BN(vaultAccount.amount.toString()).eq(new anchor.BN(0))
    );

    const tokenAccount = await splToken.getAccount(provider.connection, token);
    assert.ok(
      new anchor.BN(tokenAccount.amount.toString()).eq(new anchor.BN(100))
    );
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
  let _registryBump = null as number;

  const registryNonce = new anchor.BN(Math.floor(Math.random() * 100000000));

  it("Creates registry genesis", async () => {
    [registryAddress, _registryBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from(anchor.utils.bytes.utf8.encode("registry")),
          registryNonce.toBuffer("le", 8),
        ],
        registry.programId
      );

    const [_registrarSigner, _nonce] =
      await anchor.web3.PublicKey.findProgramAddress(
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
      realizorProgram: realizor.programId,
      registry: registryAddress,
      systemProgram: SystemProgram.programId,
    };
    await registry.methods.newRegistry(registryNonce).accounts(accounts).rpc();

    const registryAccount = await registry.account.registry.fetch(
      registryAddress
    );
    assert.ok(registryAccount.lockupProgram.equals(lockup.programId));

    // Should not allow a second initializatoin.
    await assert.rejects(
      async () => {
        await registry.methods
          .newRegistry(registryNonce)
          .accounts(accounts)
          .rpc();
      },
      (err) => {
        return true;
      }
    );
  });

  it("Initializes the registrar", async () => {
    await registry.methods
      .initialize(
        mint,
        provider.wallet.publicKey,
        registrarNonce,
        withdrawalTimelock,
        stakeRate,
        rewardQLen
      )
      .accounts({
        registrar: registrar.publicKey,
        poolMint,
        rewardEventQ: rewardQ.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([registrar, rewardQ])
      .preInstructions([
        await registry.account.registrar.createInstruction(registrar),
        await registry.account.rewardQueue.createInstruction(rewardQ, 8250),
      ])
      .rpc();

    registrarAccount = await registry.account.registrar.fetch(
      registrar.publicKey
    );

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
    const [_memberSigner, memberNonce] =
      await anchor.web3.PublicKey.findProgramAddress(
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

    const txCreate = await registry.methods
      .createMember(memberNonce)
      .accounts({
        registrar: registrar.publicKey,
        member: member.publicKey,
        beneficiary: provider.wallet.publicKey,
        memberSigner,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .preInstructions([
        await registry.account.member.createInstruction(member),
      ])
      .transaction();

    const txUpdateBalances = await registry.methods
      .updateMemberBalances(memberNonce)
      .accounts({
        registrar: registrar.publicKey,
        member: member.publicKey,
        memberSigner,
        balances,
      })
      .transaction();

    const txUpdateBalancesLock = await registry.methods
      .updateMemberBalancesLock(memberNonce)
      .accounts({
        registrar: registrar.publicKey,
        member: member.publicKey,
        memberSigner,
        balancesLocked,
      })
      .transaction();

    const wallet = provider.wallet as NodeWallet;

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
    await registry.methods
      .deposit(depositAmount)
      .accounts({
        depositor: god,
        depositorAuthority: provider.wallet.publicKey,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        vault: memberAccount.balances.vault,
        beneficiary: provider.wallet.publicKey,
        member: member.publicKey,
      })
      .rpc();

    const memberVault = await splToken.getAccount(
      provider.connection,
      memberAccount.balances.vault
    );
    assert.ok(new anchor.BN(memberVault.amount.toString()).eq(depositAmount));
  });

  it("Stakes to a member (unlocked)", async () => {
    const stakeAmount = new anchor.BN(10);
    await registry.methods
      .stake(stakeAmount)
      .accounts({
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
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
      })
      .rpc();

    const vault = await splToken.getAccount(
      provider.connection,
      memberAccount.balances.vault
    );
    const vaultStake = await splToken.getAccount(
      provider.connection,
      memberAccount.balances.vaultStake
    );
    const spt = await splToken.getAccount(
      provider.connection,
      memberAccount.balances.spt
    );

    assert.ok(new anchor.BN(vault.amount.toString()).eq(new anchor.BN(100)));
    assert.ok(
      new anchor.BN(vaultStake.amount.toString()).eq(new anchor.BN(20))
    );
    assert.ok(new anchor.BN(spt.amount.toString()).eq(new anchor.BN(10)));
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
    const [_vendorSigner, nonce] =
      await anchor.web3.PublicKey.findProgramAddress(
        [registrar.publicKey.toBuffer(), unlockedVendor.publicKey.toBuffer()],
        registry.programId
      );
    unlockedVendorSigner = _vendorSigner;

    await registry.methods
      .dropReward(
        rewardKind,
        rewardAmount,
        expiry,
        provider.wallet.publicKey,
        nonce
      )
      .accounts({
        registrar: registrar.publicKey,
        rewardEventQ: rewardQ.publicKey,
        poolMint,
        vendor: unlockedVendor.publicKey,
        vendorVault: unlockedVendorVault.publicKey,
        depositor: god,
        depositorAuthority: provider.wallet.publicKey,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([unlockedVendorVault, unlockedVendor])
      .preInstructions([
        ...(await createTokenAccountInstrs(
          provider,
          unlockedVendorVault.publicKey,
          mint,
          unlockedVendorSigner
        )),
        await registry.account.rewardVendor.createInstruction(unlockedVendor),
      ])
      .rpc();

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
    await registry.methods
      .claimReward()
      .accounts({
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
          tokenProgram: splToken.TOKEN_PROGRAM_ID,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
      })
      .rpc();

    let tokenAccount = await splToken.getAccount(provider.connection, token);
    assert.ok(
      new anchor.BN(tokenAccount.amount.toString()).eq(new anchor.BN(200))
    );

    const memberAccount = await registry.account.member.fetch(member.publicKey);
    assert.ok(memberAccount.rewardsCursor == 1);
  });

  const lockedVendor = Keypair.generate();
  const lockedVendorVault = Keypair.generate();
  let lockedVendorSigner = null;
  let lockedRewardAmount = null;
  let lockedRewardKind = null;

  it("Drops a locked reward", async () => {
    const slot = await connection.getSlot();
    const blocktime = await connection.getBlockTime(slot);
    const startTs = new anchor.BN(blocktime);
    const endTs = new anchor.BN(startTs.toNumber() + 6);
    lockedRewardKind = {
      locked: {
        startTs,
        endTs,
        periodCount: new anchor.BN(2),
      },
    };
    lockedRewardAmount = new anchor.BN(200);
    const expiry = new anchor.BN(Date.now() / 1000 + 5);
    const [_vendorSigner, nonce] =
      await anchor.web3.PublicKey.findProgramAddress(
        [registrar.publicKey.toBuffer(), lockedVendor.publicKey.toBuffer()],
        registry.programId
      );
    lockedVendorSigner = _vendorSigner;

    await registry.methods
      .dropReward(
        lockedRewardKind,
        lockedRewardAmount,
        expiry,
        provider.wallet.publicKey,
        nonce
      )
      .accounts({
        registrar: registrar.publicKey,
        rewardEventQ: rewardQ.publicKey,
        poolMint,
        vendor: lockedVendor.publicKey,
        vendorVault: lockedVendorVault.publicKey,
        depositor: god,
        depositorAuthority: provider.wallet.publicKey,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([lockedVendorVault, lockedVendor])
      .preInstructions([
        ...(await createTokenAccountInstrs(
          provider,
          lockedVendorVault.publicKey,
          mint,
          lockedVendorSigner
        )),
        await registry.account.rewardVendor.createInstruction(lockedVendor),
      ])
      .rpc();

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
    let [_vendoredVestingSigner, nonce] =
      await anchor.web3.PublicKey.findProgramAddress(
        [vendoredVesting.publicKey.toBuffer()],
        lockup.programId
      );
    vendoredVestingSigner = _vendoredVestingSigner;

    // Make remaining accounts for createVesting ix

    const remainingAccounts = [
      // vesting
      {
        pubkey: vendoredVesting.publicKey,
        isWritable: true,
        isSigner: false,
      },
      // vault
      {
        pubkey: vendoredVestingVault.publicKey,
        isWritable: true,
        isSigner: false,
      },
      // depositor
      {
        pubkey: lockedVendorVault.publicKey,
        isWritable: true,
        isSigner: false,
      },
      // depositorAuthority
      // Note: Change the signer status on the vendor signer since it's signed by the program, not the client.
      {
        pubkey: lockedVendorSigner,
        isWritable: false,
        isSigner: false,
      },
      // tokenProgram
      {
        pubkey: splToken.TOKEN_PROGRAM_ID,
        isWritable: false,
        isSigner: false,
      },
      // rent
      {
        pubkey: anchor.web3.SYSVAR_RENT_PUBKEY,
        isWritable: false,
        isSigner: false,
      },
      // clock
      {
        pubkey: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        isWritable: false,
        isSigner: false,
      },
    ];

    const sig = await registry.methods
      .claimRewardLocked(registryNonce, nonce)
      .accounts({
        registry: registryAddress,
        lockupProgram: lockup.programId,
        realizorProgram: realizor.programId,
        cmn: {
          registrar: registrar.publicKey,

          member: member.publicKey,
          beneficiary: provider.wallet.publicKey,
          balancesSpt: balances.spt,
          balancesLockedSpt: balancesLocked.spt,

          vendor: lockedVendor.publicKey,
          vault: lockedVendorVault.publicKey,
          vendorSigner: lockedVendorSigner,

          tokenProgram: splToken.TOKEN_PROGRAM_ID,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
      })
      .remainingAccounts(remainingAccounts)
      .signers([vendoredVesting, vendoredVestingVault])
      .preInstructions([
        await lockup.account.vesting.createInstruction(vendoredVesting),
        ...(await createTokenAccountInstrs(
          provider,
          vendoredVestingVault.publicKey,
          mint,
          vendoredVestingSigner
        )),
      ])
      .rpc();

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
    assert.ok(lockupAccount.realizor.program.equals(realizor.programId));
    assert.ok(lockupAccount.realizor.metadata.equals(member.publicKey));
  });

  it("Waits for the lockup period to pass", async () => {
    await sleep(10 * 1000);
  });

  it("Should fail to unlock an unrealized lockup reward", async () => {
    // Get Member account
    const memberAccount = await registry.account.member.fetch(member.publicKey);

    const token = await createTokenAccount(
      provider,
      mint,
      provider.wallet.publicKey
    );
    await assert.rejects(
      async () => {
        const withdrawAmount = new anchor.BN(10);
        await lockup.methods
          .withdraw(withdrawAmount)
          .accounts({
            vesting: vendoredVesting.publicKey,
            beneficiary: provider.wallet.publicKey,
            token,
            vault: vendoredVestingVault.publicKey,
            vestingSigner: vendoredVestingSigner,
            tokenProgram: splToken.TOKEN_PROGRAM_ID,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          })
          // TODO: trait methods generated on the client. Until then, we need to manually
          //       specify the account metas here.
          .remainingAccounts([
            {
              pubkey: realizor.programId,
              isWritable: false,
              isSigner: false,
            },
            { pubkey: registry.programId, isWritable: false, isSigner: false },
            // is_realized
            { pubkey: member.publicKey, isWritable: false, isSigner: false },
            { pubkey: balances.spt, isWritable: false, isSigner: false },
            { pubkey: balancesLocked.spt, isWritable: false, isSigner: false },
            // MemberData
            {
              pubkey: memberAccount.registrar,
              isWritable: false,
              isSigner: false,
            },
            {
              pubkey: memberAccount.beneficiary,
              isWritable: false,
              isSigner: false,
            },
            {
              pubkey: memberAccount.metadata,
              isWritable: false,
              isSigner: false,
            },
            {
              pubkey: memberAccount.balances.spt,
              isWritable: false,
              isSigner: false,
            },
            {
              pubkey: memberAccount.balancesLocked.spt,
              isWritable: false,
              isSigner: false,
            },
          ])
          .rpc();
      },
      (err: anchor.AnchorError) => {
        // Solana doesn't propagate errors across CPI. So we receive the registry's error code,
        // not the lockup's.
        // const errorCode = "custom program error: 0x65";
        // assert.ok(err.toString().split(errorCode).length === 2);
        assert.equal(err.error.errorCode.code, "UnrealizedReward");
        assert.equal(err.error.errorCode.number, 6000);
        return true;
      }
    );
  });

  const pendingWithdrawal = Keypair.generate();

  it("Unstakes (unlocked)", async () => {
    const unstakeAmount = new anchor.BN(10);

    await registry.methods
      .startUnstake(unstakeAmount, false)
      .accounts({
        registrar: registrar.publicKey,
        rewardEventQ: rewardQ.publicKey,
        poolMint,
        pendingWithdrawal: pendingWithdrawal.publicKey,
        member: member.publicKey,
        beneficiary: provider.wallet.publicKey,
        balances,
        memberSigner,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([pendingWithdrawal])
      .preInstructions([
        await registry.account.pendingWithdrawal.createInstruction(
          pendingWithdrawal
        ),
      ])
      .rpc();

    const vaultPw = await splToken.getAccount(
      provider.connection,
      memberAccount.balances.vaultPw
    );
    const vaultStake = await splToken.getAccount(
      provider.connection,
      memberAccount.balances.vaultStake
    );
    const spt = await splToken.getAccount(
      provider.connection,
      memberAccount.balances.spt
    );

    assert.ok(new anchor.BN(vaultPw.amount.toString()).eq(new anchor.BN(20)));
    assert.ok(new anchor.BN(vaultStake.amount.toString()).eq(new anchor.BN(0)));
    assert.ok(new anchor.BN(spt.amount.toString()).eq(new anchor.BN(0)));
  });

  it("Fails to end unstaking before timelock", async () => {
    await assert.rejects(
      async () => {
        await registry.methods
          .endUnstake()
          .accounts({
            registrar: registrar.publicKey,
            member: member.publicKey,
            beneficiary: provider.wallet.publicKey,
            pendingWithdrawal: pendingWithdrawal.publicKey,
            vault: balances.vault,
            vaultPw: balances.vaultPw,
            memberSigner,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
            tokenProgram: splToken.TOKEN_PROGRAM_ID,
          })
          .rpc();
      },
      (err: anchor.AnchorError) => {
        assert.equal(err.error.errorCode.code, "UnstakeTimelock");
        assert.equal(err.error.errorCode.number, 6009);
        assert.equal(
          err.error.errorMessage,
          "The unstake timelock has not yet expired."
        );
        return true;
      }
    );
  });

  it("Waits for the unstake period to end", async () => {
    await sleep(5000);
  });

  it("Unstake finalizes (unlocked)", async () => {
    await registry.methods
      .endUnstake()
      .accounts({
        registrar: registrar.publicKey,
        member: member.publicKey,
        beneficiary: provider.wallet.publicKey,
        pendingWithdrawal: pendingWithdrawal.publicKey,
        vault: balances.vault,
        vaultPw: balances.vaultPw,
        memberSigner,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
      })
      .rpc();

    const vault = await splToken.getAccount(
      provider.connection,
      memberAccount.balances.vault
    );
    const vaultPw = await splToken.getAccount(
      provider.connection,
      memberAccount.balances.vaultPw
    );

    assert.ok(new anchor.BN(vault.amount.toString()).eq(new anchor.BN(120)));
    assert.ok(new anchor.BN(vaultPw.amount.toString()).eq(new anchor.BN(0)));
  });

  it("Withdraws deposits (unlocked)", async () => {
    const token = await createTokenAccount(
      provider,
      mint,
      provider.wallet.publicKey
    );
    const withdrawAmount = new anchor.BN(100);
    await registry.methods
      .withdraw(withdrawAmount)
      .accounts({
        registrar: registrar.publicKey,
        member: member.publicKey,
        beneficiary: provider.wallet.publicKey,
        vault: memberAccount.balances.vault,
        memberSigner,
        depositor: token,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
      })
      .rpc();

    const tokenAccount = await splToken.getAccount(provider.connection, token);
    assert.ok(new anchor.BN(tokenAccount.amount.toString()).eq(withdrawAmount));
  });

  it("Should succesfully unlock a locked reward after unstaking", async () => {
    // Get Member account
    const memberAccount = await registry.account.member.fetch(member.publicKey);

    const token = await createTokenAccount(
      provider,
      mint,
      provider.wallet.publicKey
    );

    const withdrawAmount = new anchor.BN(7);
    await lockup.methods
      .withdraw(withdrawAmount)
      .accounts({
        vesting: vendoredVesting.publicKey,
        beneficiary: provider.wallet.publicKey,
        token,
        vault: vendoredVestingVault.publicKey,
        vestingSigner: vendoredVestingSigner,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      // TODO: trait methods generated on the client. Until then, we need to manually
      //       specify the account metas here.
      .remainingAccounts([
        {
          pubkey: realizor.programId,
          isWritable: false,
          isSigner: false,
        },
        { pubkey: registry.programId, isWritable: false, isSigner: false },
        // is_realized
        { pubkey: member.publicKey, isWritable: false, isSigner: false },
        { pubkey: balances.spt, isWritable: false, isSigner: false },
        { pubkey: balancesLocked.spt, isWritable: false, isSigner: false },
        // MemberData
        { pubkey: memberAccount.registrar, isWritable: false, isSigner: false },
        {
          pubkey: memberAccount.beneficiary,
          isWritable: false,
          isSigner: false,
        },
        { pubkey: memberAccount.metadata, isWritable: false, isSigner: false },
        {
          pubkey: memberAccount.balances.spt,
          isWritable: false,
          isSigner: false,
        },
        {
          pubkey: memberAccount.balancesLocked.spt,
          isWritable: false,
          isSigner: false,
        },
      ])
      .rpc();

    const tokenAccount = await splToken.getAccount(provider.connection, token);
    assert.ok(new anchor.BN(tokenAccount.amount.toString()).eq(withdrawAmount));
  });
});
