# Security

`schwab-cli` stores OAuth app credentials, tokens, account maps, and snapshots locally. Treat those files as secrets.

Default paths:

- `~/.config/schwab-cli/app.json`
- `~/.config/schwab-cli/token.json`
- `~/.config/schwab-cli/accounts.json`
- `~/.local/share/schwab-cli/`

Recommended practices:

- Do not commit config, token, account-map, snapshot, order, or exported brokerage files.
- Use `orders preview` before any live order mutation.
- Live order mutations require `SCHWAB_CLI_ALLOW_LIVE_TRADING=1` and `--yes-live-order` by design.
- Review `docs/schwab-api/` licensing/provenance before publishing generated reference material.
