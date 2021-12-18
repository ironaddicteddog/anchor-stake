# Anchor Stake Program

- This codebase is evolved from this [Stake program](https://github.com/project-serum/stake) developed by Serum with some changes:
  - Upgrade Anchor to latest version
  - Replace to-be-deprecated syntax such as `#[state]`
  - Optimize some function invocation to avoid stack frame limit
- See this [doc](https://github.com/project-serum/stake/blob/master/docs/staking.md) to learn more on the architecture of this program

## Build, Deploy and Test

Run the test validator:

```
$ solana-test-validator -r
```

Run the test:

```
$ anchor test --skip-local-validator
```
