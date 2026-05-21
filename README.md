# schwab-cli

`schwab-cli` is an agent-first Rust CLI for the Schwab Trader API. It is built for LLMs, shell scripts, and humans who prefer composable tools: JSON to stdout, diagnostics to stderr, useful `--help`, explicit safety rails around live trading, and an `--agent` mode for compact machine-readable envelopes.

It covers OAuth, account numbers, balances, positions, transactions, orders, order preview/place/replace/cancel, market quotes, option chains, price history, movers, market hours, instruments, streamer metadata/listening, snapshots, and generic REST escape hatches.

This project is not affiliated with or endorsed by Charles Schwab.

## Install

The simple path installs the latest GitHub release into `~/bin/schwab-cli`:

```bash
curl -fsSL https://raw.githubusercontent.com/pashpashpash/schwab-cli/main/install.sh | sh
```

Make sure `~/bin` is on your `PATH`, then verify:

```bash
schwab-cli version
schwab-cli doctor
```

From source:

```bash
git clone https://github.com/pashpashpash/schwab-cli.git
cd schwab-cli
cargo install --path .
```

Update an existing install from the latest GitHub release:

```bash
schwab-cli update
```

For nonstandard installs:

```bash
schwab-cli update --bin /path/to/schwab-cli
```

## Auth

Create a Schwab developer app, then configure the CLI with its client ID, client secret, and callback URL:

```bash
schwab-cli setup \
  --client-id '<client id>' \
  --client-secret '<client secret>' \
  --callback-url https://127.0.0.1:8182/callback
```

Connect the app to your actual Schwab account through OAuth:

```bash
schwab-cli auth login --open
schwab-cli account-numbers
```

Schwab refresh tokens are short-lived. API commands refresh automatically while the refresh token is still valid. On macOS, install a daily launchd refresher:

```bash
schwab-cli auth keepalive install --bin "$HOME/bin/schwab-cli"
```

## Local Files

Secrets and snapshots live outside the repo by default:

```text
~/.config/schwab-cli/app.json
~/.config/schwab-cli/token.json
~/.config/schwab-cli/accounts.json
~/.local/share/schwab-cli/
```

Overrides:

```bash
export SCHWAB_CLI_CONFIG_DIR=/secure/path/schwab-cli
export SCHWAB_CLI_DATA_DIR=/data/path/schwab-cli
```

Never commit real app credentials, OAuth tokens, account maps, order files, snapshots, or exported brokerage data.

## Examples

```bash
schwab-cli account-numbers
schwab-cli accounts list --positions
schwab-cli cash status --account individual
schwab-cli transactions list --account individual --start 2026-05-01 --end 2026-05-21
schwab-cli orders preview --account individual --json-file order.json
schwab-cli market quotes --symbols AAPL,GOOGL,LLY
schwab-cli market price-history --symbol AAPL --period-type year --period 1 --frequency-type daily --frequency 1
schwab-cli streamer fields --service LEVELONE_EQUITIES
schwab-cli streamer listen --service LEVELONE_EQUITIES --keys AAPL,MSFT --jsonl
schwab-cli snapshot --include-orders --include-transactions --quotes AAPL,GOOGL,LLY
```

Generic escape hatch:

```bash
schwab-cli get --base trader --path /accounts --param fields=positions
schwab-cli get --base market --path /quotes --param symbols=AAPL,MSFT
schwab-cli post --base trader --path /accounts/{account}/previewOrder --body-file order.json
```

## Live Trading Safety

Order previews are easy and encouraged:

```bash
schwab-cli orders preview --account individual --json-file order.json
```

Live order mutations require both an environment variable and an explicit flag:

```bash
SCHWAB_CLI_ALLOW_LIVE_TRADING=1 \
  schwab-cli orders place --account individual --json-file order.json --yes-live-order
```

The same guard applies to replace/cancel and generic order mutation endpoints.

## API Boundaries

Schwab OAuth exposes only accounts Schwab offers during consent. Some workplace retirement-plan, banking, fixed-income, or other Schwab surfaces may be visible on Schwab web but absent from the Trader API.

Direct Treasury auction, CD, bond ladder, or fixed-income order-entry workflows may require Schwab web even when positions or instruments are visible through API data endpoints.

## Generated Schwab Docs

`docs/schwab-api/` contains generated reference material extracted from locally saved Schwab Developer Portal pages. It is included for development convenience, is not official Schwab documentation, and may be stale. Review Schwab's developer terms before redistributing those generated files.

## Development

```bash
cargo fmt --check
cargo test --locked
cargo run -- version
```

Pushes to `main` run CI and rebuild the mutable `latest` GitHub release. Pushing a `v*` tag creates a tagged release.

## License

Code is MIT licensed. Generated Schwab reference docs are third-party-derived reference material and are not official Schwab documentation.
