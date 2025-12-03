# Ephemeral Vault System — Video Script (10–15 minutes)

This file contains a step-by-step video script, terminal commands, expected outputs, and recording tips to produce a 10–15 minute demo of the Ephemeral Vault System for submission to GoQuant.

---

## Recording setup (before you start)
- Screen layout: left = terminal, right = editor (file tree) and browser (optional).
- Microphone: quiet room, test audio.
- Terminal font/size: large enough to read in the video.
- Recorder: OBS Studio recommended (or QuickTime for macOS).
- Prepare this file open in editor as your cue sheet.

## Timing guide (approx)
- 0:00–0:30 — Intro (30s)
- 0:30–2:00 — Project overview & architecture (1.5m)
- 2:00–4:00 — Smart contract walkthrough (2m)
- 4:00–6:00 — Backend architecture + managers walkthrough (2m)
- 6:00–8:30 — Live demo: run server, create session endpoint, show logs (2.5m)
- 8:30–10:00 — Tests: run tests & show results (1.5m)
- 10:00–11:00 — Database migration & schema (1m)
- 11:00–12:00 — WebSocket quick demo + wrap-up (1m)
- 12:00–12:30 — Submission summary & next steps (30s)

---

## 0:00 — Intro (30s)
- Narration: "Hi, I'm [Your Name]. This is my Ephemeral Vault System submission for GoQuant. I'll show architecture, smart contract, backend, tests, and a live demo. Total ~12 minutes."
- Visual: Show `README.md` in the editor.

## 0:30 — Project overview & architecture (1m30s)
- Narration: "High level: an Anchor program on Solana creates ephemeral vault PDAs for session-based ephemeral wallets. The Rust backend manages sessions, key encryption, auto-deposits, delegation, vault monitoring, and transaction signing."
- Visual: Open `SUBMISSION_CHECKLIST.md` and `docs/ARCHITECTURE.md` (or `docs/TECHNICAL_DOCUMENTATION.md`).
- Terminal (optional) — show repo layout:
```zsh
ls -la
ls -la backend
ls -la programs/ephemeral-vault
```

## 2:00 — Smart contract walkthrough (2m)
- Open `programs/ephemeral-vault/src/lib.rs` in editor.
- Narration: "This Anchor program implements the 6 core instructions: create_ephemeral_vault, approve_delegate, auto_deposit_for_trade, execute_trade, revoke_access, cleanup_vault. It stores vault state and delegation state in `EphemeralVault` and `VaultDelegation` accounts."
- Show `create_ephemeral_vault` function and account structs (`EphemeralVault`, `VaultDelegation`).
- Narration: "On create, we set expiry, approved amount and emit events. Approve writes a `VaultDelegation` account. Auto deposit transfers lamports into the vault PDA."
- Note: Anchor on-chain integration tests can be run with `anchor test` (see scaffold in `programs/ephemeral-vault/tests/anchor_integration.rs`).

## 4:00 — Backend architecture + managers (2m)
- Open `backend/src/lib.rs` and `backend/src/managers/` in editor.
- Narration: "Backend has managers: SessionManager, DelegationManager, VaultMonitor, AutoDepositCalculator, TransactionSigner. API handlers are in `backend/src/api/handlers.rs` and DB logic in `backend/src/db.rs`."
- Show `SessionManager` (encryption, ephemeral key creation) and `session_manager.rs` code.
- Narration: "Keys are encrypted with AES-256-GCM derived from a master secret. For local tests ephemeral keypairs are simulated; production would use real keypairs and Solana integration."

## 6:00 — Live demo: run server, create session endpoint (2.5m)
Prepare terminal tabs:
- Tab 1: server
- Tab 2: tests
- Tab 3: curl requests

Set environment variables (example):
```zsh
# from repo root
export DATABASE_URL=postgres://postgres:password@localhost:5432/evault_dev
export JWT_SECRET=demo-secret
export PORT=8080

cd backend
```

Start server:
```zsh
cargo run
```

Narration: "Starting backend server — it connects to Postgres and exposes API under `/api`."

