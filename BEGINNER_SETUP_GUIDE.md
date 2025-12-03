# 🚀 Complete Beginner's Guide to Setting Up Ephemeral Vault

## **PART 1: Understanding What You Need**

Think of this system like a restaurant:
- **Smart Contract** = The recipe and kitchen rules
- **Backend Service** = The waiter taking orders
- **Database** = The ledger recording all transactions
- **Tests** = Quality checks to make sure everything works

To run everything, you need **3 main tools** installed on your computer:

### **1. Rust** (The Programming Language)
### **2. PostgreSQL** (The Database - where we store data)
### **3. Solana CLI** (Tools for blockchain interaction)

---

## **PART 2: Installation (Step by Step)**

### **Step 1: Install Rust**

**What is Rust?** It's a programming language. Think of it like installing a language translator on your computer.

**Mac/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**What to do:**
1. Open Terminal (Mac: Cmd+Space, type "Terminal", press Enter)
2. Copy the command above and paste it
3. Press Enter
4. When asked, press Enter again (accept default installation)
5. Wait for it to finish

**To verify it worked:**
```bash
rustc --version
```
You should see something like: `rustc 1.75.0`

---

### **Step 2: Install PostgreSQL Database**

**What is PostgreSQL?** It's a database (like a digital filing cabinet) where all data is stored.

**Mac (using Homebrew):**
```bash
# First, install Homebrew if you don't have it
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Then install PostgreSQL
brew install postgresql
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
```

**Start PostgreSQL:**

**Mac:**
```bash
brew services start postgresql
```

**Linux:**
```bash
sudo systemctl start postgresql
```

**To verify it works:**
```bash
psql --version
```
You should see: `psql (PostgreSQL) 13.x`

---

### **Step 3: Install Solana CLI**

**What is Solana CLI?** Tools to interact with the Solana blockchain.

```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
```

**To verify:**
```bash
solana --version
```
You should see a version number.

---

## **PART 3: Set Up the Database**

### **Create the Database**

Open Terminal and run:

```bash
# Open PostgreSQL
psql -U postgres

# Inside PostgreSQL, create the database
CREATE DATABASE ephemeral_vault OWNER postgres;

# Exit PostgreSQL
\q
```

**What does this do?**
- Creates a new database called "ephemeral_vault"
- Like creating a new filing cabinet in your office

---

### **Create Database Tables**

Now we need to create the "folders" (tables) inside the database.

```bash
# Navigate to your project folder
cd /Users/anuj/Desktop/Quant_vs

# Create the tables
psql -d ephemeral_vault < migrations/001_initial_schema.sql
```

**To verify it worked:**
```bash
# Connect to the database
psql -d ephemeral_vault

# See all tables
\dt

# You should see:
# - sessions
# - vault_transactions
# - delegations
# - cleanup_events
# - security_events
```

Type `\q` to exit.

---

## **PART 4: Fill Your .env File (THE SECRETS FILE)**

### **What is .env?**
It's a configuration file with settings and secrets (like passwords). Think of it as a configuration memo for the system.

### **Step 1: Copy the Template**

```bash
# Navigate to your project
cd /Users/anuj/Desktop/Quant_vs

# Copy the example file
cp .env.example .env
```

### **Step 2: Edit the .env File**

```bash
# Open the file in a text editor
nano .env
```

Or use VS Code:
```bash
code .env
```

---

## **PART 5: Fill In Each Value**

Here's what each setting means and where to get the value:

### **📍 Database Configuration**

```env
DATABASE_URL=postgresql://postgres:password@localhost:5432/ephemeral_vault
```

**What it means:** Connection details to your database
- `postgres` = username (default)
- `password` = PostgreSQL password (you created this)
- `localhost:5432` = database location (local on your computer)
- `ephemeral_vault` = database name

**If you used the default PostgreSQL installation, just use:**
```env
DATABASE_URL=postgresql://postgres@localhost:5432/ephemeral_vault
```

**Status:** ✅ REQUIRED - without this, nothing works

---

### **🔌 Solana Configuration**

#### Option A: Using Local Development (RECOMMENDED FOR TESTING)

```env
SOLANA_RPC_URL=http://localhost:8899
SOLANA_WS_URL=ws://localhost:8900
VAULT_PROGRAM_ID=11111111111111111111111111111111
```

