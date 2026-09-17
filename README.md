# ORBITAL

<p align="center">
  <strong>Programmable launch infrastructure for Solana.</strong>
</p>

<p align="center">
  Experimental protocol architecture for deterministic, configurable, and composable token launches on the SVM.
</p>

---

# Overview

Orbital is an experimental Solana protocol designed around programmable token launches.

The project explores how token deployment, launch configuration, pricing mechanics, lifecycle transitions, fee routing, creator permissions, liquidity management, Token-2022 functionality, and future migration logic can be represented as explicit on-chain state.

Instead of treating a token launch as a single opaque action, Orbital models a launch as a collection of protocol primitives.

Conceptually:

```text
Launch
│
├── Creator
├── Token Mint
├── Metadata
├── Launch Configuration
├── Pricing Configuration
├── Virtual Reserves
├── Trading State
├── Creator Configuration
├── Protocol Configuration
├── Fee Configuration
├── Vault State
├── Migration State
├── Authority State
├── Lifecycle State
└── Finalization State
```

The current repository is an early protocol implementation and development scaffold.

It currently focuses on:

```text
global platform state
deterministic launch accounts
launch initialization
virtual reserve accounting
constant-product quote logic
buy state transitions
sell state transitions
slippage checks
launch finalization
TypeScript SDK helpers
PDA derivation
CLI development helpers
basic testing
```

The current implementation does **not** yet custody real SOL or real SPL tokens.

Production asset movement, vaults, Token-2022 mint creation, DEX liquidity migration, fee settlement, liquidity locking, and related functionality remain future work.

---

# Mission

Orbital is built around a simple idea:

```text
Launch mechanics should be explicit.

Launch configuration should be inspectable.

Protocol state should be deterministic.

Creator permissions should be visible.

Trading logic should be reproducible.

Critical behavior should live on-chain.
```

A user or developer should ideally be able to inspect a launch and independently determine:

```text
Who created it?

What mint does it use?

What curve does it use?

What are the current reserves?

What fees exist?

What creator privileges exist?

What authorities remain?

Is the launch active?

Has it finalized?

Has it migrated?

What conditions determine migration?

Can the configuration change?
```

Orbital aims to make those answers part of protocol state rather than undocumented frontend assumptions.

---

# Project Status

Orbital is currently:

```text
Version:       0.1.0
Stage:         Experimental
Network:       Development
Program Type:  Anchor / Solana
Mainnet:       Not production ready
Audit Status:  Unaudited
```

The repository should currently be treated as:

```text
protocol research
architecture experimentation
development scaffolding
SDK experimentation
curve logic experimentation
Solana launch infrastructure research
```

It should **not** currently be treated as audited production financial software.

---

# Technology

Orbital is currently built around:

```text
Solana
SVM
Rust
Anchor
TypeScript
Node.js
SPL-compatible architecture
Token-2022-oriented architecture
Program Derived Addresses
Integer-based financial arithmetic
Virtual reserve pricing
```

Potential future infrastructure may include:

```text
Yellowstone Geyser
PostgreSQL
Redis
WebSockets
gRPC
Next.js
React
Solana RPC
Jito infrastructure
DEX integrations
analytics services
indexing infrastructure
```

These future components are not all currently part of this repository.

---

# Design Goals

Orbital is being designed around several long-term goals.

## Deterministic

Core protocol accounts should use deterministic PDA derivation.

Example:

```text
creator
+
mint
+
Orbital program
=
launch account
```

This reduces dependence on centralized lookup systems for core state.

---

## Transparent

Important launch configuration should be visible on-chain.

Examples may eventually include:

```text
launch mode
creator fee
protocol fee
curve configuration
token configuration
authority state
migration configuration
creator allocation
launch status
```

---

## Composable

Third-party software should eventually be able to integrate with Orbital without depending on the Orbital frontend.

Possible integrations include:

```text
wallets
bots
trading terminals
DEX aggregators
portfolio trackers
analytics dashboards
token explorers
launch aggregators
mobile applications
developer SDKs
indexers
```

---

## Modular

Launch mechanics should be separable where possible.

Possible future architecture:

```text
orbital-core
orbital-launch
orbital-curves
orbital-fees
orbital-migration
orbital-token22
orbital-indexer
orbital-sdk
orbital-cli
```

---

