# GhostKeys Vault Canister – Changelog (Aug–Sep 2025)

## Overview
- From mid-August onward the vault canister evolved from an initial prototype into a stable-memory backed service aligned with the `@ghostkeys/ghostkeys-sdk` serial protocol.
- Focus areas: stable storage migration, vetKD-centric key flows, richer sync/query APIs, and comprehensive documentation of vault data structures.

## Data Model & Stable Storage
- Replatformed vault data onto `StableBTreeMap` collections for spreadsheets, login metadata, secure notes, and vault names to guarantee upgrade safety (`bc1df80`, `fd780aa`, `914584f`).
- Introduced cascading delete semantics so removing metadata columns also purges orphaned logins (`2d2fe8b`) and tightened delete semantics across cell-based structures via backend deserialisers.
- Added dedicated column metadata allowing usernames and passwords to live in distinct columns, complete with obscured/visible flags for UIs (`b07dc20`, `4654b67`).
- Stored vault display names alongside principals to support discovery flows and cross-canister lookups (`a639904`, `c167961`).
- README updates now highlight the flexible grid model, column schemas, spreadsheet obscurity flags, login metadata associations, and secure note headers so client engineers understand how ciphertext is laid out (see `README.md`).

## Sync & API Surface
- Implemented stable serial write APIs for secure notes and spreadsheet data, plus JSON fetch endpoints for parity with the SDK (`e6a107f`, `dea2e9f`).
- Exposed vault discovery and fetch endpoints: `get_user_vault`, `get_all_user_vaults`, secure note/listing queries, and spreadsheet fetches for richer UI bootstrap flows (`8d834fa`, `bdf1a79`, `eb242ca`, `a835929`).
- Added a consolidated `global_sync` write path and matching tests so clients can push spreadsheets, columns, logins, and notes in one round trip (`b80f2fb`, `305b7d2`, `a549739`).
- Delivered maintenance and lifecycle hooks such as `maintain_status` for automated cycle top-ups and `purge_user` for data hygiene (`dc89c1e`, `674b87f`).
- Provided vetKD helpers, user registration flows, and inspect_message filtering to ensure only authorised principals can interact with shared vaults (`ecd74cf`, `534bb1a`, `a2924f2`, `fe9ffbd`).

## Reliability & Hardening
- Fixed wasm entropy initialisation by coercing `getrandom` to `raw_rand`, unblocking wasm32 builds (`4cc8d0d`).
- Resolved endianness handling in the byte protocol and numerous build regressions (pocket-ic, type mismatches, usize overflow) to keep the pipeline green (`8a19f1c`, `3aaa141`, `8ef7173`, `7fd3a9f`).
- Addressed secure note indexing, login metadata defaults, and vetKD key bounds to prevent user-visible bugs (`6ca6abe`, `b603297`, `68bb43d`, `a0ca56d`).
- Added regression tests for spreadsheet columns, global sync, shared-backend exports, and column info to catch breakages early (`13f9ecc`, `a549739`, `305b7d2`).

## Tooling & Operations
- Bootstrapped continuous integration, publishing optimised WASM and Candid artifacts for both shared and dedicated canisters (`bfc453f`, `c90d99a`, `38528ce`, `8ca29bb`).
- Updated workflows, build scripts, and documentation (including architecture diagrams) to streamline contributor onboarding (`8f2b1bc`, `3f9af11`, `959c575`).
- Added scripts and state initialisation fixes so shared vs premium vault deployments initialise reliably without panics (`1687100`, `1e4dede`, `1e26258`).

## Documentation & SDK Alignment
- `README.md` now documents the vault container layout—logins, secure notes, and the flexible grid with column secrecy flags—mirroring the backend types and emphasising the client-side encryption guarantee.
- `README sdk.md` details the serial protocol: big-endian headers, 5-byte segment lengths for `global_sync`, per-structure header formats for vault names, cells, login metadata, spreadsheet columns, and secure notes. These references ensure SDK implementations stay in lockstep with the backend deserialisers.
- The TypeScript SDK (`@ghostkeys/ghostkeys-sdk`) serialisers align with these docs; backend traps protect against malformed byte streams, so clients should rely on the SDK or replicate its schema exactly.

## Timeline Highlights (chronological)
- **14–20 Aug**: Initial commit, encryption challenge scaffolding, state module foundations, cycle optimisation utilities, GitHub Actions workflow, and wasm build unblocks (`489037c`, `7b2cfd3`, `bc1df80`, `fd780aa`, `42c40e3`, `4cc8d0d`, `bfc453f`).
- **21 Aug**: vetKD derivation, shared vault registration, key retrieval, inspect_message-based filtering, and vault name metadata (`ecd74cf`, `f8e42a9`, `534bb1a`, `a2924f2`, `a639904`).
- **22–24 Aug**: Capacity controls, user filtering fixes, workflow cleanups, architecture docs, new build outputs, and vetKD key derivation checks (`3e975cc`, `fe9ffbd`, `8ca29bb`, `8f2b1bc`, `3f9af11`, `959c575`, `771a7cc`).
- **4 Sep**: First deserialiser module pass with endianness corrections (`3ca1a0c`, `8a19f1c`).
- **10 Sep**: Stable storage migration, spreadsheet & login fetch APIs, documentation on data structures, shared backend test exports (`914584f`, `8d834fa`, `dea2e9f`, `2d2fe8b`, `7b99002`, `13f9ecc`).
- **11–14 Sep**: Secure notes serial write + JSON API, vault naming updates, stability fixes, and secure notes endpoints (`e6a107f`, `c167961`, `f27dea3`, `bdf1a79`).
- **15–18 Sep**: Memory optimisations, column metadata improvements, global sync introduction, get_all_user_vaults, dev APIs, and documentation refresh (`0301477`, `b07dc20`, `4654b67`, `b80f2fb`, `305b7d2`, `a549739`, `a835929`, `4a7af64`, `e0ae4cb`).
- **19–21 Sep**: Key bounds fixes, secure note bugfixes, purge endpoints, and login column cleanup (`a0ca56d`, `6ca6abe`, `674b87f`, `b603297`, `68bb43d`).