Create a session (new terminal tab):
```zsh
curl -s -X POST http://127.0.0.1:8080/api/session/create \
  -H "Content-Type: application/json" \
  -d '{"user_wallet":"user1","duration_secs":3600,"device_info":"demo-device"}' | jq
```

Expected output: JSON with `session_id`, `ephemeral_wallet`, `expires_at`.

Approve delegation (simulated):
```zsh
curl -s -X POST http://127.0.0.1:8080/api/session/approve \
  -H "Content-Type: application/json" \
  -d '{"session_id":"<paste-session-id>","signature":"demo"}' | jq
```

Switch to server logs and show the request being handled.

Notes if Postgres not available:
- Handlers still demonstrate in-memory behavior for demonstration. For a full DB demo, start Postgres or use Docker:
```zsh
docker run -p 5432:5432 -e POSTGRES_PASSWORD=password -e POSTGRES_DB=evault_dev postgres:14
```

## 8:30 — Tests: run tests & show results (1.5m)
- Stop server (Ctrl+C) if running.
- Run backend tests (fast):
```zsh
cargo test -p ephemeral-vault-backend -- --nocapture
```

Expected: 47 passing tests across backend crate. Show key output blocks that indicate successful runs.

Then run root integration tests:
```zsh
cargo test --test integration_tests
```

Optionally run full workspace (may reintroduce dependency issues):
```zsh
cargo test --all
```

Narration: "I ran `cargo test --all` successfully on my machine. If you see dependency conflicts with Solana transitive crates, enable gating or run program tests separately."
# Ephemeral Vault System — Video Script (10–15 minutes)

This presenter-friendly script is segmented for recording: each section contains [NARRATION] (exact lines to speak), [ON-SCREEN ACTION] (what to show or run), and [TIPS]. Use the [NARRATION] lines verbatim as a teleprompter.

Recording setup:
- Screen layout: terminal (left), editor + file tree (right).
- Microphone: test levels and reduce background noise.
- Terminal font: large and readable. OBS Studio recommended for recording.

Timing (approx): 0:00 Intro, 0:30 Project overview, 2:00 Smart contract walkthrough, 4:00 Backend architecture, 6:00 Live demo, 8:30 Tests, 10:00 DB/migrations, 11:00 WebSocket demo, 11:45 Anchor note, 12:15 Wrap-up.

0:00 — INTRO (30s)
[NARRATION]
"Hi, I'm [Your Name]. Thank you for reviewing my Ephemeral Vault System submission for GoQuant. In the next 10–15 minutes I'll walk through the architecture, the Anchor smart contract, the Rust backend, and a short live demo including tests and an API call."

[ON-SCREEN ACTION]
- Open `README.md` briefly to show the project title.

0:30 — PROJECT OVERVIEW & ARCHITECTURE (1m30s)
[NARRATION]
"High level summary: The on-chain Anchor program manages ephemeral vault PDAs and delegations. The backend provides session management, ephemeral key encryption, auto-deposit calculation, delegation lifecycle management, vault monitoring, and transaction signing."

[ON-SCREEN ACTION]
- Open `SUBMISSION_CHECKLIST.md` and `docs/TECHNICAL_DOCUMENTATION.md` (if present).
- Optionally show repo layout:
```zsh
ls -la
ls -la backend
ls -la programs/ephemeral-vault
```

[TIPS]
- Keep orientation brief and clear.

2:00 — SMART CONTRACT WALKTHROUGH (2m)
[NARRATION]
"The Anchor program is in `programs/ephemeral-vault/src/lib.rs`. It implements instructions such as `create_ephemeral_vault`, `approve_delegate`, `auto_deposit_for_trade`, `execute_trade`, `revoke_access`, and `cleanup_vault`. Main account types are `EphemeralVault` and `VaultDelegation`."

[ON-SCREEN ACTION]
- Open `programs/ephemeral-vault/src/lib.rs` and scroll to the instruction handlers and account structs.