## Inspectable

Launch behavior should be understandable from:

```text
program code
account state
transaction history
protocol events
```

rather than relying solely on frontend labels.

---

## Extensible

Orbital should eventually support multiple launch modes without requiring an entirely separate protocol for every new launch mechanic.

---

# High-Level Architecture

The simplest architecture is:

```text
┌─────────────────────────────────────────┐
│              Orbital UI                 │
│                                         │
│ Launch                                  │
│ Explore                                 │
│ Trade                                   │
│ Portfolio                               │
│ Analytics                               │
└───────────────────┬─────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────┐
│              Orbital SDK                │
│                                         │
│ PDA Derivation                          │
│ Account Parsing                         │
│ Quotes                                  │
│ Transaction Builders                    │
│ Launch Helpers                          │
└───────────────────┬─────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────┐
│           Orbital Program               │
│                                         │
│ Platform Configuration                  │
│ Launch State                            │
│ Trading State                           │
│ Curve Accounting                        │
│ Lifecycle Transitions                   │
│ Finalization                            │
└───────────────────┬─────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────┐
│              Solana SVM                 │
└─────────────────────────────────────────┘
```

A more complete future architecture could look like:

```text
                           USERS
                             │
                             ▼
                  ┌─────────────────────┐
                  │    Orbital UI       │
                  └──────────┬──────────┘
                             │
                 ┌───────────┴────────────┐
                 │                        │
                 ▼                        ▼
        ┌────────────────┐       ┌────────────────┐
        │  Orbital API   │       │  Orbital SDK   │
        └───────┬────────┘       └───────┬────────┘
                │                        │
                ▼                        ▼
        ┌────────────────┐       ┌────────────────┐
        │ Orbital Indexer│       │ Orbital Program│
        └───────┬────────┘       └───────┬────────┘
                │                        │
        ┌───────┴────────┐               │
        │                │               │
        ▼                ▼               ▼
 ┌─────────────┐  ┌─────────────┐  ┌──────────────┐
 │ PostgreSQL  │  │    Redis    │  │ Solana / SVM │
 └─────────────┘  └─────────────┘  └──────┬───────┘
                                          │
                                          ▼
                                 ┌──────────────────┐
                                 │ Geyser / RPC     │
                                 └──────────────────┘
```

---

# Repository Structure

Current repository layout:

```text
orbital-protocol/
│
├── programs/
│   └── orbital/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── state.rs
│           ├── math.rs
│           ├── constants.rs
│           ├── errors.rs
│           └── instructions/
│               ├── mod.rs
│               ├── initialize_platform.rs
│               ├── create_launch.rs
│               ├── trade.rs
│               └── finalize_launch.rs
│
├── sdk/
│   └── src/
│       ├── index.ts
│       ├── pdas.ts
│       ├── quotes.ts
│       └── types.ts
│
├── cli/
│   └── src/
│       └── index.ts
│
├── tests/
│   └── orbital.ts
│
├── docs/
│   ├── architecture.md
│   └── security.md
│
├── assets/
│   └── orbital-logo.png
│
├── Anchor.toml
├── Cargo.toml
├── package.json
├── tsconfig.json
├── .gitignore
├── LICENSE
└── README.md
```

---

# Program Overview

The current Orbital program exposes several basic instructions.

```text
initialize_platform
create_launch
buy
sell
finalize_launch
```

These instructions represent the initial protocol state machine.

Conceptually:

```text
Orbital
│
├── Platform
│   │
│   └── initialize_platform
│
├── Launch
│   │
│   └── create_launch
│
├── Trading
│   │
│   ├── buy
│   └── sell
│
└── Lifecycle
    │
    └── finalize_launch
```

---

# Platform Account

Orbital maintains a global platform configuration PDA.

Seeds:

```text
["platform"]
```

Current state:

```text
PlatformConfig
│
├── authority
├── protocol_fee_bps
├── launch_count
└── bump
```

The global configuration account represents protocol-wide state.

Potential future values may include:

```text
protocol treasury
protocol version
default creator fee
maximum creator fee
maximum protocol fee
pause authority
supported token programs
supported launch modes
supported curve models
supported migration targets
default migration configuration
protocol state
```

---

# Launch Account

Every launch receives its own deterministic account.

Current seeds:

```text
[
    "launch",
    creator,
    mint
]
```

