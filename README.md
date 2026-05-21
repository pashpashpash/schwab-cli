# schwab-cli

Agent-first Rust CLI for Schwab Trader API accounts, orders, market data, streaming, and local snapshots.

The CLI is JSON-first by default so it composes cleanly with `jq`, shell pipes, scripts, and agents. Human diagnostics and API errors go to stderr. Use `--agent` for a compact agent envelope with overflow hints.

## Status

This is a personal-tooling extraction prepared for open source. It is not affiliated with, endorsed by, or supported by Charles Schwab.

The generated reference files in `docs/schwab-api/` were extracted from locally saved Schwab Developer Portal pages. They are included as development reference material; review Schwab's developer terms before redistributing them publicly.

## Install

```bash
cargo build --release
```

Optional global install:

```bash
mkdir -p ~/bin
ln -sf "$PWD/target/release/schwab-cli" ~/bin/schwab-cli
```

Or install with Cargo:

```bash
cargo install --path .
```

## Configuration

By default, local files live outside the repo:

```text
~/.config/schwab-cli/app.json       # OAuth app client ID, secret, callback URL
~/.config/schwab-cli/token.json     # access token, refresh token, expiry metadata
~/.config/schwab-cli/accounts.json  # encrypted Schwab account IDs, aliases, last4 mapping
~/.local/share/schwab-cli/          # snapshots, overflow files, logs
```

Override paths when needed:

```bash
export SCHWAB_CLI_CONFIG_DIR=/secure/path/schwab-cli
export SCHWAB_CLI_DATA_DIR=/data/path/schwab-cli
```

Never commit real `app.json`, `token.json`, `accounts.json`, order files with account IDs, or snapshots with personal holdings.

## Auth

A Schwab developer app identifies the application. It does not grant account access until the account holder completes OAuth consent.

```bash
schwab-cli setup \
  --client-id '<client id>' \
  --client-secret '<client secret>' \
  --callback-url https://127.0.0.1:8182/callback

schwab-cli auth url --open
schwab-cli auth exchange --callback-url '<full redirected callback URL containing code=...>'
schwab-cli account-numbers
```

Interactive paste flow:

```bash
schwab-cli auth login
```

Schwab refresh tokens are short-lived. API commands auto-refresh while the refresh token is still valid. To keep auth warm on macOS, install the launchd keepalive:

```bash
schwab-cli auth keepalive install --bin "$HOME/bin/schwab-cli"
schwab-cli auth keepalive status
```

If `auth status` says the refresh token is expired, run `schwab-cli auth login` again.

## Coverage

Account and trading:

```bash
schwab-cli account-numbers
schwab-cli accounts list --positions
schwab-cli accounts get --account individual --positions
schwab-cli accounts alias --account 1234 --name individual
schwab-cli cash status --account individual
schwab-cli transactions list --account individual --start 2026-05-01 --end 2026-05-21
schwab-cli orders list --account individual --from 2026-05-01T00:00:00Z --to 2026-05-21T23:59:59Z
schwab-cli orders get --account individual --order-id 123
schwab-cli orders preview --account individual --json-file order.json
```

Market data:

```bash
schwab-cli market quotes --symbols AAPL,GOOGL,LLY
schwab-cli market quote --symbol AAPL
schwab-cli market chains --symbol AAPL
schwab-cli market expiration-chain --symbol AAPL
schwab-cli market price-history --symbol AAPL --period-type year --period 1 --frequency-type daily --frequency 1
schwab-cli market movers --index '$SPX'
schwab-cli market hours --markets equity,option --date 2026-05-21
schwab-cli market instrument-search --symbol AAPL --projection symbol-search
schwab-cli market instrument --cusip 037833100
```

Streaming:

```bash
schwab-cli streamer info
schwab-cli streamer fields
schwab-cli streamer fields --service LEVELONE_EQUITIES
schwab-cli streamer listen --service LEVELONE_EQUITIES --keys AAPL,MSFT --fields 0,1,2,3 --jsonl
schwab-cli streamer listen --service ACCT_ACTIVITY --jsonl
```

Docs and generic escape hatches:

```bash
schwab-cli docs list
schwab-cli docs endpoint getAccount
schwab-cli docs model Order
schwab-cli docs search pricehistory
schwab-cli get --base trader --path /accounts --param fields=positions
schwab-cli get --base market --path /quotes --param symbols=AAPL,MSFT
schwab-cli post --base trader --path /accounts/{account}/previewOrder --body-file order.json
```

Snapshots:

```bash
schwab-cli snapshot --include-orders --include-transactions --quotes AAPL,GOOGL,LLY
```

## Cash Status

Use `cash status` before sizing trades:

```bash
schwab-cli cash status --account individual
schwab-cli cash status --account individual --raw
```

This command calls `GET /accounts/{accountNumber}` only. It does not run hidden order previews and never places orders.

Important Schwab behavior: the web UI can show same-day ACH funds as held or available differently than Trader API balance fields. Treat `non_margin_trade_capacity` as the conservative cash-only sizing field, treat `stock_buying_power` as margin-sensitive, and use `orders preview` for concrete order preflight.

## Live Order Guard

Previewing orders is intentionally easy:

```bash
schwab-cli orders preview --account individual --json-file order.json
```

Live order mutations require both an environment variable and an explicit flag:

```bash
SCHWAB_CLI_ALLOW_LIVE_TRADING=1 \
  schwab-cli orders place --account individual --json-file order.json --yes-live-order
```

The same guard applies to order replace/cancel and generic order mutation endpoints.

## API Boundaries

Schwab Trader API OAuth exposes only accounts Schwab makes available during consent. Some workplace retirement-plan, banking, fixed-income, or other Schwab surfaces may be visible on Schwab web but absent from API account lists.

The documented order-entry surface is equities/options oriented. Direct Treasury auction, CD, bond ladder, or fixed-income order-placement flows may require Schwab web even if positions or instruments are visible through API data endpoints.

## Development

```bash
cargo fmt
cargo test
cargo run -- version
cargo run -- doctor
```

## License

Code is released under the MIT License. Generated Schwab reference docs under `docs/schwab-api/` are third-party-derived reference material and are not official Schwab documentation.