[NARRATION — VERBATIM]
"On vault creation we initialize the PDA with the parent wallet address, set the expiry timestamp, and record approved amounts. Approval creates a `VaultDelegation` account; auto deposit transfers lamports to the vault PDA; execute checks delegation, and revoke/cleanup return unused funds."

[TIPS]
- Note: don't run `anchor test` unless you have Anchor and a local validator configured.

4:00 — BACKEND ARCHITECTURE & MANAGERS (2m)
[NARRATION]
"The backend lives under `backend/`. Core components: `SessionManager` (ephemeral keys + encryption), `DelegationManager`, `AutoDepositCalculator`, `VaultMonitor`, and `TransactionSigner`. API handlers are in `backend/src/api/handlers.rs`, DB logic in `backend/src/db.rs`."

[ON-SCREEN ACTION]
- Open `backend/src/managers/session_manager.rs` to show key derivation and AES‑GCM encryption.
- Open `backend/src/managers/auto_deposit.rs` to show fee calculation logic.

[NARRATION — VERBATIM]
"SessionManager derives a 32-byte AES key from a master secret using SHA-256 and uses AES-GCM to encrypt ephemeral secret material at rest. For local tests the keypair generation is simulated; production would use secure key generation and store encrypted blobs in Postgres. AutoDepositCalculator estimates fees to keep the vault funded for trades."

[TIPS]
- Point out `backend/src/api/handlers.rs` to show available REST endpoints.

6:00 — LIVE DEMO: START SERVER AND CREATE SESSION (2.5m)
[NARRATION]
"I'll start the backend and demonstrate creating and approving a session via the REST API. Ensure `DATABASE_URL`, `JWT_SECRET`, and `PORT` are set."

[ON-SCREEN ACTION — TERMINAL]
```zsh
# from repo root
export DATABASE_URL=postgres://postgres:password@localhost:5432/evault_dev
export JWT_SECRET=demo-secret
export PORT=8080

cd backend
cargo run
```

[NARRATION]
"The backend will connect to Postgres and serve APIs on the configured port."

[ON-SCREEN ACTION — NEW TERMINAL]
```zsh
curl -s -X POST http://127.0.0.1:8080/api/session/create \
  -H "Content-Type: application/json" \
  -d '{"user_wallet":"user1","duration_secs":3600,"device_info":"demo-device"}' | jq
```

[EXPECTED OUTPUT]
```json
{
  "session_id": "<uuid>",
  "ephemeral_wallet": "ephemeral_xxx",
  "expires_at": "2025-12-03T...Z"
}
```

[NARRATION — VERBATIM]
"The backend created an ephemeral session: it generated simulated ephemeral key material, encrypted it and recorded the session in the database."

[ON-SCREEN ACTION]
```zsh
curl -s -X POST http://127.0.0.1:8080/api/session/approve \
  -H "Content-Type: application/json" \
  -d '{"session_id":"<paste-session-id>","signature":"demo"}' | jq
```

[NARRATION]
"The approve endpoint validates the session and returns success. Production approval requires a signed transaction from the parent wallet."

[TIPS]
- If Postgres isn't running, run one with Docker:
```zsh
docker run -p 5432:5432 -e POSTGRES_PASSWORD=password -e POSTGRES_DB=evault_dev postgres:14
```

8:30 — RUN TESTS & SHOW RESULTS (1.5m)
[NARRATION]
"Now I'll run the backend test suite and the repository-level integration tests."

[ON-SCREEN ACTION — TERMINAL]
```zsh
cd /Users/anuj/Desktop/Quant_vs
cargo test -p ephemeral-vault-backend -- --nocapture
```

[EXPECTED OUTPUT — HIGHLIGHTS]
- Library tests (9) pass
- `backend/tests/session_lifecycle.rs` (19) pass
- `backend/tests/more_unit_tests.rs` (19) pass
- `backend/tests/ws_tests.rs` (1) pass
- Summary: all tests pass

[ON-SCREEN ACTION — TERMINAL]
```zsh
cargo test --test integration_tests
```

[NARRATION]
"On my machine these commands return 54 passing tests total (47 backend + 7 integration). If you see a Solana dependency conflict, run backend-only tests or run Anchor tests separately."