**What it means:**
- `SOLANA_RPC_URL` = Where to connect to blockchain
- `SOLANA_WS_URL` = WebSocket connection (for real-time updates)
- `VAULT_PROGRAM_ID` = Your smart contract address

**For development/testing:** Use `localhost` (your own computer)

**Status:** ✅ REQUIRED

---

#### Option B: Using Devnet (Solana Test Network)

```env
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_WS_URL=wss://api.devnet.solana.com
VAULT_PROGRAM_ID=11111111111111111111111111111111
```

**Note:** You'd need to deploy your smart contract first (more advanced)

---

### **🔐 Security Settings**

```env
JWT_SECRET=your-super-secret-key-change-this-in-production
```

**What it means:** A secret key to encrypt sensitive data

**How to create one (copy-paste this command):**
```bash
openssl rand -base64 32
```

**It will give you something like:**
```
xK7mP9vL2qR8sT3xY6zQ1wE5rT9uI2oP5sA7dF4gH1jK
```

**Copy that and replace the value:**
```env
JWT_SECRET=xK7mP9vL2qR8sT3xY6zQ1wE5rT9uI2oP5sA7dF4gH1jK
```

**Status:** ✅ REQUIRED

---

### **⚙️ Service Configuration**

```env
PORT=8080
MAX_CONCURRENT_SESSIONS=1000
SESSION_DURATION_SECS=3600
INACTIVITY_TIMEOUT_SECS=1800
```

**What each means:**
- `PORT=8080` = Where the server runs (like address in a building)
- `MAX_CONCURRENT_SESSIONS=1000` = Max users at once (1000)
- `SESSION_DURATION_SECS=3600` = How long a session lasts (3600 seconds = 1 hour)
- `INACTIVITY_TIMEOUT_SECS=1800` = Timeout after 30 minutes of no activity

**What to do:** KEEP THESE DEFAULT VALUES ✅ These are safe defaults

**Status:** ⚙️ OPTIONAL (good defaults provided)

---

### **📝 Logging**

```env
RUST_LOG=info
LOG_FILE=./logs/app.log
```

**What it means:**
- `RUST_LOG=info` = Show important messages (not debug details)
- `LOG_FILE=./logs/app.log` = Where to save log files

**What to do:** KEEP THESE DEFAULT VALUES ✅

**Status:** ⚙️ OPTIONAL

---

### **🚀 Feature Flags**

```env
ENABLE_RATE_LIMITING=true
ENABLE_ANOMALY_DETECTION=true
ENABLE_DEVICE_FINGERPRINTING=true
```

**What these do:**
- Rate limiting = Prevent spam attacks
- Anomaly detection = Detect unusual activity
- Device fingerprinting = Identify devices

**What to do:** KEEP THESE AS `true` ✅ (security features)

**Status:** ⚙️ OPTIONAL

---

### **📊 Thresholds**

```env
RATE_LIMIT_PER_MINUTE=100
ANOMALY_DETECTION_MULTIPLIER=2.5
MIN_DEPOSIT_AMOUNT=5000
MAX_DEPOSIT_AMOUNT=10000000000
```

**What they mean:**
- `RATE_LIMIT_PER_MINUTE=100` = Max 100 requests per minute
- `ANOMALY_DETECTION_MULTIPLIER=2.5` = Alert if spending is 2.5x normal
- `MIN_DEPOSIT_AMOUNT=5000` = Minimum amount to deposit
- `MAX_DEPOSIT_AMOUNT=10000000000` = Maximum amount to deposit

**What to do:** KEEP THESE DEFAULT VALUES ✅

**Status:** ⚙️ OPTIONAL

---

## **FINAL .env FILE (READY TO COPY)**

Here's a complete, ready-to-use .env file:

```env
# Database Configuration
DATABASE_URL=postgresql://postgres@localhost:5432/ephemeral_vault

# Solana Configuration
SOLANA_RPC_URL=http://localhost:8899
SOLANA_WS_URL=ws://localhost:8900
VAULT_PROGRAM_ID=11111111111111111111111111111111

# Security (CHANGE THIS - run: openssl rand -base64 32)
JWT_SECRET=xK7mP9vL2qR8sT3xY6zQ1wE5rT9uI2oP5sA7dF4gH1jK

# Service Configuration
PORT=8080
MAX_CONCURRENT_SESSIONS=1000
SESSION_DURATION_SECS=3600
INACTIVITY_TIMEOUT_SECS=1800

# Logging
RUST_LOG=info
LOG_FILE=./logs/app.log

# Feature Flags
ENABLE_RATE_LIMITING=true
ENABLE_ANOMALY_DETECTION=true
ENABLE_DEVICE_FINGERPRINTING=true

# Thresholds
RATE_LIMIT_PER_MINUTE=100
ANOMALY_DETECTION_MULTIPLIER=2.5
MIN_DEPOSIT_AMOUNT=5000
MAX_DEPOSIT_AMOUNT=10000000000
```

---

## **PART 6: Create .env File**

**Using Terminal:**

```bash
# Go to your project folder
cd /Users/anuj/Desktop/Quant_vs

# Create the file with all settings
cat > .env << 'EOF'
DATABASE_URL=postgresql://postgres@localhost:5432/ephemeral_vault
SOLANA_RPC_URL=http://localhost:8899
SOLANA_WS_URL=ws://localhost:8900
VAULT_PROGRAM_ID=11111111111111111111111111111111
JWT_SECRET=xK7mP9vL2qR8sT3xY6zQ1wE5rT9uI2oP5sA7dF4gH1jK
PORT=8080
MAX_CONCURRENT_SESSIONS=1000
SESSION_DURATION_SECS=3600
INACTIVITY_TIMEOUT_SECS=1800
RUST_LOG=info
LOG_FILE=./logs/app.log
ENABLE_RATE_LIMITING=true
ENABLE_ANOMALY_DETECTION=true
ENABLE_DEVICE_FINGERPRINTING=true
RATE_LIMIT_PER_MINUTE=100
ANOMALY_DETECTION_MULTIPLIER=2.5
MIN_DEPOSIT_AMOUNT=5000
MAX_DEPOSIT_AMOUNT=10000000000
EOF
```

**Or use VS Code:**
1. Open VS Code
2. Go to File → Open Folder → Select `/Users/anuj/Desktop/Quant_vs`
3. Click New File
4. Name it `.env`
5. Paste the content above
6. Save (Cmd+S)

---

## **PART 7: Run the Tests**

### **Before Running Tests - Check Prerequisites**

```bash
# Verify Rust
rustc --version

# Verify PostgreSQL is running
psql -d ephemeral_vault -c "SELECT 1;"

# You should see:
# ?column?
# ----------
#        1
```

If the PostgreSQL check fails, run:
```bash
brew services start postgresql  # Mac
# or
sudo systemctl start postgresql  # Linux
```

---

### **Run the Tests**

Navigate to your project and run tests:

```bash
# Go to project folder
cd /Users/anuj/Desktop/Quant_vs

# Create logs directory first
mkdir -p logs

# Run ALL tests
cargo test --all

# OR run with output visible
cargo test --all -- --nocapture

# OR run specific tests
cargo test --lib  # Only unit tests

# OR run backend tests only
cd backend
cargo test --lib
cd ..
```

---

## **What You'll See**

When tests run, you'll see output like:

```
running 9 tests

test vault::tests::test_ephemeral_vault_creation ... ok
test vault::tests::test_session_duration_validation ... ok
test managers::session_manager::tests::test_session_generation ... ok
...

test result: ok. 9 passed; 0 failed; 0 ignored
``

**That means everything works! ✅**

---

## **Troubleshooting: Common Problems**

### **Problem 1: "DatabaseError" when running tests**

**Error message:** `DatabaseError: connection refused`

**Solution:**
```bash
# Start PostgreSQL
brew services start postgresql

# Verify it's running
psql -d ephemeral_vault -c "SELECT 1;"
```

---

### **Problem 2: "Connection refused" on port 8899**

**Error:** SOLANA_RPC_URL points to localhost but it's not running

**Solution for testing:**
This is OK! For tests, we don't need a real Solana connection. Just leave it in the .env file.

---

### **Problem 3: ".env file not found"**

**Error:** `thread 'main' panicked at 'Failed to load .env'`

**Solution:**
```bash
# Make sure you're in the right directory
cd /Users/anuj/Desktop/Quant_vs

