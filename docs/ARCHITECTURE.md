# Ephemeral Vault System - Architecture & Security Guide

## System Overview

The Ephemeral Vault System is a sophisticated solution for enabling gasless trading on a dark pool perpetual futures DEX. It bridges user experience with security by:

1. **Delegating Trading Authority**: Users maintain custody while delegating limited authority to temporary wallets
2. **Automatic Fee Management**: System manages SOL deposits for transaction fees automatically
3. **Session Management**: Time-limited trading sessions that auto-expire and recover funds
4. **High Performance**: Supports 1000+ concurrent sessions with minimal latency

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        USER APPLICATION                          │
└──────────────┬──────────────────────────────────────────────────┘
               │
        ┌──────▼──────┐
        │ Parent Wallet│  (Maintains custody)
        │             │
        └──────┬──────┘
               │
        ┌──────▼─────────────────────────────────────────────┐
        │         EPHEMERAL VAULT SYSTEM (Backend)           │
        │                                                    │
        │  ┌────────────────────────────────────────────┐  │
        │  │  Session Manager                           │  │
        │  │  - Generate ephemeral keypairs            │  │
        │  │  - Manage session lifecycle               │  │
        │  │  - Encrypt/decrypt keys (AES-256-GCM)    │  │
        │  └────────────────────────────────────────────┘  │
        │                     │                              │
        │  ┌──────────────────┼──────────────────────────┐  │
        │  │                  │                          │  │
        │  ▼                  ▼                          ▼  │
        │ ┌──────────┐  ┌──────────┐  ┌──────────┐         │
        │ │Auto-     │  │Delegation│  │Vault     │         │
        │ │Deposit   │  │Manager   │  │Monitor   │         │
        │ │Calculator│  │          │  │          │         │
        │ └──────────┘  └──────────┘  └──────────┘         │
        │                                                    │
        │  ┌────────────────────────────────────────────┐  │
        │  │  Security Layer                            │  │
        │  │  - Rate limiting (IP-based)               │  │
        │  │  - Anomaly detection (spending patterns)  │  │
        │  │  - Device fingerprinting                  │  │
        │  │  - Signature verification                 │  │
        │  └────────────────────────────────────────────┘  │
        │                                                    │
        │  ┌────────────────────────────────────────────┐  │
        │  │  REST API / WebSocket Server              │  │
        │  └────────────────────────────────────────────┘  │
        └──────────────┬──────────────────────────────────┘
                       │
        ┌──────────────┼───────────────────┬──────────────┐
        │              │                   │              │
        ▼              ▼                   ▼              ▼
    ┌────────┐   ┌──────────┐      ┌──────────────┐  ┌───────────┐
    │PostgreSQL│  │Ephemeral │      │Solana        │  │Vault Smart│
    │Database │  │Wallet(s) │      │Blockchain    │  │Contract   │
    └────────┘   └──────────┘      └──────────────┘  └───────────┘
```

## Session Lifecycle

```
1. USER INITIATES
   └─> Connects parent wallet

2. SESSION CREATION
   └─> Backend generates ephemeral wallet
   └─> Creates session with encrypted keypair
   └─> Returns ephemeral wallet address & session ID

3. DELEGATION APPROVAL
   └─> Parent wallet signs delegation tx
   └─> Smart contract records delegation
   └─> Sets session expiry timestamp

4. AUTO-DEPOSIT
   └─> Parent deposits SOL for transaction fees
   └─> Backend tracks balance
   └─> Fee estimator calculates optimal amount

5. TRADING PHASE
   └─> User can execute unlimited trades
   └─> Uses ephemeral wallet (no signing needed)
   └─> Fees deducted from vault balance
   └─> All transactions tracked in database

6. SESSION MANAGEMENT
   └─> Monitor: Balance, activity, expiry
   └─> Anomaly detection: Spending patterns
   └─> Auto-topup: Insufficient balance check
   └─> Activity monitoring: Inactivity detection

7. SESSION TERMINATION
   ├─> Manual revocation (parent-initiated)
   │   └─> Immediate delegation revocation
   │   └─> Return remaining balance
   └─> Automatic expiry
       └─> Cleanup triggered (anyone can call)
       └─> Return all remaining funds
       └─> Close vault account

8. CLEANUP & RECOVERY
   └─> Return unused SOL to parent wallet
   └─> Record all transactions in database
   └─> Generate analytics
   └─> Free up on-chain space (reclaim rent)
```

## Fund Flow

### Deposit Flow
```
Parent Wallet Balance
        │
        ├──> SOL Transfer (auto_deposit_for_trade)
        │
        ▼
    Vault PDA
        │
        ├──> Available Balance
        ├──> Used Balance (trades)
        └──> Tracking in database
```

### Trade Flow
```
Vault PDA Balance
        │
        ├──> Deduct Trade Amount
        ├──> Deduct Fee Amount
        │
        ▼
Available Balance Updates
        │
        └──> Logged in vault_transactions table
