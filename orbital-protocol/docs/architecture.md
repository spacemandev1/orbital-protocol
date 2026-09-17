# Orbital architecture

## Accounts

### PlatformConfig

Global protocol parameters.

```text
PDA = ["platform"]
```

### Launch

Per-launch state.

```text
PDA = ["launch", creator, mint]
```

## Starter trading model

The example uses virtual reserves:

```text
x = virtual SOL
y = virtual token reserve
k = x * y
```

For a buy:

```text
new_x = x + sol_in
new_y = k / new_x
tokens_out = y - new_y
```

For a sell:

```text
new_y = y + tokens_in
new_x = k / new_y
sol_out = x - new_x
```

No real assets move in the starter. It is an accounting/state-machine example.

## Production design additions

A production implementation should include:

- SPL or Token-2022 mint validation
- associated token accounts
- token vault PDA
- SOL vault PDA
- checked token transfers
- checked SOL transfers
- protocol fee vault
- creator fee vault
- explicit rounding rules
- supply caps
- migration threshold
- DEX migration
- authority revocation
- emergency pause
- oracle-independent invariant tests
