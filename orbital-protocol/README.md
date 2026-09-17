# ORBITAL

Minimal Solana launchpad protocol starter.

Orbital is an Anchor-based example architecture for:
- platform configuration
- launch creation
- deterministic launch PDAs
- simple virtual bonding-curve quoting
- buy/sell accounting
- launch finalization
- TypeScript SDK helpers
- CLI examples

> This repository is a development starter, not audited production code.
> Do not deploy with real funds before adding token CPIs, vault accounting,
> slippage protection, migration logic, access-control review, invariant tests,
> fuzzing, and an independent security audit.

## Repository layout

```text
programs/orbital/        Anchor program
sdk/                     TypeScript SDK
cli/                     CLI examples
tests/                   Anchor integration tests
docs/                    Architecture notes
```

## Program model

Each launch is a PDA derived from:

```text
["launch", creator, mint]
```

The current starter stores virtual SOL/token reserves in program state and uses
a constant-product quote model for deterministic examples.

The starter intentionally does **not** custody real SOL or SPL tokens yet.
That keeps the example small enough to understand and audit before adding CPIs.

## Commands

```bash
npm install
anchor build
anchor test
```

Run the CLI:

```bash
npm run cli -- create
```

## Next production steps

1. Add SPL/Token-2022 mint creation.
2. Create SOL and token vault PDAs.
3. Transfer assets with checked CPIs.
4. Add `min_out` slippage guards.
5. Add protocol/creator fee accounting.
6. Add liquidity migration.
7. Add pause/emergency controls.
8. Add property/fuzz tests.
9. Audit before mainnet.
