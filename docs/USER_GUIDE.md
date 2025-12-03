# Ephemeral Vault - User Guide

## What is the Ephemeral Vault System?

The Ephemeral Vault System allows you to trade on GoQuant's dark pool perpetual futures DEX without signing every transaction. Here's how it works:

1. **You keep control** - Your main wallet remains in full control
2. **Temporary wallet handles trading** - A secure temporary wallet executes trades
3. **Fees handled automatically** - The system manages transaction fees for you
4. **Auto cleanup** - Unused funds return to you automatically

## Getting Started

### Step 1: Create a Session

Visit the GoQuant trading interface and click "Create Trading Session".

**What happens**:
- A temporary wallet is generated for your session
- Your main wallet is NOT involved yet
- Your session has an expiry time (default 1 hour)

**You receive**:
- Session ID (save this)
- Temporary wallet address
- Expiry time

### Step 2: Approve Delegation

Sign a delegation message with your main wallet.

**What you're signing**:
- Permission to delegate trading authority
- Session duration
- Maximum approved amount
- Your temporary wallet address

**Important**:
- This is a one-time signature
- You can revoke it anytime
- It only allows trading, not withdrawals

### Step 3: Deposit SOL for Fees

Transfer SOL from your main wallet to cover transaction fees.

**How much to deposit**:
- The system estimates fees for your trades
- We recommend depositing 1.5x the estimate
- Example: 10 trades × 5,000 lamports = 50,000 lamports + 25,000 buffer = 75,000 total

**The system will**:
- Track your balance
- Prevent overdrafts
- Alert if balance is running low
- Automatically suggest top-ups

### Step 4: Start Trading

Execute unlimited trades without signing! Your temporary wallet handles everything.

**Behind the scenes**:
- Each trade uses your temporary wallet
- Fees are deducted from your SOL balance
- All transactions are recorded
- Your main wallet maintains control

### Step 5: Session Ends

Two ways your session ends:

#### Manual Revocation (Recommended)
- Click "End Session"
- Sign revocation message (optional)
- All remaining SOL returns to your wallet immediately
- Session terminates

#### Automatic Expiry
- Session expires after 1 hour
- Anyone can trigger cleanup
- All remaining SOL returns to your wallet
- Account is cleaned up

## Session Management

### Check Your Balance

Click "View Session Status" to see:
- Total SOL deposited
- SOL spent on fees
- SOL remaining
- Session expiry time

### Add More SOL

If you're running low on funds:
1. Click "Deposit More"
2. Enter amount
3. Sign transaction from main wallet
4. Balance updates immediately

### End Session Early

To revoke your session:
1. Click "Revoke Session"
2. Verify in confirmation dialog
3. All unused SOL returns immediately
4. Session terminates

## Security Features

### Your Protection

**Control**: Your main wallet is ALWAYS in control
- Only you can approve delegation
- Only you can revoke access
- Only your main wallet can recover funds

**Time Limit**: Sessions automatically expire
- Default 1 hour
- No session lasts forever
- Automatic cleanup afterward

**Encryption**: Your temporary wallet is secure
- Private key is encrypted
- Only decrypted when needed
- Never stored unencrypted

### System Protection

**Fraud Detection**: We monitor for suspicious activity
- Unusual transaction sizes
- Rapid-fire transactions
- Unusual spending patterns
- Immediate alerts

**Rate Limiting**: Protection against abuse
- IP-based limits
- Device fingerprinting
- Automatic blocking of suspicious access

## Troubleshooting

### "Session Expired"
**Problem**: Session is no longer valid

**Solution**:
- Create a new session
- Sessions last 1 hour
- You can create multiple sessions

### "Insufficient Funds"
**Problem**: Not enough SOL in vault

**Solution**:
- Click "Deposit More"
- Add additional SOL
- System shows recommended amount

### "Delegation Failed"
**Problem**: Approval message wasn't signed correctly

**Solution**:
- Return to step 2 and sign again
- Ensure you're using the correct wallet
- Check your wallet's signing capability

### "Anomalous Activity Detected"
**Problem**: System detected unusual spending

**Solution**:
- This is a security feature
- Contact GoQuant support with details
- Sessions may be temporarily locked for review

## Tips & Best Practices

### For Best Performance

1. **Deposit enough upfront**
   - Estimate your fee costs
   - Add 50% buffer for safety
   - Reduces API calls

2. **Monitor your balance**
   - Check status regularly
   - Don't let it run out
   - Allows uninterrupted trading

3. **End sessions when done**
   - Don't leave sessions idle
   - Recover funds promptly
   - Reduces security exposure

### For Maximum Security

1. **Use trusted networks**
   - Avoid public WiFi
   - Use VPN if possible
   - Check connection security

2. **Verify session details**
   - Check expiry time
   - Confirm wallet address
   - Note session ID

3. **Set session duration carefully**
   - 1 hour for quick trades
   - Extend for longer sessions
   - Revoke if not needed

4. **Never share session details**
   - Session ID is sensitive
   - Temporary wallet is public
   - Main wallet stays private

## FAQ

### Q: Can someone steal my funds with just the temporary wallet?
**A**: No. The temporary wallet has limited permissions - trading only. Funds can't be withdrawn without the parent wallet's permission.

### Q: What if I lose my session ID?
**A**: You can generate a new session. Your old session will auto-expire after 1 hour anyway.

### Q: Can I have multiple sessions?
**A**: Yes, but we recommend one at a time for better management. Each session is independent.

### Q: What happens if I never revoke my session?
**A**: It automatically expires after 1 hour, and funds are returned. You don't need to do anything.

### Q: Can I extend my session?
**A**: No, sessions have fixed duration. Create a new session when the current one expires.

### Q: How are fees calculated?
**A**: Each transaction has a base fee (5,000 lamports) plus size-based fees. The system estimates before you trade.

### Q: Is my temporary wallet saved?
**A**: No, it's generated fresh each session and encrypted. Previous temporary wallets are discarded.

### Q: Can the platform access my main wallet?
**A**: No. The platform only sees your signature on the delegation message. Your main wallet remains entirely under your control.

### Q: What if I disconnect during a trade?
**A**: Your session remains active. You can reconnect and continue trading. The temporary wallet doesn't require your presence.

## Contact Support

For issues or questions:
- Email: support@goquant.io
- Discord: [GoQuant Community](https://discord.gg/goquant)
- Docs: [Full Documentation](https://docs.goquant.io)

## Glossary

**Parent Wallet**: Your main wallet that maintains custody
**Ephemeral Wallet**: Temporary wallet created for your session
**Delegation**: Permission given to temporary wallet for trading
**Vault**: On-chain account holding your session funds
**Session**: One trading period (typically 1 hour)
**PDA**: Program Derived Address (deterministic on-chain account)

---

**Remember**: Your main wallet is always in control. Stay secure!