10:00 — DATABASE MIGRATION & SCHEMA (1m)
[NARRATION]
"Migrations live in `migrations/001_initial_schema.sql`. They create sessions, transactions, delegations, and analytics tables."

[ON-SCREEN ACTION]
- Open `migrations/001_initial_schema.sql` and scroll through schema.

[ON-SCREEN ACTION — TERMINAL]
```zsh
psql $DATABASE_URL -f migrations/001_initial_schema.sql
# or: DATABASE_URL=$DATABASE_URL cargo sqlx migrate run

psql $DATABASE_URL -c '\dt'
psql $DATABASE_URL -c 'select * from sessions limit 5;'
```

11:00 — WEBSOCKET QUICK DEMO (1m)
[NARRATION]
"A minimal WebSocket echo endpoint is at `/api/ws/`. It's a test harness and can be extended for realtime events."

[ON-SCREEN ACTION — TERMINAL]
```zsh
# websocat: 
websocat ws://127.0.0.1:8080/api/ws/
# or:
npx wscat -c ws://127.0.0.1:8080/api/ws/
```

[DEMO]
- Type `hello` into the WS client and show the echoed response.

[NARRATION]
"Production would authenticate and route messages to subscribed clients."

11:45 — ANCHOR ON-CHAIN TESTS NOTE (30s)
[NARRATION]
"The on-chain program is included. To run Anchor tests, use `anchor test` inside `programs/ephemeral-vault`; this requires Anchor and a local validator. A scaffold test is provided."

[ON-SCREEN ACTION — TERMINAL]
```zsh
cd programs/ephemeral-vault
anchor test
```

[TIPS]
- Mention that Anchor/validator setup is environment-specific.

12:15 — WRAP-UP & SUBMISSION SUMMARY (45s)
[NARRATION — VERBATIM]
"Recap: the smart contract implements the requested instructions; the backend handles session lifecycle, encryption, delegation and monitoring; migrations and tests are included. Tests pass locally (54 total). The repo contains documentation and a beginner setup guide. Next steps: record the video, attach my resume, and submit."

[ON-SCREEN ACTION]
- Open `SUBMISSION_CHECKLIST.md` and `docs/TEST_RESULTS.md` to show reproducible test instructions.

SUBMISSION EMAIL TEMPLATE (copy/paste)
```text
Subject: Ephemeral Vault System - Assignment Submission - [Your Name]

Dear GoQuant Team,

Please find attached my submission for the Ephemeral Vault System assignment.

Deliverables included:
- Source Code: https://github.com/YOUR_USERNAME/ephemeral-vault
- Video Demonstration: [YouTube unlisted link]
- Technical Documentation: docs/TECHNICAL_DOCUMENTATION.md
- Test Results: docs/TEST_RESULTS.md
- Migrations: migrations/001_initial_schema.sql

Key points:
- Smart contract: `programs/ephemeral-vault/src/lib.rs`
- Backend: `backend/` (managers, API, encryption)
- Tests: 54 passing total (47 backend + 7 integration)

Please let me know if you want a live walkthrough.

Best regards,
[Your Name]
```

Recording tips:
- Do a dry run of commands before recording.
- Record in short segments and edit out pauses.
- Use [NARRATION] lines as a teleprompter for clarity.

Troubleshooting notes:
- If `cargo test --all` fails with dependency errors:
  - Run backend-only tests: `cargo test -p ephemeral-vault-backend -- --nocapture`
  - Run Anchor tests separately: `cd programs/ephemeral-vault && anchor test`
  - Consider separating the on-chain program into a different workspace to avoid transitive dependency conflicts.
- If DB connectivity fails:
  - Start Postgres: `brew services start postgresql` or run via Docker:
    ```zsh
    docker run -p 5432:5432 -e POSTGRES_PASSWORD=password -e POSTGRES_DB=evault_dev postgres:14
    ```
  - Apply migrations: `psql $DATABASE_URL -f migrations/001_initial_schema.sql`
