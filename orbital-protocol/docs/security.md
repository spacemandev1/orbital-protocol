# Security checklist

Before handling real value:

- audit every PDA seed and signer constraint
- add slippage protection to every trade
- define exact integer rounding behavior
- test reserve underflow/overflow boundaries
- verify token mint ownership
- verify token-program IDs
- use checked token CPIs
- prevent vault spoofing
- prevent account substitution
- prevent creator fee overflow
- cap all fee parameters
- test duplicate initialization paths
- test pause/finalize state transitions
- fuzz constant-product invariants
- run local-validator integration tests
- obtain an independent audit
