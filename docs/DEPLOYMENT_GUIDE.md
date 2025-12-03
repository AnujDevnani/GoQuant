# Deployment Guide

## Pre-Deployment Checklist

- [ ] Rust 1.75+ installed
- [ ] Solana CLI tools installed
- [ ] PostgreSQL 13+ running
- [ ] Node.js 16+ installed
- [ ] Git repository initialized
- [ ] Environment variables configured

## Step 1: Smart Contract Deployment

### Build Anchor Program

```bash
# Navigate to workspace
cd /path/to/ephemeral-vault

# Install Anchor (if not already installed)
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.29.0
avm use 0.29.0

# Build the program
anchor build
```

### Deploy to Devnet

```bash
# Configure Solana CLI
solana config set --url devnet
solana-keygen new -o ~/my-keypair.json

# Fund your keypair (request from devnet faucet)
solana airdrop 5

# Deploy
anchor deploy --provider.cluster devnet

# Output will show:
# Program ID: YOUR_PROGRAM_ID
# Save this for later!
```

### Verify Deployment

```bash
# Check program
solana program show YOUR_PROGRAM_ID

# Get program authority
solana program show YOUR_PROGRAM_ID --programs
```

## Step 2: Database Setup

### Create Database

```bash
# Connect to PostgreSQL
psql -U postgres

# Create database
CREATE DATABASE ephemeral_vault OWNER postgres;

# Exit psql
\q
```

### Run Migrations

```bash
# Apply schema
psql -d ephemeral_vault < migrations/001_initial_schema.sql

# Verify tables created
psql -d ephemeral_vault -c "\dt"

# Should show:
# - sessions
# - vault_transactions
# - delegations
# - cleanup_events
# - security_events
```

### Create Database User (Production)

```bash
psql -U postgres

CREATE USER vault_service WITH PASSWORD 'secure_password_here';
GRANT CONNECT ON DATABASE ephemeral_vault TO vault_service;
GRANT USAGE ON SCHEMA public TO vault_service;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO vault_service;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO vault_service;

\q
```

## Step 3: Backend Configuration

### Create Environment File

```bash
cp .env.example .env
```

Edit `.env`:

```env
# Database
DATABASE_URL=postgresql://vault_service:password@localhost:5432/ephemeral_vault

# Solana
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_WS_URL=wss://api.devnet.solana.com
VAULT_PROGRAM_ID=YOUR_PROGRAM_ID_FROM_DEPLOYMENT

# Security
JWT_SECRET=your-secure-random-key-here-minimum-32-chars

# Service
PORT=8080
MAX_CONCURRENT_SESSIONS=1000
SESSION_DURATION_SECS=3600
INACTIVITY_TIMEOUT_SECS=1800

# Logging
RUST_LOG=info
```

### Generate Secure JWT Secret

```bash
# Generate secure random secret
openssl rand -base64 32
# Copy output to JWT_SECRET in .env
```

## Step 4: Backend Build

### Build Release Binary

```bash
cd backend

# Build optimized release
cargo build --release

# Binary location: ./target/release/ephemeral-vault-backend
```

### Test Build

```bash
# Run tests before deploying
cargo test --release

# Output should show: test result: ok
```

## Step 5: Start Backend

### Development

```bash
# Run with logging
RUST_LOG=debug cargo run
```

### Production

```bash
# Run in background
nohup cargo run --release > app.log 2>&1 &

# Or use systemd (see systemd service below)
systemctl start ephemeral-vault
```

### Verify Running

```bash
# Check API health
curl http://localhost:8080/api/analytics/user/test

# Should return 200 with response
```

## Step 6: Production Setup

### Using Systemd

Create `/etc/systemd/system/ephemeral-vault.service`:

```ini
[Unit]
Description=Ephemeral Vault Backend Service
After=network.target

[Service]
Type=simple
User=vault
WorkingDirectory=/opt/ephemeral-vault
ExecStart=/opt/ephemeral-vault/target/release/ephemeral-vault-backend
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
EnvironmentFile=/opt/ephemeral-vault/.env

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable ephemeral-vault
sudo systemctl start ephemeral-vault

# Check status
sudo systemctl status ephemeral-vault

# View logs
sudo journalctl -u ephemeral-vault -f
```

### Using Supervisor

Create `/etc/supervisor/conf.d/ephemeral-vault.conf`:

```ini
[program:ephemeral-vault]
command=/opt/ephemeral-vault/target/release/ephemeral-vault-backend
directory=/opt/ephemeral-vault
user=vault
autostart=true
autorestart=true
redirect_stderr=true
stdout_logfile=/var/log/ephemeral-vault.log
environment=PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin",RUST_LOG="info"
```

Start:

```bash
sudo supervisorctl reread
sudo supervisorctl update
sudo supervisorctl start ephemeral-vault
```

### Reverse Proxy (Nginx)

Create `/etc/nginx/sites-available/ephemeral-vault`:

```nginx
upstream vault_backend {
    server 127.0.0.1:8080;
}

server {
    listen 443 ssl http2;
    server_name api.goquant.io;

    ssl_certificate /etc/letsencrypt/live/api.goquant.io/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.goquant.io/privkey.pem;

    # SSL configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;

    # Security headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-Frame-Options "DENY" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=api_limit:10m rate=10r/s;
    limit_req zone=api_limit burst=20 nodelay;

    location /api/ {
        proxy_pass http://vault_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # Timeouts
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }
}

# Redirect HTTP to HTTPS
server {
    listen 80;
    server_name api.goquant.io;
    return 301 https://$server_name$request_uri;
}
```