Current state:

```text
Launch
│
├── creator
├── mint
├── name
├── symbol
├── uri
├── virtual_sol_reserve
├── virtual_token_reserve
├── total_tokens_sold
├── total_sol_volume
├── creator_fee_bps
├── active
├── finalized
└── bump
```

---

# Why Use PDAs?

Program Derived Addresses provide deterministic protocol-owned addresses.

Instead of storing every launch address in a centralized database, a client can derive launch state from known inputs.

Example:

```text
Creator:
ABC...

Mint:
XYZ...

Seeds:
["launch", creator, mint]

Result:
Launch PDA
```

Benefits include:

```text
deterministic discovery
account ownership guarantees
program-controlled authorities
reduced dependence on centralized registries
predictable account relationships
```

---

# Proposed PDA Architecture

Future Orbital versions may use a larger PDA hierarchy.

```text
Orbital Program
│
├── Platform PDA
│   │
│   ├── Treasury PDA
│   └── Protocol Config
│
└── Launch PDA
    │
    ├── SOL Vault PDA
    ├── Token Vault PDA
    ├── Creator Fee Vault PDA
    ├── Protocol Fee Vault PDA
    ├── Migration PDA
    ├── Curve PDA
    └── Metadata State PDA
```

Possible seeds:

```text
Platform:
["platform"]

Launch:
["launch", creator, mint]

SOL Vault:
["sol-vault", launch]

Token Vault:
["token-vault", launch]

Creator Fee Vault:
["creator-fees", launch]

Migration:
["migration", launch]

Curve:
["curve", launch]
```

This architecture is conceptual and not fully implemented.

---

# Launch Lifecycle

Orbital aims to model launches as explicit lifecycle transitions.

Potential lifecycle:

```text
UNINITIALIZED
      │
      ▼
INITIALIZED
      │
      ▼
WAITING
      │
      ▼
ACTIVE
      │
      ├───────────────┐
      │               │
      ▼               ▼
   PAUSED          COMPLETE
                      │
                      ▼
                  MIGRATING
                      │
                      ▼
                   MIGRATED
                      │
                      ▼
                  FINALIZED
```

The current implementation is simpler.

Current behavior:

```text
CREATE LAUNCH
      │
      ▼
ACTIVE
      │
      ├── BUY
      ├── SELL
      │
      ▼
FINALIZE
```

---

# Current Trading Model

Orbital currently implements virtual constant-product accounting.

Two reserves are tracked:

```text
x = virtual SOL reserve
y = virtual token reserve
```

The system models:

```text
k = x * y
```

where:

```text
k = constant product
```

The current implementation updates state based on this relationship.

No actual SOL or token transfer occurs in the current starter.

---

# Buy Calculation

For a buy:

```text
x = current virtual SOL reserve

y = current virtual token reserve

dx = incoming SOL
```

Calculate:

```text
k = x * y
```

Then:

```text
new_x = x + dx
```

Then:

```text
new_y = k / new_x
```

Token output:

```text
tokens_out = y - new_y
```

Visualized:

```text
             SOL IN
               │
               ▼
       ┌─────────────────┐
       │  ORBITAL CURVE  │
       │                 │
       │  x increases    │
       │  y decreases    │
       └────────┬────────┘
                │
                ▼
            TOKENS OUT
```

---

# Sell Calculation

For a sell:

```text
x = current virtual SOL reserve

y = current virtual token reserve

dy = incoming tokens
```

Calculate:

```text
k = x * y
```

Then:

```text
new_y = y + dy
```

Then:

```text
new_x = k / new_y
```

SOL output:

```text
sol_out = x - new_x
```

Visualized:

```text
           TOKENS IN
               │
               ▼
       ┌─────────────────┐
       │  ORBITAL CURVE  │
       │                 │
       │  y increases    │
       │  x decreases    │
       └────────┬────────┘
                │
                ▼
              SOL OUT
```

---

# Virtual Reserves

Virtual reserves are accounting values used to determine pricing behavior.

They do not necessarily correspond directly to real assets held by a vault.

Virtual reserves may allow launch designers to influence:

```text
starting price
price sensitivity
effective market depth
curve steepness
initial price impact
migration behavior
```

---

# Larger Virtual Reserves

Larger reserves generally imply:

```text
more apparent depth
lower price impact for a given trade
slower curve movement
```

Conceptually:

```text
VERY LARGE RESERVES
        │
        ▼
SMALL TRADE RELATIVE TO RESERVES
        │
        ▼
SMALL PRICE CHANGE
```

---

# Smaller Virtual Reserves

Smaller reserves generally imply:

```text
less apparent depth
greater price impact
faster price movement
```

Conceptually:

```text
SMALL RESERVES
       │
       ▼
TRADE REPRESENTS LARGE % OF RESERVE
       │
       ▼
LARGER PRICE CHANGE
```

---

# Curve Price

A simplified conceptual spot relationship may be described as:

```text
price ≈ virtual_sol_reserve / virtual_token_reserve
```

However, users do not necessarily receive the current spot price for the entire trade.

A trade executes across the changing curve.

Actual output depends on:

```text
initial reserves
trade size
final reserves
integer rounding
fees
slippage configuration
```

---

# Marginal Price

As tokens are bought:

```text
virtual SOL reserve increases
virtual token reserve decreases
```

The implied marginal token price therefore rises.

As tokens are sold:

```text
virtual token reserve increases
virtual SOL reserve decreases
```

The implied marginal token price falls.

---

# Price Impact

Trade size relative to reserves determines price impact.

Example:

```text
Reserves:
1000 units

Trade:
1 unit

Relative size:
small
```

Price impact may be small.

Compared with:

```text
Reserves:
1000 units

Trade:
300 units

Relative size:
large
```

Price impact may be substantial.

---

# Slippage Protection

The current trading functions include minimum output parameters.

For buys:

```text
sol_in
min_tokens_out
```

For sells:

```text
tokens_in
min_sol_out
```

If the calculated output is lower than the user's specified minimum:

```text
transaction fails
```

Example:

```text
Quote:

10,000 tokens

User minimum:

9,800 tokens

Execution output:

9,750 tokens

Result:

REJECTED
```

---

# Why Slippage Exists

Between transaction creation and execution, state may change.

For example:

```text
User A receives quote
        │
        ▼
User B trades first
        │
        ▼
Curve changes
        │
        ▼
User A transaction executes
```

Without a minimum output check, User A could receive significantly less than expected.

---

# Current Fee State

Orbital currently stores:

```text
protocol_fee_bps
```

and:

```text
creator_fee_bps
```

The current development scaffold does not yet implement full real-asset fee settlement.

---

# Basis Points

Orbital uses basis-point-style configuration.

```text
1 bps     = 0.01%
10 bps    = 0.10%
25 bps    = 0.25%
50 bps    = 0.50%
100 bps   = 1.00%
500 bps   = 5.00%
1000 bps  = 10.00%
```

---

# Future Fee Architecture

A future trade could conceptually route value like:

```text
                    TRADE INPUT
                         │
            ┌────────────┼────────────┐
            │            │            │
            ▼            ▼            ▼
        CURVE        PROTOCOL       CREATOR
       LIQUIDITY        FEE            FEE
                         │              │
                         ▼              ▼
                     TREASURY      FEE VAULT
```

Potential fee categories:

```text
protocol fee
creator fee
referral fee
integrator fee
migration fee
launch creation fee
```

---

# Fee Requirements

Any production fee system should make the following explicit:

```text
fee numerator
fee denominator
maximum fee
fee recipient
fee claim rules
fee mutability
fee timing
rounding rules
```

---

# Creator Configuration

A future launch creator may be able to configure:

```text
name
symbol
metadata
launch mode
creator fee
initial curve configuration
launch timing
migration target
creator allocation
```

---

# Creator Privileges

Orbital aims to make creator privileges visible.

Possible frontend representation:

```text
Creator Mint Authority:
Revoked

Freeze Authority:
None

Metadata Mutable:
No

Creator Allocation:
2.00%

Creator Fee:
0.50%

Curve:
Standard

Migration:
Automatic
```

---

# Creator Restrictions

Certain launch modes may restrict creators.

Example:

```text
Fair Launch

Creator mint authority:          not permitted
Creator freeze authority:        not permitted
Creator allocation:              capped
Creator fee:                     capped
Mutable supply:                  not permitted
Curve editing after activation:  not permitted
```

---

# Fair Launch Mode

A future Orbital fair launch mode may define a stricter launch configuration.