# Check if .env exists
ls -la .env

# If not, create it
cat > .env << 'EOF'
DATABASE_URL=postgresql://postgres@localhost:5432/ephemeral_vault
SOLANA_RPC_URL=http://localhost:8899
SOLANA_WS_URL=ws://localhost:8900
VAULT_PROGRAM_ID=11111111111111111111111111111111
JWT_SECRET=xK7mP9vL2qR8sT3xY6zQ1wE5rT9uI2oP5sA7dF4gH1jK
PORT=8080
MAX_CONCURRENT_SESSIONS=1000
SESSION_DURATION_SECS=3600
INACTIVITY_TIMEOUT_SECS=1800
RUST_LOG=info
LOG_FILE=./logs/app.log
ENABLE_RATE_LIMITING=true
ENABLE_ANOMALY_DETECTION=true
ENABLE_DEVICE_FINGERPRINTING=true
RATE_LIMIT_PER_MINUTE=100
ANOMALY_DETECTION_MULTIPLIER=2.5
MIN_DEPOSIT_AMOUNT=5000
MAX_DEPOSIT_AMOUNT=10000000000
EOF
```

---

### **Problem 4: Rust compilation errors**

**Error:** `error: could not compile ... `

**Solution:**
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

---

## **SUMMARY: Quick Checklist**

- [ ] Installed Rust (`rustc --version` works)
- [ ] Installed PostgreSQL (`psql --version` works)
- [ ] Installed Solana CLI (`solana --version` works)
- [ ] Created database (`psql -d ephemeral_vault -c "SELECT 1;"` works)
- [ ] Ran migrations (`\dt` shows 5 tables)
- [ ] Created .env file in `/Users/anuj/Desktop/Quant_vs/`
- [ ] JWT_SECRET is filled in (or used provided value)
- [ ] PostgreSQL is running
- [ ] Ready to run: `cargo test --all`

---

## **What's REQUIRED vs OPTIONAL**

| Setting | Required? | What to Do |
|---------|-----------|-----------|
| DATABASE_URL | ✅ YES | Leave as default or use your password |
| SOLANA_RPC_URL | ✅ YES | Leave as `http://localhost:8899` |
| SOLANA_WS_URL | ✅ YES | Leave as `ws://localhost:8900` |
| VAULT_PROGRAM_ID | ✅ YES | Leave as `11111111111111111111111111111111` |
| JWT_SECRET | ✅ YES | Generate using `openssl rand -base64 32` |
| PORT | ⚙️ OPTIONAL | Leave as 8080 |
| MAX_CONCURRENT_SESSIONS | ⚙️ OPTIONAL | Leave as 1000 |
| SESSION_DURATION_SECS | ⚙️ OPTIONAL | Leave as 3600 |
| INACTIVITY_TIMEOUT_SECS | ⚙️ OPTIONAL | Leave as 1800 |
| RUST_LOG | ⚙️ OPTIONAL | Leave as info |
| LOG_FILE | ⚙️ OPTIONAL | Leave as ./logs/app.log |
| ENABLE_RATE_LIMITING | ⚙️ OPTIONAL | Leave as true |
| ENABLE_ANOMALY_DETECTION | ⚙️ OPTIONAL | Leave as true |
| ENABLE_DEVICE_FINGERPRINTING | ⚙️ OPTIONAL | Leave as true |
| RATE_LIMIT_PER_MINUTE | ⚙️ OPTIONAL | Leave as 100 |
| ANOMALY_DETECTION_MULTIPLIER | ⚙️ OPTIONAL | Leave as 2.5 |
| MIN_DEPOSIT_AMOUNT | ⚙️ OPTIONAL | Leave as 5000 |
| MAX_DEPOSIT_AMOUNT | ⚙️ OPTIONAL | Leave as 10000000000 |

---

## **NEXT: Run the Tests!**

Once everything is set up:

```bash
cd /Users/anuj/Desktop/Quant_vs
cargo test --all
```

You should see:
```
test result: ok. 54 passed; 0 failed
```

**That's it! Everything is working! 🎉**

---

**Questions?** Every command is shown above. Just copy-paste them into Terminal.

**Problems?** Scroll up to "Troubleshooting" section.

Good luck! 🚀
