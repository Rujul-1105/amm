# AMM (Automated Market Maker)

> **All tests passing** — see [`Screenshot From 2026-06-08 15-18-43.png`](./Screenshot%20From%202026-06-08%2015-18-43.png) for proof (`anchor test` — 5 tests, 0 failures).

A Solana on-chain Automated Market Maker built with [Anchor](https://www.anchor-lang.com/) v1.0.2. The program implements a constant-product liquidity pool (`x * y = k`) for two SPL tokens, with LP token minting, swaps, and configurable fees.

**Program ID (localnet):** `D58puanNfoPFjurvZ1NqZ6ag2eJuCQZBdL565HvtYRG1`

## Features

| Instruction   | Description |
|---------------|-------------|
| `initialize`  | Create a new pool for a token pair, set fee and optional authority |
| `deposit`     | Add liquidity; receive LP tokens proportional to pool share |
| `withdraw`    | Burn LP tokens; receive underlying token X and Y |
| `swap`        | Trade token X for Y (or vice versa) along the constant-product curve |

- **Constant-product curve** via [`constant-product-curve`](https://github.com/deanmlittle/constant-product-curve)
- **Slippage protection** on deposit (`max_x`, `max_y`), withdraw (`min_x`, `min_y`), and swap (`min_amount_out`)
- **Configurable swap fee** (basis points, set at initialization)
- **Optional pool authority** and a `locked` flag on the config account

## Architecture

### Accounts

**Config** (PDA: `["config", seed.to_le_bytes()]`)
- Stores pool metadata: seed, mints, fee, authority, bumps, and lock state

**LP mint** (PDA: `["lp", config.key()]`)
- One LP token mint per pool; mint authority is the config PDA

**Vaults**
- Associated token accounts owned by the config PDA for `mint_x` and `mint_y`

### Instructions overview

```
initialize(seed, fee, authority)
  → Creates config PDA, LP mint, and vault ATAs

deposit(amount, max_x, max_y)
  → Transfers X/Y into vaults, mints LP tokens to user

withdraw(amount, min_x, min_y)
  → Burns LP tokens, transfers X/Y from vaults to user

swap(is_x, amount_in, min_amount_out)
  → Swaps along the curve; fee applied on output
```

## Project structure

```
amm/
├── programs/amm/
│   ├── src/
│   │   ├── lib.rs              # Program entrypoint & instruction dispatch
│   │   ├── state.rs            # Config account
│   │   ├── error.rs            # Custom errors
│   │   └── instructions/     # initialize, deposit, withdraw, swap
│   └── tests/
│       ├── test.rs             # LiteSVM integration tests
│       └── ix_handlers/        # Instruction builders for tests
├── Anchor.toml
├── Cargo.toml
└── migrations/
```

## Prerequisites

- [Rust](https://rustup.rs/) 1.89.0 (see `rust-toolchain.toml`)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) 0.31+
- [Yarn](https://yarnpkg.com/)

## Build & test

```bash
# Build the program
anchor build

# Run all tests (unit + LiteSVM integration)
anchor test
```

Tests use [LiteSVM](https://github.com/LiteSVM/LiteSVM) for fast in-process simulation — no local validator required.

| Test | What it covers |
|------|----------------|
| `test_initialize` | Pool creation, PDAs, vault setup |
| `test_deposit` | Initial liquidity deposit and LP minting |
| `test_withdraw` | LP burn and token redemption |
| `test_swap` | Constant-product swap between X and Y |

## Dependencies

- `anchor-lang` / `anchor-spl` 1.0.2
- `constant-product-curve` (git)
- `litesvm` + `litesvm-token` (dev, for tests)