Potential requirements:

```text
fixed supply
fixed curve
fixed fee limits
transparent creator allocation
revoked mint authority
no freeze authority
immutable launch parameters
transparent initial creator buy
automatic activation
```

The exact rules have not yet been finalized.

---

# Standard Launch Mode

A standard launch mode might offer greater configurability.

Potential options:

```text
custom metadata
creator fee
supported curve preset
migration target
launch timing
```

---

# Timed Launch Mode

A future timed launch could use:

```text
creation slot
activation slot
```

Lifecycle:

```text
CREATED
   │
   ▼
WAITING
   │
   │ current_slot < activation_slot
   │
   ▼
ACTIVATION SLOT
   │
   ▼
ACTIVE
```

---

# Fixed Allocation Mode

Future launches could define predetermined allocation buckets.

Example:

```text
Total Supply
│
├── 80% Bonding Curve
├── 10% Migration Liquidity
├── 5% Creator
└── 5% Ecosystem
```

Any allocation model should be represented explicitly in state.

---

# Auction Launch Research

Orbital may eventually research auction-based token launches.

Potential lifecycle:

```text
REGISTRATION
     │
     ▼
COMMITMENTS
     │
     ▼
PRICE DISCOVERY
     │
     ▼
CLEARING PRICE
     │
     ▼
ALLOCATION
     │
     ▼
SECONDARY TRADING
```

This is research only and is not currently implemented.

---

# Batch Launch Research

A possible batch model could allow users to submit orders during a launch window.

Orders could then settle at one clearing price.

Potential benefits include:

```text
reduced ordering advantage
single clearing price
transparent allocation
```

This is also not implemented.

---

# Token Mint Architecture

A production token launch will require explicit mint handling.

Potential creation flow:

```text
CREATE LAUNCH CONFIG
        │
        ▼
CREATE MINT
        │
        ▼
CONFIGURE TOKEN PROGRAM
        │
        ▼
INITIALIZE METADATA
        │
        ▼
MINT SUPPLY
        │
        ▼
CREATE VAULTS
        │
        ▼
TRANSFER INVENTORY
        │
        ▼
REVOKE / TRANSFER AUTHORITIES
        │
        ▼
ACTIVATE
```

---

# Mint Authority

Mint authority must be handled explicitly.

Possible configurations:

```text
revoked immediately

temporarily held by protocol PDA

retained until initial supply mint completes

transferred to another deterministic authority
```

Permissionless trading systems should avoid undocumented mint authority.

---

# Freeze Authority

Freeze authority should also be explicit.

Potential safe launch rule:

```text
freeze authority = none
```

Alternative controlled modes would require clear disclosure.

---

# Metadata

Future token metadata may include:

```text
name
symbol
image
description
website
social links
metadata URI
```

Metadata mutability should be explicit.

---

# Token-2022

Orbital is designed with future Token-2022 experimentation in mind.

Token-2022 introduces extensions that can modify token behavior.

Potential extensions include:

```text
Transfer Fees
Transfer Hooks
Metadata Pointer
Permanent Delegate
Mint Close Authority
Non-Transferable Tokens
Interest Bearing
Default Account State
Confidential Transfers
CPI Guard
Memo Transfer
```

---

# Token-2022 Safety

Token-2022 extensions can introduce behavior traders may not expect.

A launch protocol should therefore not automatically accept every possible extension.

Orbital may eventually maintain:

```text
supported extensions
unsupported extensions
allowed extension combinations
launch-mode-specific extension policies
```

---

# Token-2022 Validation Flow

Potential future flow:

```text
READ TOKEN PROGRAM
        │
        ▼
READ EXTENSIONS
        │
        ▼
VALIDATE EXTENSIONS
        │
        ├── unsupported
        │      │
        │      ▼
        │    REJECT
        │
        ▼
SUPPORTED
        │
        ▼
CREATE LAUNCH
```

---

# Token-2022 Launch Profiles

Possible future profiles:

```text
STANDARD_TOKEN_2022

TRANSFER_FEE_TOKEN

METADATA_POINTER_TOKEN

CUSTOM_SUPPORTED_EXTENSIONS
```

---

# Vault Architecture

Real token trading requires actual custody.

Potential architecture:

```text
Launch PDA
│
├── SOL Vault
├── Token Vault
├── Protocol Fee Vault
├── Creator Fee Vault
└── Migration Vault
```

---

# SOL Vault

Potential responsibilities:

```text
receive SOL from buys
pay SOL during sells
hold curve liquidity
provide migration liquidity
```

---

# Token Vault

Potential responsibilities:

```text
hold curve inventory
send tokens during buys
receive tokens during sells
reserve migration allocation
```

---

# Protocol Fee Vault

Potential responsibilities:

```text
accumulate protocol fees
settle protocol revenue
provide transparent fee accounting
```

---

# Creator Fee Vault

Potential responsibilities:

```text
accumulate creator revenue
separate creator fees from curve liquidity
allow deterministic fee accounting
```

---

# Vault Security

Vault validation is critical.

Potential invariants:

```text
vault PDA matches launch

vault mint matches launch mint

vault authority matches expected PDA

vault program owner is correct

arbitrary vault substitution is impossible

creator cannot withdraw curve reserves

user cannot replace destination vault
```

---

# Future Buy Flow

A production buy might look like:

```text
USER
 │
 │ SOL
 ▼
SOL VAULT
 │
 │
 │ update reserve accounting
 │
 ▼
ORBITAL PROGRAM
 │
 │
 │ determines token output
 │
 ▼
TOKEN VAULT
 │
 │ tokens
 ▼
USER TOKEN ACCOUNT
```

---

# Future Sell Flow

A production sell might look like:

```text
USER TOKEN ACCOUNT
 │
 │ tokens
 ▼
TOKEN VAULT
 │
 ▼
ORBITAL PROGRAM
 │
 │ calculates SOL output
 ▼
SOL VAULT
 │
 │ SOL
 ▼
USER
```

---

# Associated Token Accounts

A production implementation may use associated token accounts for user balances.

Validation should ensure:

```text
correct mint
correct token program
correct owner
correct ATA derivation
```

---

# Curve Completion

A launch may eventually have a predefined completion condition.

Potential conditions:

```text
token inventory threshold
SOL reserve threshold
market-cap threshold
number of tokens sold
time-based threshold
explicit fixed target
```

---

# Migration

After curve completion, a launch may migrate liquidity to an external market.

Conceptually:

```text
CURVE ACTIVE
     │
     ▼
COMPLETION THRESHOLD
     │
     ▼
LOCK CURVE
     │
     ▼
CALCULATE MIGRATION ASSETS
     │
     ▼
CREATE DEX POOL
     │
     ▼
DEPOSIT LIQUIDITY
     │
     ▼
STORE LP STATE
     │
     ▼
FINALIZE MIGRATION
```

---

# Migration State

Possible future migration state:

```text
Migration
│
├── launch
├── destination
├── migration_threshold
├── token_amount
├── sol_amount
├── pool
├── lp_mint
├── lp_destination
├── status
└── bump
```

---

# Migration Destinations

Potential integrations could eventually include supported Solana liquidity venues.

Each external integration would require:

```text
program validation
account validation
pool validation
mint validation
slippage protection
CPI safety
migration testing
```

No specific production DEX integration is currently implemented.

---

# Liquidity Ownership

Post-migration LP ownership should be explicit.

Possible designs:

```text
LP burned

LP locked

LP controlled by protocol

LP sent to creator

LP sent to governance
```

Different launch modes may use different policies.

---

# Liquidity Locking

A launchpad should clearly expose liquidity handling.

Example display:

```text
Migration Status:
Complete

LP Tokens:
Locked

Lock Duration:
Permanent

Creator Withdrawal:
Disabled
```

---

# Launch Finalization

The current program includes creator-controlled finalization.

Current behavior:

```text
active = false

finalized = true
```

Future versions may make finalization automatic based on protocol conditions.

---

# Automatic Finalization

Possible future conditions:

```text
if curve completion threshold reached:
    active = false
    migration_ready = true
```

This reduces reliance on the creator manually changing launch state.

---

# Events

Orbital currently emits protocol events.

Current event examples:

```text
BuyEvent
SellEvent
LaunchFinalizedEvent
```

---

# Buy Event

Conceptually:

```text
BuyEvent
│
├── launch
├── user
├── sol_in
└── tokens_out
```

---

# Sell Event

Conceptually:

```text
SellEvent
│
├── launch
├── user
├── tokens_in
└── sol_out
```

---

# Launch Finalized Event

Conceptually:

```text
LaunchFinalizedEvent
│
├── launch
└── creator
```

---

# Future Events

Potential future events:

```text
PlatformInitialized

LaunchCreated

LaunchActivated

TradeExecuted

CreatorFeeAccrued

ProtocolFeeAccrued

CurveCompleted

MigrationStarted

MigrationCompleted

LaunchPaused

LaunchResumed

AuthorityRevoked
```

---

# Event Indexing

Events can support off-chain systems.

Example pipeline:

```text
SOLANA
  │
  ▼
PROGRAM LOGS
  │
  ▼
GEYSER / RPC
  │
  ▼
ORBITAL INDEXER
  │
  ▼
NORMALIZED EVENTS
  │
  ├── DATABASE
  ├── API
  └── WEBSOCKET
```

---

# Orbital SDK

The repository contains a TypeScript SDK layer.

Current functionality includes:

```text
PDA derivation
buy quote helper
sell quote helper
shared types
program ID constant
```

---

# SDK Design Goals

The SDK should eventually provide:

```text
PDA derivation

account fetching

account parsing

launch creation

buy transaction construction

sell transaction construction

slippage calculations

quote calculations

fee calculations

migration inspection

launch discovery

event parsing

transaction simulation

priority fee helpers
```

---

# Example SDK Usage

```ts
import {
  findPlatformPda,
  findLaunchPda,
  quoteBuy,
  quoteSell,
} from "./sdk/src/index.js";
```

---

# Platform PDA

```ts
const [platform, bump] = findPlatformPda();

console.log(platform.toBase58());
console.log(bump);
```

---

# Launch PDA

```ts
const [launch, bump] = findLaunchPda(
  creatorPublicKey,
  mintPublicKey
);

console.log(launch.toBase58());
```

---

# Buy Quote

```ts
const tokensOut = quoteBuy(
  virtualSolReserve,
  virtualTokenReserve,
  solIn
);
```

---

# Sell Quote

```ts
const solOut = quoteSell(
  virtualSolReserve,
  virtualTokenReserve,
  tokensIn
);
```

---

# Quote Matching

Client-side quote functions should closely match on-chain arithmetic.

Differences between frontend and on-chain arithmetic can create:

```text
failed transactions
unexpected output
incorrect slippage estimates
incorrect price impact
```

Therefore the SDK and program should use equivalent formulas.

---

# CLI

Orbital includes a development CLI.

Current commands include examples for:

```text
platform PDA derivation
launch PDA derivation
temporary development key generation
```

---

# Platform Command

Example:

```bash
npm run cli -- platform
```

Possible output:

```json
{
  "platform": "PDA_ADDRESS",
  "bump": 255
}
```

---

# Create Command

Example:

```bash
npm run cli -- create
```

The current CLI generates temporary development keys and derives a launch PDA.

These are runtime-generated development keys.

They are not hardcoded project wallets.

---

# Future CLI

Potential future CLI:

```text
orbital config show

orbital platform show

orbital launch create

orbital launch inspect

orbital launch activate

orbital launch finalize

orbital quote buy

orbital quote sell

orbital trade buy

orbital trade sell

orbital migration inspect

orbital token inspect

orbital authority inspect
```

---

# Indexer

A production interface typically needs faster querying than repeatedly scanning all Solana accounts.

Orbital may eventually use an indexer.

Possible responsibilities:

```text
discover launches
store launch metadata
store trades
calculate volume
calculate market statistics
track migration state
track wallet activity
produce OHLC candles
track creator activity
serve rankings
```

---

# Indexer Architecture

Possible architecture:

```text
Solana
  │
  ▼
Yellowstone Geyser
  │
  ▼
Orbital Event Consumer
  │
  ▼
Normalization
  │
  ├─────────────┐
  │             │
  ▼             ▼
PostgreSQL     Redis
  │             │
  └──────┬──────┘
         ▼
    Orbital API
```

---

# Geyser

Geyser infrastructure may provide lower-latency streaming access to Solana state.

Potential data streams:

```text
transactions
account updates
slots
blocks
program logs
```

---

# RPC

Standard RPC may still be useful for:

```text
account reads
transaction simulation
transaction submission
historical lookup
signature status