Enable:

```bash
sudo ln -s /etc/nginx/sites-available/ephemeral-vault \
           /etc/nginx/sites-enabled/

sudo nginx -t
sudo systemctl restart nginx
```

## Step 7: Monitoring & Logging

### Application Logs

```bash
# View recent logs
tail -f /var/log/ephemeral-vault.log

# View specific level
grep "ERROR" /var/log/ephemeral-vault.log

# Archive old logs
gzip /var/log/ephemeral-vault.log.*
```

### Database Monitoring

```bash
# Connect to database
psql -d ephemeral_vault

# Check active sessions
SELECT pid, usename, application_name, state 
FROM pg_stat_activity;

# Check table sizes
SELECT schemaname, tablename, pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename))
FROM pg_tables
WHERE schemaname = 'public'
ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;

# Analyze performance
EXPLAIN ANALYZE SELECT * FROM sessions LIMIT 10;
```

### Health Checks

```bash
# API health endpoint
curl http://localhost:8080/api/analytics/user/test

# Database connectivity
psql -d ephemeral_vault -c "SELECT NOW();"

# Solana RPC connectivity
curl https://api.devnet.solana.com -X POST -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'
```

## Step 8: Backup & Recovery

### Database Backup

```bash
# Full backup
pg_dump ephemeral_vault > backup_$(date +%Y%m%d_%H%M%S).sql

# Compressed backup
pg_dump ephemeral_vault | gzip > backup_$(date +%Y%m%d_%H%M%S).sql.gz

# Regular automated backups (crontab)
# Add to crontab: 0 2 * * * pg_dump ephemeral_vault | gzip > /backups/vault_$(date +\%Y\%m\%d).sql.gz
```

### Database Recovery

```bash
# From backup
psql ephemeral_vault < backup_20231203_120000.sql

# From compressed backup
gunzip -c backup_20231203_120000.sql.gz | psql ephemeral_vault
```

### Program State Backup

```bash
# Backup program keypair
cp ~/.config/solana/id.json ~/.config/solana/id.json.backup

# Backup program state
solana program dump YOUR_PROGRAM_ID program_state.bin
```

## Step 9: Upgrades

### Update Backend

```bash
# Pull latest code
git pull origin main

# Rebuild
cargo build --release

# Stop current instance
sudo systemctl stop ephemeral-vault

# Deploy new binary
cp target/release/ephemeral-vault-backend /opt/ephemeral-vault/

# Start new instance
sudo systemctl start ephemeral-vault

# Verify
curl http://localhost:8080/api/health
```

### Update Database Schema

```bash
# Create migration file
cat > migrations/002_add_new_column.sql << 'EOF'
ALTER TABLE sessions ADD COLUMN new_field VARCHAR(255);
CREATE INDEX idx_sessions_new_field ON sessions(new_field);
EOF

# Apply migration
psql -d ephemeral_vault < migrations/002_add_new_column.sql
```

## Troubleshooting

### Backend Won't Start

```bash
# Check logs
journalctl -u ephemeral-vault -n 50

# Verify port availability
lsof -i :8080

# Check database connectivity
psql -d ephemeral_vault -c "SELECT 1;"
```

### Database Connection Errors

```bash
# Check PostgreSQL service
systemctl status postgresql

# Check credentials
psql -U vault_service -d ephemeral_vault -c "SELECT 1;"

# Check host/port
psql -h localhost -U vault_service -d ephemeral_vault
```

### API Response Issues

```bash
# Check backend health
curl -v http://localhost:8080/api/analytics/user/test

# Check logs for errors
tail -f /var/log/ephemeral-vault.log | grep ERROR

# Check database queries
tail -f /var/log/postgresql.log
```

## Performance Optimization

### Database Optimization

```sql
-- Add indexes
CREATE INDEX idx_sessions_user_wallet ON sessions(user_wallet);
CREATE INDEX idx_sessions_created_at ON sessions(created_at);

-- Analyze query plans
EXPLAIN ANALYZE SELECT * FROM sessions WHERE user_wallet = 'test';

-- Vacuum and analyze
VACUUM ANALYZE;
```

### Backend Optimization

```bash
# Enable connection pooling
# Edit .env:
DATABASE_POOL_SIZE=20
DATABASE_QUEUE_TIMEOUT=30

# Increase ulimit for more connections
ulimit -n 65536

# Use jemalloc for better memory allocation
MALLOC_CONF=prof:true cargo run --release
```

## Security Hardening

### System Security

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Configure firewall
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow 22/tcp   # SSH
sudo ufw allow 80/tcp   # HTTP
sudo ufw allow 443/tcp  # HTTPS
sudo ufw enable

# Set up fail2ban
sudo apt install fail2ban
sudo systemctl start fail2ban
```

### Application Security

```bash
# Set strong JWT secret
JWT_SECRET=$(openssl rand -base64 32)

# Enable HTTPS only
# (See Nginx config above)

# Set secure database password
# Use: pwgen -s 32 1
```

## Monitoring Dashboard

Set up monitoring with:

1. **Prometheus**: Metrics collection
2. **Grafana**: Visualization
3. **AlertManager**: Alerts
4. **ELK Stack**: Log aggregation

See [monitoring setup guide](./docs/MONITORING.md) for details.

---

**Status**: Ready for Production
**Last Updated**: December 3, 2025
