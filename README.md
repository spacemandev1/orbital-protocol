# ORBITAL

**Programmable launch infrastructure for Solana.**

Orbital is an experimental Solana protocol for creating, configuring, and coordinating permissionless token launches.

The protocol is designed around a modular launch architecture where deployment logic, launch configuration, pricing mechanics, liquidity state, and future migration behavior can be separated into explicit on-chain components.

Orbital is currently a development-stage protocol scaffold built with Rust, Anchor, and TypeScript.

---

## Overview

Most token launch systems combine a large amount of logic into a single launch flow.

Orbital takes a more modular approach.

A launch can be thought of as a collection of independently configurable components:

```text
Launch
│
├── Token Mint
├── Launch State
├── Virtual Reserves
├── Pricing Curve
├── Creator Configuration
├── Fee Configuration
├── Vault State
├── Migration Configuration
└── Finalization State