```

### Cleanup Flow
```
Vault PDA Balance (Expired or Revoked)
        │
        ├──> Calculate Remaining Amount
        │
        ▼
    Return to Parent Wallet
        │
        ├──> Update session status
        ├──> Record cleanup event
        └──> Close vault account
```

## Component Details

### 1. Smart Contract (Anchor Program)

**Key Features**:
- PDA-based vault accounts for isolation
- Parent wallet control and revocation
- Atomic operations prevent race conditions
- Event emission for off-chain tracking
- Overflow protection in all arithmetic

**Security**:
- Only parent wallet can approve delegation
- Only approved delegate can trade
- Session expiry enforcement
- Fund isolation per user

### 2. Session Manager

**Responsibilities**:
- Generate cryptographically secure ephemeral keypairs
- Encrypt/decrypt keypairs using AES-256-GCM
- Manage session state lifecycle
- Validate session integrity on each request

**Encryption Process**:
```
Master Secret (from .env)
    │
    ├─> SHA-256 Hash
    │
    ▼
256-bit Key
    │
    ├─> (combined with random nonce)
    │
    ▼
AES-256-GCM Encryption
    │
    ├─> Ciphertext + Nonce (Base64 encoded)
    │
    ▼
Database Storage
```

### 3. Auto-Deposit Calculator

**Algorithm**:
```
Estimated Trade Fee
    │
    ├─> Apply 1.5x Priority Multiplier
    │
    ▼
Required Deposit = Fee × 1.5
    │
    ├─> Add Buffer for Safety
    │
    ▼
Optimal Amount = Required × Safety Factor
```

**Topup Decision Tree**:
```
Current Balance
    │
    ├─> Pending Operations Count
    │
    ▼
Should Topup? = CurrentBalance < (BaseFee × PendingOps × 2)
    │
    ├─> YES: Calculate optimal topup
    └─> NO: Continue trading
```

### 4. Security Layer

**Rate Limiting**:
- Token bucket algorithm
- Per-IP tracking
- 1-minute reset window
- Default: 100 requests/minute

**Anomaly Detection**:
- Exponential moving average (α = 0.3)
- Transaction size baseline
- Configurable multiplier threshold (2.5x)
- Automatic alert generation

**Device Fingerprinting**:
```
Device Fingerprint = SHA256(User-Agent + IP)
    │
    ├─> Stored in session
    ├─> Validated on each request
    └─> Mismatch = Session rejection
```

## Security Guarantees

### 1. Fund Safety
✓ No orphaned balances - automatic cleanup
✓ Time-limited delegation - sessions auto-expire
✓ Parent wallet control - can revoke anytime
✓ Atomic operations - no partial states

### 2. Key Security
✓ Encrypted at rest (AES-256-GCM)
✓ Separate from parent wallet
✓ Limited scope (trading only)
✓ Session-bound (time-limited)

### 3. Access Control
✓ IP-based session binding
✓ Device fingerprinting
✓ Delegation verification
✓ Parent signature requirement

### 4. Attack Prevention
✓ Rate limiting - brute force protection
✓ Anomaly detection - unauthorized access detection
✓ Overflow protection - arithmetic safeguards
✓ Replay protection - nonce-based encryption

## Performance Specifications

### Latency Targets
- Session creation: < 500ms
- Transaction signing: < 50ms
- API response time: < 100ms
- Database query: < 10ms

### Throughput Targets
- Concurrent sessions: 1000+
- Transactions per second: 100+
- Connections per minute: 1000+

### Optimization Strategies
1. **Connection pooling** for database
2. **Async I/O** throughout backend
3. **Batch cleanup** operations
4. **In-memory caching** for hot sessions
5. **Efficient PDA derivation** on-chain

## Deployment Considerations

### On-Chain (Solana)
- Deploy to devnet first for testing
- Verify against mainnet-beta
- Set appropriate account sizes
- Monitor compute unit usage

### Off-Chain (Backend)
- Use environment variables for secrets
- Deploy behind reverse proxy (nginx)
- Enable HTTPS/TLS
- Set up database replication for HA

### Database
- Create proper indexes
- Set up automated backups
- Enable replication for failover
- Monitor query performance

## Monitoring & Alerting

### Key Metrics
- Active session count
- Failed transaction rate
- Average session duration
- Fund utilization rate
- Security event rate

### Alerts
- Rate limit exceeded (per IP)
- Anomalous spending detected
- Session expiry approaching
- Vault balance critically low
- Unauthorized access attempt

## Compliance & Best Practices

### Regulatory
- Session audit trail maintained
- Transaction history preserved
- User identification (wallet address)
- Compliance event logging

### Operational
- Regular security audits
- Penetration testing
- Key rotation procedures
- Disaster recovery drills

---

## Conclusion

The Ephemeral Vault System provides a secure, efficient solution for gasless trading while maintaining strong custody controls. Its multi-layered security approach, combined with automated fund management, creates a robust platform for high-frequency trading scenarios.

**Key Advantages**:
- 0 ETH/SOL gas for users (delegated)
- Instant trading (no signing delays)
- Secure key management
- Automatic fund recovery
- Full auditability

