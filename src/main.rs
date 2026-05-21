use anyhow::{anyhow, bail, Context, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::{DateTime, Duration, Utc};
use clap::{Args, Parser, Subcommand, ValueEnum};
use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_ENCODING, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, HashMap};
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use std::time::Instant;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message};
use uuid::Uuid;

const TRADER_BASE: &str = "https://api.schwabapi.com/trader/v1";
const MARKET_BASE: &str = "https://api.schwabapi.com/marketdata/v1";
const OAUTH_AUTHORIZE_URL: &str = "https://api.schwabapi.com/v1/oauth/authorize";
const OAUTH_TOKEN_URL: &str = "https://api.schwabapi.com/v1/oauth/token";
const DEFAULT_CALLBACK_URL: &str = "https://127.0.0.1:8182/callback";
const ACCESS_REFRESH_SKEW_SECONDS: i64 = 120;
const REFRESH_TOKEN_RENEWAL_SKEW_SECONDS: i64 = 24 * 60 * 60;
const REFRESH_TOKEN_LIFETIME_DAYS: i64 = 7;
const KEEPALIVE_LABEL: &str = "com.schwab-cli.refresh";
const DEFAULT_KEEPALIVE_HOUR: u8 = 6;
const DEFAULT_KEEPALIVE_MINUTE: u8 = 30;
const AGENT_MAX_BYTES: usize = 50 * 1024;
const AGENT_MAX_LINES: usize = 200;
const SCHWAB_COVERAGE_NOTE: &str = "Schwab Trader API OAuth exposes only accounts Schwab makes available during consent. Some workplace, retirement-plan, fixed-income, banking, or other Schwab accounts may be absent from OAuth even when visible in Schwab web.";

const TRANSACTION_TYPES: &[&str] = &[
    "TRADE",
    "RECEIVE_AND_DELIVER",
    "DIVIDEND_OR_INTEREST",
    "ACH_RECEIPT",
    "ACH_DISBURSEMENT",
    "CASH_RECEIPT",
    "CASH_DISBURSEMENT",
    "ELECTRONIC_FUND",
    "WIRE_OUT",
    "WIRE_IN",
    "JOURNAL",
    "MEMORANDUM",
    "MARGIN_CALL",
    "MONEY_MARKET",
    "SMA_ADJUSTMENT",
];

#[derive(Parser)]
#[command(name = "schwab-cli")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Agent-first Schwab CLI for accounts, trading, market data, and snapshots")]
struct Cli {
    #[arg(
        long,
        global = true,
        help = "Wrap output in an agent-oriented envelope with overflow hints"
    )]
    agent: bool,

    #[arg(long, global = true, help = "Emit compact JSON instead of pretty JSON")]
    compact: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Version,
    Doctor,
    Setup(SetupArgs),
    Auth(AuthCommand),
    AccountNumbers,
    Accounts(AccountsCommand),
    Cash(CashCommand),
    Transactions(TransactionsCommand),
    Orders(OrdersCommand),
    Market(MarketCommand),
    Streamer(StreamerCommand),
    Get(GenericGetArgs),
    Post(GenericWriteArgs),
    Put(GenericWriteArgs),
    Delete(GenericDeleteArgs),
    Docs(DocsCommand),
    Snapshot(SnapshotArgs),
}

#[derive(Args)]
struct SetupArgs {
    #[arg(long)]
    client_id: String,
    #[arg(long)]
    client_secret: String,
    #[arg(long, default_value = DEFAULT_CALLBACK_URL)]
    callback_url: String,
}

#[derive(Subcommand)]
enum AuthSubcommand {
    Status,
    Url(AuthUrlArgs),
    Login(AuthUrlArgs),
    Exchange(AuthExchangeArgs),
    Refresh,
    Keepalive(AuthKeepaliveCommand),
    Clear,
}

#[derive(Args)]
struct AuthCommand {
    #[command(subcommand)]
    command: AuthSubcommand,
}

#[derive(Args, Clone)]
struct AuthUrlArgs {
    #[arg(long, help = "Open the authorization URL with macOS open")]
    open: bool,
}

#[derive(Args)]
struct AuthExchangeArgs {
    #[arg(long, conflicts_with = "code")]
    callback_url: Option<String>,
    #[arg(long, conflicts_with = "callback_url")]
    code: Option<String>,
}

#[derive(Args)]
struct AuthKeepaliveCommand {
    #[command(subcommand)]
    command: AuthKeepaliveSubcommand,
}

#[derive(Subcommand)]
enum AuthKeepaliveSubcommand {
    Status,
    Install(AuthKeepaliveInstallArgs),
    Uninstall,
    Plist(AuthKeepaliveInstallArgs),
}

#[derive(Args, Clone)]
struct AuthKeepaliveInstallArgs {
    #[arg(long, default_value_t = DEFAULT_KEEPALIVE_HOUR)]
    hour: u8,
    #[arg(long, default_value_t = DEFAULT_KEEPALIVE_MINUTE)]
    minute: u8,
    #[arg(
        long,
        default_value = "/usr/local/bin/schwab-cli",
        help = "Absolute schwab-cli path launchd should execute"
    )]
    bin: PathBuf,
}

#[derive(Subcommand)]
enum AccountsSubcommand {
    List(AccountListArgs),
    Get(AccountGetArgs),
    Alias(AccountAliasArgs),
}

#[derive(Args)]
struct AccountsCommand {
    #[command(subcommand)]
    command: AccountsSubcommand,
}

#[derive(Args)]
struct AccountListArgs {
    #[arg(long)]
    positions: bool,
}

#[derive(Args)]
struct AccountGetArgs {
    #[arg(long)]
    account: String,
    #[arg(long)]
    positions: bool,
}

#[derive(Args)]
struct AccountAliasArgs {
    #[arg(long)]
    account: String,
    #[arg(long)]
    name: String,
}

#[derive(Subcommand)]
enum CashSubcommand {
    Status(CashStatusArgs),
}

#[derive(Args)]
struct CashCommand {
    #[command(subcommand)]
    command: CashSubcommand,
}

#[derive(Args)]
struct CashStatusArgs {
    #[arg(long)]
    account: String,
    #[arg(long, help = "Include the full raw securitiesAccount payload")]
    raw: bool,
}

#[derive(Subcommand)]
enum TransactionsSubcommand {
    List(TransactionListArgs),
    Get(TransactionGetArgs),
}

#[derive(Args)]
struct TransactionsCommand {
    #[command(subcommand)]
    command: TransactionsSubcommand,
}

#[derive(Args)]
struct TransactionListArgs {
    #[arg(long)]
    account: String,
    #[arg(long)]
    start: String,
    #[arg(long)]
    end: String,
    #[arg(long)]
    symbol: Option<String>,
    #[arg(long = "type", alias = "types")]
    types: Vec<String>,
}

#[derive(Args)]
struct TransactionGetArgs {
    #[arg(long)]
    account: String,
    #[arg(long)]
    transaction_id: String,
}

#[derive(Subcommand)]
enum OrdersSubcommand {
    List(OrderListArgs),
    All(OrderAllArgs),
    Get(OrderGetArgs),
    Preview(OrderBodyArgs),
    Place(OrderBodyArgs),
    Replace(OrderReplaceArgs),
    Cancel(OrderCancelArgs),
}

#[derive(Args)]
struct OrdersCommand {
    #[command(subcommand)]
    command: OrdersSubcommand,
}

#[derive(Args)]
struct OrderListArgs {
    #[arg(long)]
    account: String,
    #[arg(long)]
    from: String,
    #[arg(long)]
    to: String,
    #[arg(long)]
    status: Option<String>,
    #[arg(long, default_value_t = 3000)]
    max_results: u32,
}

#[derive(Args)]
struct OrderAllArgs {
    #[arg(long)]
    from: String,
    #[arg(long)]
    to: String,
    #[arg(long)]
    status: Option<String>,
    #[arg(long, default_value_t = 3000)]
    max_results: u32,
}

#[derive(Args)]
struct OrderGetArgs {
    #[arg(long)]
    account: String,
    #[arg(long)]
    order_id: String,
}

#[derive(Args)]
struct OrderBodyArgs {
    #[arg(long)]
    account: String,
    #[arg(long)]
    json_file: PathBuf,
    #[arg(long)]
    yes_live_order: bool,
}

#[derive(Args)]
struct OrderReplaceArgs {
    #[arg(long)]
    account: String,
    #[arg(long)]
    order_id: String,
    #[arg(long)]
    json_file: PathBuf,
    #[arg(long)]
    yes_live_order: bool,
}

#[derive(Args)]
struct OrderCancelArgs {
    #[arg(long)]
    account: String,
    #[arg(long)]
    order_id: String,
    #[arg(long)]
    yes_live_order: bool,
}

#[derive(Subcommand)]
enum MarketSubcommand {
    Quotes(QuotesArgs),
    Quote(QuoteArgs),
    Chains(ChainsArgs),
    ExpirationChain(SymbolArgs),
    PriceHistory(PriceHistoryArgs),
    Movers(MoversArgs),
    Hours(MarketHoursArgs),
    InstrumentSearch(InstrumentSearchArgs),
    Instrument(InstrumentArgs),
}

#[derive(Args)]
struct MarketCommand {
    #[command(subcommand)]
    command: MarketSubcommand,
}

#[derive(Args)]
struct QuotesArgs {
    #[arg(long)]
    symbols: String,
    #[arg(long)]
    fields: Option<String>,
    #[arg(long)]
    indicative: Option<bool>,
}

#[derive(Args)]
struct QuoteArgs {
    #[arg(long)]
    symbol: String,
    #[arg(long)]
    fields: Option<String>,
}

#[derive(Args)]
struct SymbolArgs {
    #[arg(long)]
    symbol: String,
}

#[derive(Args)]
struct ChainsArgs {
    #[arg(long)]
    symbol: String,
    #[arg(long)]
    contract_type: Option<String>,
    #[arg(long)]
    strike_count: Option<u32>,
    #[arg(long)]
    include_underlying_quote: Option<bool>,
    #[arg(long)]
    strategy: Option<String>,
    #[arg(long)]
    interval: Option<f64>,
    #[arg(long)]
    strike: Option<f64>,
    #[arg(long)]
    range: Option<String>,
    #[arg(long)]
    from_date: Option<String>,
    #[arg(long)]
    to_date: Option<String>,
    #[arg(long)]
    volatility: Option<f64>,
    #[arg(long)]
    underlying_price: Option<f64>,
    #[arg(long)]
    interest_rate: Option<f64>,
    #[arg(long)]
    days_to_expiration: Option<u32>,
    #[arg(long)]
    exp_month: Option<String>,
    #[arg(long)]
    option_type: Option<String>,
    #[arg(long)]
    entitlement: Option<String>,
}

#[derive(Args)]
struct PriceHistoryArgs {
    #[arg(long)]
    symbol: String,
    #[arg(long)]
    period_type: Option<String>,
    #[arg(long)]
    period: Option<u32>,
    #[arg(long)]
    frequency_type: Option<String>,
    #[arg(long)]
    frequency: Option<u32>,
    #[arg(long)]
    start_date: Option<String>,
    #[arg(long)]
    end_date: Option<String>,
    #[arg(long)]
    need_extended_hours_data: Option<bool>,
    #[arg(long)]
    need_previous_close: Option<bool>,
}

#[derive(Args)]
struct MoversArgs {
    #[arg(long, alias = "index")]
    symbol: String,
    #[arg(long)]
    sort: Option<String>,
    #[arg(long)]
    frequency: Option<u32>,
}

#[derive(Args)]
struct MarketHoursArgs {
    #[arg(long)]
    markets: Option<String>,
    #[arg(long)]
    market: Option<String>,
    #[arg(long)]
    date: Option<String>,
}

#[derive(Args)]
struct InstrumentSearchArgs {
    #[arg(long)]
    symbol: String,
    #[arg(long)]
    projection: String,
}

#[derive(Args)]
struct InstrumentArgs {
    #[arg(long)]
    cusip: String,
}

#[derive(Subcommand)]
enum StreamerSubcommand {
    Info,
    Fields(StreamerFieldsArgs),
    Listen(StreamerListenArgs),
}

#[derive(Args)]
struct StreamerCommand {
    #[command(subcommand)]
    command: StreamerSubcommand,
}

#[derive(Args)]
struct StreamerListenArgs {
    #[arg(long)]
    service: String,
    #[arg(long)]
    keys: Option<String>,
    #[arg(long)]
    fields: Option<String>,
    #[arg(long)]
    jsonl: bool,
    #[arg(
        long,
        help = "Do not decode numeric Schwab streamer fields into named fields"
    )]
    raw: bool,
    #[arg(
        long,
        default_value_t = 3,
        help = "Number of messages to read; use 0 for infinite"
    )]
    limit: u32,
}

#[derive(Args)]
struct StreamerFieldsArgs {
    #[arg(long)]
    service: Option<String>,
}

#[derive(Clone, Debug, ValueEnum)]
enum BaseArg {
    Trader,
    Market,
}

#[derive(Args)]
struct GenericGetArgs {
    #[arg(long, value_enum)]
    base: BaseArg,
    #[arg(long)]
    path: String,
    #[arg(long = "param")]
    params: Vec<String>,
    #[arg(long)]
    account: Option<String>,
}

#[derive(Args)]
struct GenericWriteArgs {
    #[arg(long, value_enum)]
    base: BaseArg,
    #[arg(long)]
    path: String,
    #[arg(long = "param")]
    params: Vec<String>,
    #[arg(long)]
    body_file: PathBuf,
    #[arg(long)]
    account: Option<String>,
    #[arg(long)]
    yes_live_order: bool,
}

#[derive(Args)]
struct GenericDeleteArgs {
    #[arg(long, value_enum)]
    base: BaseArg,
    #[arg(long)]
    path: String,
    #[arg(long = "param")]
    params: Vec<String>,
    #[arg(long)]
    account: Option<String>,
    #[arg(long)]
    yes_live_order: bool,
}

#[derive(Subcommand)]
enum DocsSubcommand {
    List,
    Endpoint(DocsLookupArgs),
    Model(DocsLookupArgs),
    Search(DocsSearchArgs),
}

#[derive(Args)]
struct DocsCommand {
    #[command(subcommand)]
    command: DocsSubcommand,
}

#[derive(Args)]
struct DocsLookupArgs {
    value: String,
}

#[derive(Args)]
struct DocsSearchArgs {
    query: String,
}

#[derive(Args)]
struct SnapshotArgs {
    #[arg(long)]
    include_orders: bool,
    #[arg(long)]
    include_transactions: bool,
    #[arg(long)]
    quotes: Option<String>,
    #[arg(long)]
    start: Option<String>,
    #[arg(long)]
    end: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Clone, Copy)]
enum ApiBase {
    Trader,
    Market,
}

#[derive(Serialize, Deserialize, Clone)]
struct AppConfig {
    client_id: String,
    client_secret: String,
    callback_url: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct TokenStore {
    access_token: String,
    refresh_token: String,
    token_type: String,
    scope: Option<String>,
    id_token: Option<String>,
    access_expires_at: DateTime<Utc>,
    refresh_expires_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    token_type: Option<String>,
    scope: Option<String>,
    id_token: Option<String>,
    expires_in: i64,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct AccountMap {
    accounts: Vec<AccountEntry>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct AccountEntry {
    account_number: String,
    hash_value: String,
    last4: String,
    alias: Option<String>,
    nick_name: Option<String>,
    display_acct_id: Option<String>,
    account_type: Option<String>,
}

struct SchwabClient {
    http: Client,
    app: AppConfig,
    token: TokenStore,
}

fn main() {
    let start = Instant::now();
    if let Err(err) = run(start) {
        eprintln!("[error] {err}");
        std::process::exit(1);
    }
}

fn run(start: Instant) -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Command::Version => emit(&cli, version_payload(), start, vec!["schwab-cli doctor"]),
        Command::Doctor => emit(
            &cli,
            doctor_payload(),
            start,
            vec![
                "schwab-cli setup --client-id ... --client-secret ...",
                "schwab-cli auth status",
            ],
        ),
        Command::Setup(args) => {
            let app = AppConfig {
                client_id: args.client_id.clone(),
                client_secret: args.client_secret.clone(),
                callback_url: args.callback_url.clone(),
            };
            write_secret_json(&app_config_path(), &app)?;
            emit(
                &cli,
                json!({
                    "status": "ok",
                    "app_config": app_config_path(),
                    "callback_url": app.callback_url,
                    "next": ["schwab-cli auth url", "schwab-cli auth login"]
                }),
                start,
                vec!["schwab-cli auth login"],
            )
        }
        Command::Auth(auth) => handle_auth(&cli, &auth.command, start),
        Command::AccountNumbers => {
            let mut client = load_client()?;
            let payload = sync_account_numbers(&mut client)?;
            emit(
                &cli,
                payload,
                start,
                vec!["schwab-cli accounts list --positions"],
            )
        }
        Command::Accounts(accounts) => handle_accounts(&cli, &accounts.command, start),
        Command::Cash(cash) => handle_cash(&cli, &cash.command, start),
        Command::Transactions(transactions) => {
            handle_transactions(&cli, &transactions.command, start)
        }
        Command::Orders(orders) => handle_orders(&cli, &orders.command, start),
        Command::Market(market) => handle_market(&cli, &market.command, start),
        Command::Streamer(streamer) => handle_streamer(&cli, &streamer.command, start),
        Command::Get(args) => handle_generic_get(&cli, args, start),
        Command::Post(args) => handle_generic_write(&cli, "POST", args, start),
        Command::Put(args) => handle_generic_write(&cli, "PUT", args, start),
        Command::Delete(args) => handle_generic_delete(&cli, args, start),
        Command::Docs(docs) => handle_docs(&cli, &docs.command, start),
        Command::Snapshot(args) => handle_snapshot(&cli, args, start),
    }
}

fn handle_auth(cli: &Cli, command: &AuthSubcommand, start: Instant) -> Result<()> {
    match command {
        AuthSubcommand::Status => emit(
            cli,
            auth_status_payload(),
            start,
            vec!["schwab-cli auth login"],
        ),
        AuthSubcommand::Url(args) => {
            let app = load_app_config()?;
            let url = authorization_url(&app);
            if args.open {
                open_url(&url)?;
            }
            emit(
                cli,
                json!({
                    "authorization_url": url,
                    "callback_url": app.callback_url,
                    "next": "Open URL, complete Schwab login/consent, then run schwab-cli auth exchange --callback-url '<redirected-url>'"
                }),
                start,
                vec!["schwab-cli auth exchange --callback-url '<redirected-url>'"],
            )
        }
        AuthSubcommand::Login(args) => {
            let app = load_app_config()?;
            let url = authorization_url(&app);
            eprintln!("Open this Schwab authorization URL:");
            eprintln!("{url}");
            if args.open {
                open_url(&url)?;
            }
            eprintln!("After Schwab redirects to a 404/callback page, paste the full redirected URL or the code value, then press enter:");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let code = parse_auth_code(input.trim())?;
            let token = exchange_code(&app, &code)?;
            write_secret_json(&token_path(), &token)?;
            emit(
                cli,
                redacted_token_payload(&token),
                start,
                vec!["schwab-cli account-numbers"],
            )
        }
        AuthSubcommand::Exchange(args) => {
            let app = load_app_config()?;
            let code = match (&args.callback_url, &args.code) {
                (Some(callback_url), None) => parse_auth_code(callback_url)?,
                (None, Some(code)) => parse_auth_code(code)?,
                _ => bail!(
                    "auth exchange requires --callback-url or --code. Next: schwab-cli auth url"
                ),
            };
            let token = exchange_code(&app, &code)?;
            write_secret_json(&token_path(), &token)?;
            emit(
                cli,
                redacted_token_payload(&token),
                start,
                vec!["schwab-cli account-numbers"],
            )
        }
        AuthSubcommand::Refresh => {
            let app = load_app_config()?;
            let token = load_token()?;
            ensure_refresh_token_valid(&token)?;
            let refreshed = refresh_access_token(&app, &token)?;
            write_secret_json(&token_path(), &refreshed)?;
            emit(
                cli,
                redacted_token_payload(&refreshed),
                start,
                vec!["schwab-cli account-numbers"],
            )
        }
        AuthSubcommand::Keepalive(keepalive) => {
            handle_auth_keepalive(cli, &keepalive.command, start)
        }
        AuthSubcommand::Clear => {
            remove_if_exists(&token_path())?;
            remove_if_exists(&account_map_path())?;
            emit(
                cli,
                json!({"status": "ok", "cleared": [token_path(), account_map_path()], "next": "schwab-cli auth login"}),
                start,
                vec!["schwab-cli auth login"],
            )
        }
    }
}

fn handle_auth_keepalive(
    cli: &Cli,
    command: &AuthKeepaliveSubcommand,
    start: Instant,
) -> Result<()> {
    match command {
        AuthKeepaliveSubcommand::Status => emit(
            cli,
            keepalive_status_payload(),
            start,
            vec!["schwab-cli auth keepalive install"],
        ),
        AuthKeepaliveSubcommand::Install(args) => {
            validate_keepalive_time(args.hour, args.minute)?;
            let path = keepalive_plist_path()?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::create_dir_all(schwab_data_dir().join("logs"))?;
            let plist = keepalive_plist(args);
            fs::write(&path, plist)
                .with_context(|| format!("Failed to write {}", path.display()))?;
            let load_status = ProcessCommand::new("launchctl")
                .arg("load")
                .arg("-w")
                .arg(&path)
                .status();
            let load_result = match load_status {
                Ok(status) if status.success() => json!({"status": "loaded"}),
                Ok(status) => json!({
                    "status": "load_failed",
                    "exit_code": status.code(),
                    "next": format!("launchctl load -w {}", path.display())
                }),
                Err(err) => json!({
                    "status": "load_failed",
                    "error": err.to_string(),
                    "next": format!("launchctl load -w {}", path.display())
                }),
            };
            emit(
                cli,
                json!({
                    "status": "ok",
                    "plist": path,
                    "label": KEEPALIVE_LABEL,
                    "schedule": {"hour": args.hour, "minute": args.minute},
                    "program": args.bin,
                    "launchctl": load_result,
                    "next": "schwab-cli auth keepalive status"
                }),
                start,
                vec!["schwab-cli auth keepalive status"],
            )
        }
        AuthKeepaliveSubcommand::Uninstall => {
            let path = keepalive_plist_path()?;
            let unload_status = if path.exists() {
                ProcessCommand::new("launchctl")
                    .arg("unload")
                    .arg("-w")
                    .arg(&path)
                    .status()
                    .ok()
                    .map(|status| status.success())
            } else {
                None
            };
            remove_if_exists(&path)?;
            emit(
                cli,
                json!({
                    "status": "ok",
                    "removed": path,
                    "launchctl_unloaded": unload_status,
                    "next": "schwab-cli auth keepalive install"
                }),
                start,
                vec!["schwab-cli auth keepalive install"],
            )
        }
        AuthKeepaliveSubcommand::Plist(args) => {
            validate_keepalive_time(args.hour, args.minute)?;
            emit(
                cli,
                json!({
                    "label": KEEPALIVE_LABEL,
                    "path": keepalive_plist_path()?,
                    "plist": keepalive_plist(args)
                }),
                start,
                vec!["schwab-cli auth keepalive install"],
            )
        }
    }
}

fn handle_accounts(cli: &Cli, command: &AccountsSubcommand, start: Instant) -> Result<()> {
    match command {
        AccountsSubcommand::List(args) => {
            let mut client = load_client()?;
            let mut params = Vec::new();
            if args.positions {
                params.push(("fields".to_string(), "positions".to_string()));
            }
            let payload = client.request_json("GET", ApiBase::Trader, "/accounts", params, None)?;
            merge_user_preferences(&mut client).ok();
            emit(
                cli,
                payload,
                start,
                vec!["schwab-cli accounts get --account <alias-or-last4> --positions"],
            )
        }
        AccountsSubcommand::Get(args) => {
            let mut client = load_client()?;
            let account = resolve_account(&mut client, &args.account)?;
            let mut params = Vec::new();
            if args.positions {
                params.push(("fields".to_string(), "positions".to_string()));
            }
            let path = format!("/accounts/{}", account.hash_value);
            let payload = client.request_json("GET", ApiBase::Trader, &path, params, None)?;
            emit(
                cli,
                payload,
                start,
                vec!["schwab-cli transactions list --account <account> --start <iso> --end <iso>"],
            )
        }
        AccountsSubcommand::Alias(args) => {
            let mut client = load_client()?;
            ensure_account_map(&mut client)?;
            let mut account_map = load_account_map()?;
            let resolved = resolve_from_map(&account_map, &args.account)?;
            for entry in &mut account_map.accounts {
                if entry.hash_value == resolved.hash_value {
                    entry.alias = Some(args.name.clone());
                } else if entry.alias.as_deref() == Some(&args.name) {
                    entry.alias = None;
                }
            }
            account_map.updated_at = Some(Utc::now());
            write_secret_json(&account_map_path(), &account_map)?;
            emit(
                cli,
                json!({"status": "ok", "alias": args.name, "account": public_account_entry(&resolved)}),
                start,
                vec!["schwab-cli accounts list --positions"],
            )
        }
    }
}

fn handle_cash(cli: &Cli, command: &CashSubcommand, start: Instant) -> Result<()> {
    match command {
        CashSubcommand::Status(args) => {
            let mut client = load_client()?;
            let account = resolve_account(&mut client, &args.account)?;
            let path = format!("/accounts/{}", account.hash_value);
            let payload = client.request_json("GET", ApiBase::Trader, &path, Vec::new(), None)?;
            let status = cash_status_payload(&account, &payload, args.raw)?;
            emit(
                cli,
                status,
                start,
                vec![
                    "schwab-cli orders preview --account <account> --json-file order.json",
                    "schwab-cli accounts get --account <account>",
                ],
            )
        }
    }
}

fn handle_transactions(cli: &Cli, command: &TransactionsSubcommand, start: Instant) -> Result<()> {
    match command {
        TransactionsSubcommand::List(args) => {
            let mut client = load_client()?;
            let account = resolve_account(&mut client, &args.account)?;
            let types = normalize_transaction_types(&args.types);
            let start_date = normalize_schwab_datetime(&args.start, false);
            let end_date = normalize_schwab_datetime(&args.end, true);
            let mut results = Vec::new();
            let mut errors = Vec::new();
            for tx_type in types {
                let mut params = vec![
                    ("startDate".to_string(), start_date.clone()),
                    ("endDate".to_string(), end_date.clone()),
                    ("types".to_string(), tx_type.clone()),
                ];
                if let Some(symbol) = &args.symbol {
                    params.push(("symbol".to_string(), symbol.clone()));
                }
                let path = format!("/accounts/{}/transactions", account.hash_value);
                match client.request_json("GET", ApiBase::Trader, &path, params, None) {
                    Ok(payload) => results.push(json!({"type": tx_type, "payload": payload})),
                    Err(err) => errors.push(json!({"type": tx_type, "error": err.to_string()})),
                }
            }
            emit(cli, json!({"account": public_account_entry(&account), "results": results, "errors": errors}), start, vec!["schwab-cli transactions list --account <account> --type TRADE --start <iso> --end <iso>"])
        }
        TransactionsSubcommand::Get(args) => {
            let mut client = load_client()?;
            let account = resolve_account(&mut client, &args.account)?;
            let path = format!(
                "/accounts/{}/transactions/{}",
                account.hash_value, args.transaction_id
            );
            let payload = client.request_json("GET", ApiBase::Trader, &path, Vec::new(), None)?;
            emit(cli, payload, start, Vec::new())
        }
    }
}

fn handle_orders(cli: &Cli, command: &OrdersSubcommand, start: Instant) -> Result<()> {
    match command {
        OrdersSubcommand::List(args) => {
            let mut client = load_client()?;
            let account = resolve_account(&mut client, &args.account)?;
            let path = format!("/accounts/{}/orders", account.hash_value);
            let mut params = vec![
                (
                    "fromEnteredTime".to_string(),
                    normalize_schwab_datetime(&args.from, false),
                ),
                (
                    "toEnteredTime".to_string(),
                    normalize_schwab_datetime(&args.to, true),
                ),
                ("maxResults".to_string(), args.max_results.to_string()),
            ];
            if let Some(status) = &args.status {
                params.push(("status".to_string(), status.clone()));
            }
            let payload = client.request_json("GET", ApiBase::Trader, &path, params, None)?;
            emit(
                cli,
                payload,
                start,
                vec!["schwab-cli orders preview --account <account> --json-file order.json"],
            )
        }
        OrdersSubcommand::All(args) => {
            let mut client = load_client()?;
            let mut params = vec![
                (
                    "fromEnteredTime".to_string(),
                    normalize_schwab_datetime(&args.from, false),
                ),
                (
                    "toEnteredTime".to_string(),
                    normalize_schwab_datetime(&args.to, true),
                ),
                ("maxResults".to_string(), args.max_results.to_string()),
            ];
            if let Some(status) = &args.status {
                params.push(("status".to_string(), status.clone()));
            }
            let payload = client.request_json("GET", ApiBase::Trader, "/orders", params, None)?;
            emit(cli, payload, start, Vec::new())
        }
        OrdersSubcommand::Get(args) => {
            let mut client = load_client()?;
            let account = resolve_account(&mut client, &args.account)?;
            let path = format!("/accounts/{}/orders/{}", account.hash_value, args.order_id);
            let payload = client.request_json("GET", ApiBase::Trader, &path, Vec::new(), None)?;
            emit(cli, payload, start, Vec::new())
        }
        OrdersSubcommand::Preview(args) => {
            let mut client = load_client()?;
            let account = resolve_account(&mut client, &args.account)?;
            let body = read_json_body(&args.json_file)?;
            let path = format!("/accounts/{}/previewOrder", account.hash_value);
            let payload =
                client.request_json("POST", ApiBase::Trader, &path, Vec::new(), Some(body))?;
            emit(cli, payload, start, vec!["SCHWAB_CLI_ALLOW_LIVE_TRADING=1 schwab-cli orders place --account <account> --json-file order.json --yes-live-order"])
        }
        OrdersSubcommand::Place(args) => {
            assert_live_order_allowed(args.yes_live_order)?;
            let mut client = load_client()?;
            let account = resolve_account(&mut client, &args.account)?;
            let body = read_json_body(&args.json_file)?;
            let path = format!("/accounts/{}/orders", account.hash_value);
            let payload =
                client.request_json("POST", ApiBase::Trader, &path, Vec::new(), Some(body))?;
            emit(
                cli,
                payload,
                start,
                vec!["schwab-cli orders list --account <account> --from <iso> --to <iso>"],
            )
        }
        OrdersSubcommand::Replace(args) => {
            assert_live_order_allowed(args.yes_live_order)?;
            let mut client = load_client()?;
            let account = resolve_account(&mut client, &args.account)?;
            let body = read_json_body(&args.json_file)?;
            let path = format!("/accounts/{}/orders/{}", account.hash_value, args.order_id);
            let payload =
                client.request_json("PUT", ApiBase::Trader, &path, Vec::new(), Some(body))?;
            emit(cli, payload, start, Vec::new())
        }
        OrdersSubcommand::Cancel(args) => {
            assert_live_order_allowed(args.yes_live_order)?;
            let mut client = load_client()?;
            let account = resolve_account(&mut client, &args.account)?;
            let path = format!("/accounts/{}/orders/{}", account.hash_value, args.order_id);
            let payload =
                client.request_json("DELETE", ApiBase::Trader, &path, Vec::new(), None)?;
            emit(cli, payload, start, Vec::new())
        }
    }
}

fn handle_market(cli: &Cli, command: &MarketSubcommand, start: Instant) -> Result<()> {
    let mut client = load_client()?;
    let (path, params) = match command {
        MarketSubcommand::Quotes(args) => {
            let mut params = vec![("symbols".to_string(), args.symbols.clone())];
            push_opt(&mut params, "fields", &args.fields);
            push_bool_opt(&mut params, "indicative", args.indicative);
            ("/quotes".to_string(), params)
        }
        MarketSubcommand::Quote(args) => {
            let mut params = Vec::new();
            push_opt(&mut params, "fields", &args.fields);
            (format!("/{}/quotes", args.symbol), params)
        }
        MarketSubcommand::Chains(args) => {
            let mut params = vec![("symbol".to_string(), args.symbol.clone())];
            push_opt(&mut params, "contractType", &args.contract_type);
            push_num_opt(&mut params, "strikeCount", args.strike_count);
            push_bool_opt(
                &mut params,
                "includeUnderlyingQuote",
                args.include_underlying_quote,
            );
            push_opt(&mut params, "strategy", &args.strategy);
            push_float_opt(&mut params, "interval", args.interval);
            push_float_opt(&mut params, "strike", args.strike);
            push_opt(&mut params, "range", &args.range);
            push_opt(&mut params, "fromDate", &args.from_date);
            push_opt(&mut params, "toDate", &args.to_date);
            push_float_opt(&mut params, "volatility", args.volatility);
            push_float_opt(&mut params, "underlyingPrice", args.underlying_price);
            push_float_opt(&mut params, "interestRate", args.interest_rate);
            push_num_opt(&mut params, "daysToExpiration", args.days_to_expiration);
            push_opt(&mut params, "expMonth", &args.exp_month);
            push_opt(&mut params, "optionType", &args.option_type);
            push_opt(&mut params, "entitlement", &args.entitlement);
            ("/chains".to_string(), params)
        }
        MarketSubcommand::ExpirationChain(args) => (
            "/expirationchain".to_string(),
            vec![("symbol".to_string(), args.symbol.clone())],
        ),
        MarketSubcommand::PriceHistory(args) => {
            let mut params = vec![("symbol".to_string(), args.symbol.clone())];
            push_opt(&mut params, "periodType", &args.period_type);
            push_num_opt(&mut params, "period", args.period);
            push_opt(&mut params, "frequencyType", &args.frequency_type);
            push_num_opt(&mut params, "frequency", args.frequency);
            push_opt(&mut params, "startDate", &args.start_date);
            push_opt(&mut params, "endDate", &args.end_date);
            push_bool_opt(
                &mut params,
                "needExtendedHoursData",
                args.need_extended_hours_data,
            );
            push_bool_opt(&mut params, "needPreviousClose", args.need_previous_close);
            ("/pricehistory".to_string(), params)
        }
        MarketSubcommand::Movers(args) => {
            let mut params = Vec::new();
            push_opt(&mut params, "sort", &args.sort);
            push_num_opt(&mut params, "frequency", args.frequency);
            (format!("/movers/{}", args.symbol), params)
        }
        MarketSubcommand::Hours(args) => {
            let mut params = Vec::new();
            push_opt(&mut params, "date", &args.date);
            if let Some(market) = &args.market {
                (format!("/markets/{market}"), params)
            } else if let Some(markets) = &args.markets {
                params.push(("markets".to_string(), markets.clone()));
                ("/markets".to_string(), params)
            } else {
                bail!("market hours requires --markets equity,option or --market equity. Next: schwab-cli market hours --markets equity,option");
            }
        }
        MarketSubcommand::InstrumentSearch(args) => (
            "/instruments".to_string(),
            vec![
                ("symbol".to_string(), args.symbol.clone()),
                ("projection".to_string(), args.projection.clone()),
            ],
        ),
        MarketSubcommand::Instrument(args) => (format!("/instruments/{}", args.cusip), Vec::new()),
    };
    let payload = client.request_json("GET", ApiBase::Market, &path, params, None)?;
    emit(cli, payload, start, Vec::new())
}

fn handle_streamer(cli: &Cli, command: &StreamerSubcommand, start: Instant) -> Result<()> {
    match command {
        StreamerSubcommand::Info => {
            let mut client = load_client()?;
            let payload =
                client.request_json("GET", ApiBase::Trader, "/userPreference", Vec::new(), None)?;
            let streamer = extract_streamer_info(&payload)?;
            emit(cli, json!({"streamerInfo": streamer, "raw_userPreference": payload}), start, vec!["schwab-cli streamer listen --service LEVELONE_EQUITIES --keys AAPL --fields 0,1,2,3 --jsonl"])
        }
        StreamerSubcommand::Fields(args) => emit(
            cli,
            streamer_field_maps_payload(args.service.as_deref())?,
            start,
            vec![
                "schwab-cli streamer fields --service LEVELONE_EQUITIES",
                "schwab-cli streamer listen --service LEVELONE_EQUITIES --keys AAPL --jsonl",
            ],
        ),
        StreamerSubcommand::Listen(args) => listen_streamer(cli, args, start),
    }
}

fn handle_generic_get(cli: &Cli, args: &GenericGetArgs, start: Instant) -> Result<()> {
    let mut client = load_client()?;
    let path = prepare_generic_path(&mut client, &args.path, args.account.as_deref())?;
    let payload = client.request_json(
        "GET",
        base_arg(args.base.clone()),
        &path,
        parse_params(&args.params)?,
        None,
    )?;
    emit(cli, payload, start, Vec::new())
}

fn handle_generic_write(
    cli: &Cli,
    method: &str,
    args: &GenericWriteArgs,
    start: Instant,
) -> Result<()> {
    let mut client = load_client()?;
    let path = prepare_generic_path(&mut client, &args.path, args.account.as_deref())?;
    if is_live_order_mutation(method, &path) {
        assert_live_order_allowed(args.yes_live_order)?;
    }
    let body = read_json_body(&args.body_file)?;
    let payload = client.request_json(
        method,
        base_arg(args.base.clone()),
        &path,
        parse_params(&args.params)?,
        Some(body),
    )?;
    emit(cli, payload, start, Vec::new())
}

fn handle_generic_delete(cli: &Cli, args: &GenericDeleteArgs, start: Instant) -> Result<()> {
    let mut client = load_client()?;
    let path = prepare_generic_path(&mut client, &args.path, args.account.as_deref())?;
    if is_live_order_mutation("DELETE", &path) {
        assert_live_order_allowed(args.yes_live_order)?;
    }
    let payload = client.request_json(
        "DELETE",
        base_arg(args.base.clone()),
        &path,
        parse_params(&args.params)?,
        None,
    )?;
    emit(cli, payload, start, Vec::new())
}

fn handle_docs(cli: &Cli, command: &DocsSubcommand, start: Instant) -> Result<()> {
    match command {
        DocsSubcommand::List => {
            let catalog = read_json(&endpoint_catalog_path())?;
            let endpoints = all_endpoints(&catalog);
            let trading_models = model_names(&trading_models_path())?;
            let market_models = model_names(&market_models_path())?;
            emit(
                cli,
                json!({"endpoints": endpoints, "trading_models": trading_models, "market_models": market_models}),
                start,
                vec![
                    "schwab-cli docs endpoint getAccount",
                    "schwab-cli docs model Order",
                ],
            )
        }
        DocsSubcommand::Endpoint(args) => {
            let catalog = read_json(&endpoint_catalog_path())?;
            let needle = args.value.to_lowercase();
            let matches: Vec<Value> = all_endpoints(&catalog)
                .into_iter()
                .filter(|ep| {
                    serde_json::to_string(ep)
                        .unwrap_or_default()
                        .to_lowercase()
                        .contains(&needle)
                })
                .collect();
            if matches.is_empty() {
                bail!(
                    "No Schwab endpoint matched '{}'. Next: schwab-cli docs list",
                    args.value
                );
            }
            emit(cli, json!({"matches": matches}), start, Vec::new())
        }
        DocsSubcommand::Model(args) => {
            let mut matches = find_models(&trading_models_path(), &args.value)?;
            matches.extend(find_models(&market_models_path(), &args.value)?);
            if matches.is_empty() {
                bail!(
                    "No Schwab model matched '{}'. Next: schwab-cli docs list",
                    args.value
                );
            }
            emit(cli, json!({"matches": matches}), start, Vec::new())
        }
        DocsSubcommand::Search(args) => {
            let paths = [
                oauth_doc_path(),
                trading_rest_doc_path(),
                market_rest_doc_path(),
                streamer_doc_path(),
                order_examples_doc_path(),
            ];
            let query = args.query.to_lowercase();
            let mut matches = Vec::new();
            for path in paths {
                let text = fs::read_to_string(&path).unwrap_or_default();
                for (idx, line) in text.lines().enumerate() {
                    if line.to_lowercase().contains(&query) {
                        matches.push(json!({"path": path, "line": idx + 1, "text": line}));
                        if matches.len() >= 100 {
                            break;
                        }
                    }
                }
            }
            emit(
                cli,
                json!({"query": args.query, "matches": matches}),
                start,
                Vec::new(),
            )
        }
    }
}

fn handle_snapshot(cli: &Cli, args: &SnapshotArgs, start: Instant) -> Result<()> {
    let mut client = load_client()?;
    let mut root = Map::new();
    root.insert(
        "snapshot_time_utc".to_string(),
        json!(Utc::now().to_rfc3339()),
    );
    root.insert("coverage".to_string(), schwab_coverage_payload());

    match sync_account_numbers(&mut client) {
        Ok(payload) => {
            root.insert("account_numbers".to_string(), payload);
        }
        Err(err) => {
            root.insert("account_numbers_error".to_string(), json!(err.to_string()));
        }
    }

    match client.request_json(
        "GET",
        ApiBase::Trader,
        "/accounts",
        vec![("fields".to_string(), "positions".to_string())],
        None,
    ) {
        Ok(payload) => {
            root.insert("accounts".to_string(), payload);
        }
        Err(err) => {
            root.insert("accounts_error".to_string(), json!(err.to_string()));
        }
    }

    match client.request_json("GET", ApiBase::Trader, "/userPreference", Vec::new(), None) {
        Ok(payload) => {
            root.insert("user_preference".to_string(), payload);
        }
        Err(err) => {
            root.insert("user_preference_error".to_string(), json!(err.to_string()));
        }
    }

    if let Some(symbols) = &args.quotes {
        match client.request_json(
            "GET",
            ApiBase::Market,
            "/quotes",
            vec![("symbols".to_string(), symbols.clone())],
            None,
        ) {
            Ok(payload) => {
                root.insert("quotes".to_string(), payload);
            }
            Err(err) => {
                root.insert("quotes_error".to_string(), json!(err.to_string()));
            }
        }
    }

    let start_date = args
        .start
        .clone()
        .map(|value| normalize_schwab_datetime(&value, false))
        .unwrap_or_else(|| format_schwab_datetime(Utc::now() - Duration::days(30)));
    let end_date = args
        .end
        .clone()
        .map(|value| normalize_schwab_datetime(&value, true))
        .unwrap_or_else(|| format_schwab_datetime(Utc::now()));

    if args.include_orders {
        match client.request_json(
            "GET",
            ApiBase::Trader,
            "/orders",
            vec![
                ("fromEnteredTime".to_string(), start_date.clone()),
                ("toEnteredTime".to_string(), end_date.clone()),
                ("maxResults".to_string(), "3000".to_string()),
            ],
            None,
        ) {
            Ok(payload) => {
                root.insert("orders".to_string(), payload);
            }
            Err(err) => {
                root.insert("orders_error".to_string(), json!(err.to_string()));
            }
        }
    }

    if args.include_transactions {
        let mut tx_root = Map::new();
        if let Ok(account_map) = load_account_map() {
            for account in account_map.accounts {
                let mut account_tx = Vec::new();
                for tx_type in TRANSACTION_TYPES {
                    let path = format!("/accounts/{}/transactions", account.hash_value);
                    let params = vec![
                        ("startDate".to_string(), start_date.clone()),
                        ("endDate".to_string(), end_date.clone()),
                        ("types".to_string(), (*tx_type).to_string()),
                    ];
                    match client.request_json("GET", ApiBase::Trader, &path, params, None) {
                        Ok(payload) => {
                            account_tx.push(json!({"type": tx_type, "payload": payload}))
                        }
                        Err(err) => {
                            account_tx.push(json!({"type": tx_type, "error": err.to_string()}))
                        }
                    }
                }
                tx_root.insert(account_key(&account), Value::Array(account_tx));
            }
        }
        root.insert("transactions".to_string(), Value::Object(tx_root));
    }

    let snapshot = Value::Object(root);
    let latest_path = args
        .output
        .clone()
        .unwrap_or_else(|| schwab_data_dir().join("latest_snapshot.json"));
    let dated_path = schwab_data_dir().join(format!(
        "snapshot_{}.json",
        Utc::now().format("%Y%m%dT%H%M%SZ")
    ));
    write_json(&latest_path, &snapshot)?;
    write_json(&dated_path, &snapshot)?;
    emit(
        cli,
        json!({"status": "ok", "latest_snapshot": latest_path, "dated_snapshot": dated_path}),
        start,
        vec!["cat ~/.local/share/schwab-cli/latest_snapshot.json"],
    )
}

impl SchwabClient {
    fn request_json(
        &mut self,
        method: &str,
        base: ApiBase,
        path: &str,
        params: Vec<(String, String)>,
        body: Option<Value>,
    ) -> Result<Value> {
        self.refresh_if_needed()?;
        let url = format!("{}{}", base_url(base), path);
        let mut request = match method {
            "GET" => self.http.get(&url),
            "POST" => self.http.post(&url),
            "PUT" => self.http.put(&url),
            "DELETE" => self.http.delete(&url),
            other => bail!("Unsupported method {other}"),
        };
        request = request
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.token.access_token));
        if !params.is_empty() {
            request = request.query(&params);
        }
        if let Some(body) = body {
            request = request.header(CONTENT_TYPE, "application/json").json(&body);
        }
        let response = request
            .send()
            .with_context(|| format!("Schwab {method} {path} request failed"))?;
        let status = response.status();
        let mut headers = Map::new();
        for name in [
            "Schwab-Client-CorrelId",
            "Schwab-Client-CorrelID",
            "Location",
            "Schwab-Resource-Version",
        ] {
            if let Some(value) = response.headers().get(name) {
                headers.insert(name.to_string(), json!(value.to_str().unwrap_or_default()));
            }
        }
        let text = response_body_text(response)?;
        if !status.is_success() {
            bail!(
                "Schwab {method} {path} failed with {}: {}\nNext: schwab-cli auth status",
                status.as_u16(),
                summarize_error_body(&text)
            );
        }
        if text.trim().is_empty() {
            return Ok(json!({"status": "ok", "http_status": status.as_u16(), "headers": headers}));
        }
        serde_json::from_str(&text)
            .or_else(|_| Ok(json!({"status": "ok", "http_status": status.as_u16(), "raw": text, "headers": headers})))
    }

    fn refresh_if_needed(&mut self) -> Result<()> {
        let now = Utc::now();
        ensure_refresh_token_valid_at(&self.token, now)?;
        if !should_refresh_token_at(&self.token, now) {
            return Ok(());
        }
        self.token = refresh_access_token(&self.app, &self.token)?;
        write_secret_json(&token_path(), &self.token)?;
        Ok(())
    }
}

fn load_client() -> Result<SchwabClient> {
    Ok(SchwabClient {
        http: Client::builder().build()?,
        app: load_app_config()?,
        token: load_token().context("Schwab auth token missing. Next: schwab-cli auth login")?,
    })
}

fn exchange_code(app: &AppConfig, code: &str) -> Result<TokenStore> {
    let http = Client::builder().build()?;
    let response = http
        .post(OAUTH_TOKEN_URL)
        .header(AUTHORIZATION, basic_auth_header(app))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", app.callback_url.as_str()),
        ])
        .send()
        .context("Schwab token exchange request failed")?;
    token_store_from_response(response)
}

fn refresh_access_token(app: &AppConfig, token: &TokenStore) -> Result<TokenStore> {
    let http = Client::builder().build()?;
    let response = http
        .post(OAUTH_TOKEN_URL)
        .header(AUTHORIZATION, basic_auth_header(app))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", token.refresh_token.as_str()),
        ])
        .send()
        .context("Schwab token refresh request failed")?;
    token_store_from_response(response)
}

fn should_refresh_token_at(token: &TokenStore, now: DateTime<Utc>) -> bool {
    token.access_expires_at - Duration::seconds(ACCESS_REFRESH_SKEW_SECONDS) <= now
        || token.refresh_expires_at - Duration::seconds(REFRESH_TOKEN_RENEWAL_SKEW_SECONDS) <= now
}

fn ensure_refresh_token_valid(token: &TokenStore) -> Result<()> {
    ensure_refresh_token_valid_at(token, Utc::now())
}

fn ensure_refresh_token_valid_at(token: &TokenStore, now: DateTime<Utc>) -> Result<()> {
    if token.refresh_expires_at <= now {
        bail!(
            "Schwab refresh token expired at {}. Browser login required. Next: schwab-cli auth login",
            token.refresh_expires_at
        );
    }
    Ok(())
}

fn token_store_from_response(response: reqwest::blocking::Response) -> Result<TokenStore> {
    let status = response.status();
    let text = response_body_text(response)?;
    if !status.is_success() {
        bail!(
            "Schwab OAuth token endpoint failed with {}: {}\nNext: schwab-cli auth login",
            status.as_u16(),
            summarize_error_body(&text)
        );
    }
    let parsed: TokenResponse =
        serde_json::from_str(&text).context("Failed to parse Schwab OAuth token response")?;
    let now = Utc::now();
    Ok(TokenStore {
        access_token: parsed.access_token,
        refresh_token: parsed.refresh_token,
        token_type: parsed.token_type.unwrap_or_else(|| "Bearer".to_string()),
        scope: parsed.scope,
        id_token: parsed.id_token,
        access_expires_at: now + Duration::seconds(parsed.expires_in),
        refresh_expires_at: now + Duration::days(REFRESH_TOKEN_LIFETIME_DAYS),
        created_at: now,
        updated_at: now,
    })
}

fn response_body_text(response: reqwest::blocking::Response) -> Result<String> {
    let encoding = response
        .headers()
        .get(CONTENT_ENCODING)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let bytes = response
        .bytes()
        .context("Failed to read Schwab response body")?;
    Ok(decode_response_bytes(bytes.as_ref(), &encoding))
}

fn decode_response_bytes(bytes: &[u8], encoding: &str) -> String {
    let decoded = if encoding.contains("gzip") || bytes.starts_with(&[0x1f, 0x8b]) {
        let mut decoder = GzDecoder::new(bytes);
        let mut text = String::new();
        match decoder.read_to_string(&mut text) {
            Ok(_) => return text,
            Err(_) => None,
        }
    } else {
        None
    };
    decoded.unwrap_or_else(|| String::from_utf8_lossy(bytes).to_string())
}

fn summarize_error_body(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return "<empty response body>".to_string();
    }
    if let Ok(value) = serde_json::from_str::<Value>(trimmed) {
        if let Some(message) = token_error_message(&value) {
            return message;
        }
        return value.to_string();
    }
    if trimmed.len() > 2_000 {
        let preview: String = trimmed.chars().take(2_000).collect();
        format!("{preview}... <truncated {} bytes>", trimmed.len())
    } else {
        trimmed.to_string()
    }
}

fn token_error_message(value: &Value) -> Option<String> {
    let object = value.as_object()?;
    let mut parts = Vec::new();
    for key in [
        "error",
        "error_description",
        "message",
        "errorMessage",
        "errorCode",
    ] {
        if let Some(value) = object.get(key).and_then(Value::as_str) {
            if !value.trim().is_empty() {
                parts.push(format!("{key}={value}"));
            }
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("; "))
    }
}

fn sync_account_numbers(client: &mut SchwabClient) -> Result<Value> {
    let payload = client.request_json(
        "GET",
        ApiBase::Trader,
        "/accounts/accountNumbers",
        Vec::new(),
        None,
    )?;
    let mut account_map = parse_account_numbers(&payload)?;
    preserve_aliases(&mut account_map);
    account_map.updated_at = Some(Utc::now());
    write_secret_json(&account_map_path(), &account_map)?;
    merge_user_preferences(client).ok();
    Ok(json!({
        "status": "ok",
        "coverage": schwab_coverage_payload(),
        "account_count": account_map.accounts.len(),
        "accounts": public_account_entries(&account_map.accounts),
        "raw": payload
    }))
}

fn merge_user_preferences(client: &mut SchwabClient) -> Result<()> {
    let prefs = client.request_json("GET", ApiBase::Trader, "/userPreference", Vec::new(), None)?;
    let mut account_map = load_account_map().unwrap_or_default();
    let pref_accounts = prefs
        .get("accounts")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    for pref in pref_accounts {
        let pref_account = pref
            .get("accountNumber")
            .and_then(Value::as_str)
            .unwrap_or_default();
        for entry in &mut account_map.accounts {
            if entry.account_number == pref_account
                || entry.last4 == last4(pref_account)
                || pref
                    .get("displayAcctId")
                    .and_then(Value::as_str)
                    .map(last4)
                    .as_deref()
                    == Some(entry.last4.as_str())
            {
                entry.nick_name = pref
                    .get("nickName")
                    .and_then(Value::as_str)
                    .map(ToString::to_string);
                entry.display_acct_id = pref
                    .get("displayAcctId")
                    .and_then(Value::as_str)
                    .map(ToString::to_string);
                entry.account_type = pref
                    .get("type")
                    .and_then(Value::as_str)
                    .map(ToString::to_string);
                if entry.alias.is_none() {
                    entry.alias = entry.nick_name.clone().map(safe_alias);
                }
            }
        }
    }
    account_map.updated_at = Some(Utc::now());
    write_secret_json(&account_map_path(), &account_map)?;
    Ok(())
}

fn parse_account_numbers(payload: &Value) -> Result<AccountMap> {
    let array = payload
        .as_array()
        .cloned()
        .or_else(|| {
            payload
                .get("accountNumbers")
                .and_then(Value::as_array)
                .cloned()
        })
        .or_else(|| payload.get("accounts").and_then(Value::as_array).cloned())
        .ok_or_else(|| anyhow!("Could not parse accountNumbers response as an array"))?;
    let mut accounts = Vec::new();
    for item in array {
        let account_number = item
            .get("accountNumber")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let hash_value = item
            .get("hashValue")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        if account_number.is_empty() || hash_value.is_empty() {
            continue;
        }
        accounts.push(AccountEntry {
            last4: last4(&account_number),
            account_number,
            hash_value,
            alias: None,
            nick_name: None,
            display_acct_id: None,
            account_type: None,
        });
    }
    if accounts.is_empty() {
        bail!("Schwab returned no account number/hash pairs. Confirm the OAuth consent selected accounts.");
    }
    Ok(AccountMap {
        accounts,
        updated_at: Some(Utc::now()),
    })
}

fn preserve_aliases(new_map: &mut AccountMap) {
    if let Ok(old_map) = load_account_map() {
        for new_entry in &mut new_map.accounts {
            if let Some(old_entry) = old_map.accounts.iter().find(|old| {
                old.hash_value == new_entry.hash_value
                    || old.account_number == new_entry.account_number
            }) {
                new_entry.alias = old_entry.alias.clone();
                new_entry.nick_name = old_entry.nick_name.clone();
                new_entry.display_acct_id = old_entry.display_acct_id.clone();
                new_entry.account_type = old_entry.account_type.clone();
            }
        }
    }
}

fn ensure_account_map(client: &mut SchwabClient) -> Result<()> {
    match load_account_map() {
        Ok(map) if !map.accounts.is_empty() => Ok(()),
        _ => sync_account_numbers(client).map(|_| ()),
    }
}

fn resolve_account(client: &mut SchwabClient, account: &str) -> Result<AccountEntry> {
    ensure_account_map(client)?;
    let map = load_account_map()?;
    resolve_from_map(&map, account)
}

fn resolve_from_map(map: &AccountMap, account: &str) -> Result<AccountEntry> {
    let needle = account.trim();
    let matches: Vec<AccountEntry> = map
        .accounts
        .iter()
        .filter(|entry| {
            entry.hash_value == needle
                || entry.account_number == needle
                || entry.last4 == needle.trim_start_matches("...")
                || entry.alias.as_deref() == Some(needle)
                || entry.nick_name.as_deref() == Some(needle)
                || entry.display_acct_id.as_deref() == Some(needle)
        })
        .cloned()
        .collect();
    match matches.len() {
        0 => bail!("account '{needle}' not found. Next: schwab-cli account-numbers"),
        1 => Ok(matches[0].clone()),
        _ => {
            let descriptions: Vec<String> = matches
                .iter()
                .map(|entry| format!("{} (...{})", account_label(entry), entry.last4))
                .collect();
            bail!("account '{needle}' is ambiguous. Matches: {}. Next: schwab-cli accounts alias --account <last4> --name <name>", descriptions.join(", "))
        }
    }
}

fn listen_streamer(cli: &Cli, args: &StreamerListenArgs, start: Instant) -> Result<()> {
    let mut client = load_client()?;
    client.refresh_if_needed()?;
    let prefs = client.request_json("GET", ApiBase::Trader, "/userPreference", Vec::new(), None)?;
    let streamer = extract_streamer_info(&prefs)?;
    let socket_url = streamer
        .get("streamerSocketUrl")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("streamerInfo missing streamerSocketUrl"))?;
    let customer_id = streamer
        .get("schwabClientCustomerId")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("streamerInfo missing schwabClientCustomerId"))?;
    let correl_id = streamer
        .get("schwabClientCorrelId")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("streamerInfo missing schwabClientCorrelId"))?;
    let channel = streamer
        .get("schwabClientChannel")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("streamerInfo missing schwabClientChannel"))?;
    let function_id = streamer
        .get("schwabClientFunctionId")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("streamerInfo missing schwabClientFunctionId"))?;

    let (mut socket, _) = connect(socket_url)
        .with_context(|| format!("Failed to connect to Schwab streamer {socket_url}"))?;
    let login = json!({"requests": [{
        "service": "ADMIN",
        "command": "LOGIN",
        "requestid": "1",
        "SchwabClientCustomerId": customer_id,
        "SchwabClientCorrelId": correl_id,
        "parameters": {
            "Authorization": client.token.access_token,
            "SchwabClientChannel": channel,
            "SchwabClientFunctionId": function_id
        }
    }]});
    socket.send(Message::Text(login.to_string().into()))?;
    let login_response = read_ws_json(&mut socket)?;
    print_stream(cli, &login_response, args.jsonl)?;

    if args.keys.is_some() || args.service == "ACCT_ACTIVITY" {
        let mut parameters = Map::new();
        if let Some(keys) = streamer_keys(args) {
            parameters.insert("keys".to_string(), json!(keys));
        }
        if let Some(fields) = streamer_fields(args) {
            parameters.insert("fields".to_string(), json!(fields));
        }
        let sub = json!({"requests": [{
            "service": args.service,
            "command": "SUBS",
            "requestid": "2",
            "SchwabClientCustomerId": customer_id,
            "SchwabClientCorrelId": correl_id,
            "parameters": parameters
        }]});
        socket.send(Message::Text(sub.to_string().into()))?;
    }

    let mut count = 0;
    loop {
        if args.limit > 0 && count >= args.limit {
            break;
        }
        let value = read_ws_json(&mut socket)?;
        let value = if args.raw {
            value
        } else {
            decode_streamer_message(value)
        };
        print_stream(cli, &value, args.jsonl)?;
        count += 1;
    }
    if !args.jsonl && !cli.agent {
        eprintln!(
            "[info] Schwab streamer read {count} messages in {}ms",
            start.elapsed().as_millis()
        );
    }
    Ok(())
}

fn read_ws_json(
    socket: &mut tungstenite::WebSocket<MaybeTlsStream<std::net::TcpStream>>,
) -> Result<Value> {
    loop {
        match socket.read()? {
            Message::Text(text) => {
                return serde_json::from_str(&text)
                    .or_else(|_| Ok(json!({"raw": text.to_string()})))
            }
            Message::Binary(bytes) => return Ok(json!({"binary_bytes": bytes.len()})),
            Message::Ping(_) | Message::Pong(_) => continue,
            Message::Close(frame) => bail!("Schwab streamer closed: {:?}", frame),
            _ => continue,
        }
    }
}

fn print_stream(cli: &Cli, value: &Value, jsonl: bool) -> Result<()> {
    if cli.agent || !jsonl {
        println!(
            "{}",
            if cli.compact {
                serde_json::to_string(value)?
            } else {
                serde_json::to_string_pretty(value)?
            }
        );
    } else {
        println!("{}", serde_json::to_string(value)?);
    }
    Ok(())
}

fn streamer_keys(args: &StreamerListenArgs) -> Option<String> {
    args.keys.clone().or_else(|| {
        if args.service.eq_ignore_ascii_case("ACCT_ACTIVITY") {
            Some("Account Activity".to_string())
        } else {
            None
        }
    })
}

fn streamer_fields(args: &StreamerListenArgs) -> Option<String> {
    args.fields.clone().or_else(|| {
        streamer_field_map(&args.service).map(|fields| {
            fields
                .iter()
                .map(|(field, _)| *field)
                .collect::<Vec<_>>()
                .join(",")
        })
    })
}

fn decode_streamer_message(value: Value) -> Value {
    let mut root = value;
    if let Some(data) = root.get_mut("data").and_then(Value::as_array_mut) {
        for item in data {
            let service = item
                .get("service")
                .and_then(Value::as_str)
                .map(ToString::to_string);
            if let (Some(service), Some(content)) = (
                service,
                item.get_mut("content").and_then(Value::as_array_mut),
            ) {
                for content_item in content {
                    decode_streamer_content(&service, content_item);
                }
            }
        }
    }
    root
}

fn decode_streamer_content(service: &str, value: &mut Value) {
    let Some(map) = value.as_object_mut() else {
        return;
    };
    let Some(field_map) = streamer_field_map(service) else {
        return;
    };
    let numeric_keys: Vec<String> = map
        .keys()
        .filter(|key| key.chars().all(|ch| ch.is_ascii_digit()))
        .cloned()
        .collect();
    let mut raw_fields = Map::new();
    let mut decoded_fields = Map::new();
    for key in numeric_keys {
        if let Some(value) = map.remove(&key) {
            raw_fields.insert(key.clone(), value.clone());
            if let Some((_, label)) = field_map.iter().find(|(field, _)| *field == key) {
                decoded_fields.insert(snake_key(label), value);
            } else {
                decoded_fields.insert(format!("field_{key}"), value);
            }
        }
    }
    for (key, value) in decoded_fields {
        map.insert(key, value);
    }
    if !raw_fields.is_empty() {
        map.insert("_raw_fields".to_string(), Value::Object(raw_fields));
    }

    if is_book_service(service) {
        decode_book_side(map, "bid_side_levels");
        decode_book_side(map, "ask_side_levels");
    }
    if service.eq_ignore_ascii_case("ACCT_ACTIVITY") {
        if let Some(Value::String(message_data)) = map.get("message_data") {
            if let Ok(parsed) = serde_json::from_str::<Value>(message_data) {
                map.insert("message_data_json".to_string(), parsed);
            }
        }
    }
}

fn decode_book_side(map: &mut Map<String, Value>, side_key: &str) {
    let Some(levels) = map.get_mut(side_key).and_then(Value::as_array_mut) else {
        return;
    };
    for level in levels {
        decode_numeric_object(level, BOOK_PRICE_LEVEL_FIELDS);
        if let Some(makers) = level.get_mut("market_makers").and_then(Value::as_array_mut) {
            for maker in makers {
                decode_numeric_object(maker, BOOK_MARKET_MAKER_FIELDS);
            }
        }
    }
}

fn decode_numeric_object(value: &mut Value, fields: &[(&str, &str)]) {
    if let Some(items) = value.as_array() {
        let mut decoded = Map::new();
        for (idx, item) in items.iter().enumerate() {
            let key = idx.to_string();
            if let Some((_, label)) = fields.iter().find(|(field, _)| *field == key) {
                decoded.insert(snake_key(label), item.clone());
            } else {
                decoded.insert(format!("field_{key}"), item.clone());
            }
        }
        *value = Value::Object(decoded);
        return;
    }

    let Some(map) = value.as_object_mut() else {
        return;
    };
    let numeric_keys: Vec<String> = map
        .keys()
        .filter(|key| key.chars().all(|ch| ch.is_ascii_digit()))
        .cloned()
        .collect();
    for key in numeric_keys {
        if let Some(value) = map.remove(&key) {
            if let Some((_, label)) = fields.iter().find(|(field, _)| *field == key) {
                map.insert(snake_key(label), value);
            } else {
                map.insert(format!("field_{key}"), value);
            }
        }
    }
}

fn streamer_field_maps_payload(service: Option<&str>) -> Result<Value> {
    let services = streamer_services();
    let mut payload = Map::new();
    for service_name in &services {
        if let Some(requested) = service {
            if !service_name.eq_ignore_ascii_case(requested) {
                continue;
            }
        }
        let fields = streamer_field_map(service_name)
            .unwrap_or(&[])
            .iter()
            .map(|(field, label)| {
                json!({
                    "field": field,
                    "name": label,
                    "json_key": snake_key(label),
                })
            })
            .collect::<Vec<_>>();
        payload.insert(service_name.to_string(), Value::Array(fields));
    }
    if payload.is_empty() {
        bail!(
            "No streamer field map for '{}'. Available: {}",
            service.unwrap_or(""),
            services.join(", ")
        );
    }
    Ok(Value::Object(payload))
}

fn streamer_services() -> Vec<&'static str> {
    vec![
        "LEVELONE_EQUITIES",
        "LEVELONE_OPTIONS",
        "LEVELONE_FUTURES",
        "LEVELONE_FUTURES_OPTIONS",
        "LEVELONE_FOREX",
        "NYSE_BOOK",
        "NASDAQ_BOOK",
        "OPTIONS_BOOK",
        "CHART_EQUITY",
        "CHART_FUTURES",
        "SCREENER_EQUITY",
        "SCREENER_OPTION",
        "ACCT_ACTIVITY",
    ]
}

fn streamer_field_map(service: &str) -> Option<&'static [(&'static str, &'static str)]> {
    match service.to_ascii_uppercase().as_str() {
        "LEVELONE_EQUITIES" => Some(LEVELONE_EQUITIES_FIELDS),
        "LEVELONE_OPTIONS" => Some(LEVELONE_OPTIONS_FIELDS),
        "LEVELONE_FUTURES" => Some(LEVELONE_FUTURES_FIELDS),
        "LEVELONE_FUTURES_OPTIONS" => Some(LEVELONE_FUTURES_OPTIONS_FIELDS),
        "LEVELONE_FOREX" => Some(LEVELONE_FOREX_FIELDS),
        "NYSE_BOOK" | "NASDAQ_BOOK" | "OPTIONS_BOOK" => Some(BOOK_FIELDS),
        "CHART_EQUITY" => Some(CHART_EQUITY_FIELDS),
        "CHART_FUTURES" => Some(CHART_FUTURES_FIELDS),
        "SCREENER_EQUITY" | "SCREENER_OPTION" => Some(SCREENER_FIELDS),
        "ACCT_ACTIVITY" => Some(ACCT_ACTIVITY_FIELDS),
        _ => None,
    }
}

fn is_book_service(service: &str) -> bool {
    matches!(
        service.to_ascii_uppercase().as_str(),
        "NYSE_BOOK" | "NASDAQ_BOOK" | "OPTIONS_BOOK"
    )
}

fn snake_key(label: &str) -> String {
    let mut out = String::new();
    let mut previous_was_sep = true;
    for ch in label.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase()
                && !previous_was_sep
                && out.chars().last().is_some_and(|c| c.is_ascii_lowercase())
            {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
            previous_was_sep = false;
        } else if !previous_was_sep {
            out.push('_');
            previous_was_sep = true;
        }
    }
    let trimmed = out.trim_matches('_').to_string();
    if trimmed.chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
        format!("field_{trimmed}")
    } else if trimmed.is_empty() {
        "field".to_string()
    } else {
        trimmed
    }
}

const LEVELONE_EQUITIES_FIELDS: &[(&str, &str)] = &[
    ("0", "Symbol"),
    ("1", "Bid Price"),
    ("2", "Ask Price"),
    ("3", "Last Price"),
    ("4", "Bid Size"),
    ("5", "Ask Size"),
    ("6", "Ask ID"),
    ("7", "Bid ID"),
    ("8", "Total Volume"),
    ("9", "Last Size"),
    ("10", "High Price"),
    ("11", "Low Price"),
    ("12", "Close Price"),
    ("13", "Exchange ID"),
    ("14", "Marginable"),
    ("15", "Description"),
    ("16", "Last ID"),
    ("17", "Open Price"),
    ("18", "Net Change"),
    ("19", "52 Week High"),
    ("20", "52 Week Low"),
    ("21", "PE Ratio"),
    ("22", "Annual Dividend Amount"),
    ("23", "Dividend Yield"),
    ("24", "NAV"),
    ("25", "Exchange Name"),
    ("26", "Dividend Date"),
    ("27", "Regular Market Quote"),
    ("28", "Regular Market Trade"),
    ("29", "Regular Market Last Price"),
    ("30", "Regular Market Last Size"),
    ("31", "Regular Market Net Change"),
    ("32", "Security Status"),
    ("33", "Mark Price"),
    ("34", "Quote Time"),
    ("35", "Trade Time"),
    ("36", "Regular Market Trade Time"),
    ("37", "Bid Time"),
    ("38", "Ask Time"),
    ("39", "Ask MIC ID"),
    ("40", "Bid MIC ID"),
    ("41", "Last MIC ID"),
    ("42", "Net Percent Change"),
    ("43", "Regular Market Percent Change"),
    ("44", "Mark Price Net Change"),
    ("45", "Mark Price Percent Change"),
    ("46", "Hard To Borrow Quantity"),
    ("47", "Hard To Borrow Rate"),
    ("48", "Hard To Borrow"),
    ("49", "Shortable"),
    ("50", "Post Market Net Change"),
    ("51", "Post Market Percent Change"),
];

const LEVELONE_OPTIONS_FIELDS: &[(&str, &str)] = &[
    ("0", "Symbol"),
    ("1", "Description"),
    ("2", "Bid Price"),
    ("3", "Ask Price"),
    ("4", "Last Price"),
    ("5", "High Price"),
    ("6", "Low Price"),
    ("7", "Close Price"),
    ("8", "Total Volume"),
    ("9", "Open Interest"),
    ("10", "Volatility"),
    ("11", "Money Intrinsic Value"),
    ("12", "Expiration Year"),
    ("13", "Multiplier"),
    ("14", "Digits"),
    ("15", "Open Price"),
    ("16", "Bid Size"),
    ("17", "Ask Size"),
    ("18", "Last Size"),
    ("19", "Net Change"),
    ("20", "Strike Price"),
    ("21", "Contract Type"),
    ("22", "Underlying"),
    ("23", "Expiration Month"),
    ("24", "Deliverables"),
    ("25", "Time Value"),
    ("26", "Expiration Day"),
    ("27", "Days To Expiration"),
    ("28", "Delta"),
    ("29", "Gamma"),
    ("30", "Theta"),
    ("31", "Vega"),
    ("32", "Rho"),
    ("33", "Security Status"),
    ("34", "Theoretical Option Value"),
    ("35", "Underlying Price"),
    ("36", "UV Expiration Type"),
    ("37", "Mark Price"),
    ("38", "Quote Time"),
    ("39", "Trade Time"),
    ("40", "Exchange"),
    ("41", "Exchange Name"),
    ("42", "Last Trading Day"),
    ("43", "Settlement Type"),
    ("44", "Net Percent Change"),
    ("45", "Mark Price Net Change"),
    ("46", "Mark Price Percent Change"),
    ("47", "Implied Yield"),
    ("48", "Is Penny Pilot"),
    ("49", "Option Root"),
    ("50", "52 Week High"),
    ("51", "52 Week Low"),
    ("52", "Indicative Ask Price"),
    ("53", "Indicative Bid Price"),
    ("54", "Indicative Quote Time"),
    ("55", "Exercise Type"),
];

const LEVELONE_FUTURES_FIELDS: &[(&str, &str)] = &[
    ("0", "Symbol"),
    ("1", "Bid Price"),
    ("2", "Ask Price"),
    ("3", "Last Price"),
    ("4", "Bid Size"),
    ("5", "Ask Size"),
    ("6", "Bid ID"),
    ("7", "Ask ID"),
    ("8", "Total Volume"),
    ("9", "Last Size"),
    ("10", "Quote Time"),
    ("11", "Trade Time"),
    ("12", "High Price"),
    ("13", "Low Price"),
    ("14", "Close Price"),
    ("15", "Exchange ID"),
    ("16", "Description"),
    ("17", "Last ID"),
    ("18", "Open Price"),
    ("19", "Net Change"),
    ("20", "Future Percent Change"),
    ("21", "Exchange Name"),
    ("22", "Security Status"),
    ("23", "Open Interest"),
    ("24", "Mark"),
    ("25", "Tick"),
    ("26", "Tick Amount"),
    ("27", "Product"),
    ("28", "Future Price Format"),
    ("29", "Future Trading Hours"),
    ("30", "Future Is Tradable"),
    ("31", "Future Multiplier"),
    ("32", "Future Is Active"),
    ("33", "Future Settlement Price"),
    ("34", "Future Active Symbol"),
    ("35", "Future Expiration Date"),
    ("36", "Expiration Style"),
    ("37", "Ask Time"),
    ("38", "Bid Time"),
    ("39", "Quoted In Session"),
    ("40", "Settlement Date"),
];

const LEVELONE_FUTURES_OPTIONS_FIELDS: &[(&str, &str)] = &[
    ("0", "Symbol"),
    ("1", "Bid Price"),
    ("2", "Ask Price"),
    ("3", "Last Price"),
    ("4", "Bid Size"),
    ("5", "Ask Size"),
    ("6", "Bid ID"),
    ("7", "Ask ID"),
    ("8", "Total Volume"),
    ("9", "Last Size"),
    ("10", "Quote Time"),
    ("11", "Trade Time"),
    ("12", "High Price"),
    ("13", "Low Price"),
    ("14", "Close Price"),
    ("15", "Last ID"),
    ("16", "Description"),
    ("17", "Open Price"),
    ("18", "Open Interest"),
    ("19", "Mark"),
    ("20", "Tick"),
    ("21", "Tick Amount"),
    ("22", "Future Multiplier"),
    ("23", "Future Settlement Price"),
    ("24", "Underlying Symbol"),
    ("25", "Strike Price"),
    ("26", "Future Expiration Date"),
    ("27", "Expiration Style"),
    ("28", "Contract Type"),
    ("29", "Security Status"),
    ("30", "Exchange"),
    ("31", "Exchange Name"),
];

const LEVELONE_FOREX_FIELDS: &[(&str, &str)] = &[
    ("0", "Symbol"),
    ("1", "Bid Price"),
    ("2", "Ask Price"),
    ("3", "Last Price"),
    ("4", "Bid Size"),
    ("5", "Ask Size"),
    ("6", "Total Volume"),
    ("7", "Last Size"),
    ("8", "Quote Time"),
    ("9", "Trade Time"),
    ("10", "High Price"),
    ("11", "Low Price"),
    ("12", "Close Price"),
    ("13", "Exchange"),
    ("14", "Description"),
    ("15", "Open Price"),
    ("16", "Net Change"),
    ("17", "Percent Change"),
    ("18", "Exchange Name"),
    ("19", "Digits"),
    ("20", "Security Status"),
    ("21", "Tick"),
    ("22", "Tick Amount"),
    ("23", "Product"),
    ("24", "Trading Hours"),
    ("25", "Is Tradable"),
    ("26", "Market Maker"),
    ("27", "52 Week High"),
    ("28", "52 Week Low"),
    ("29", "Mark"),
];

const BOOK_FIELDS: &[(&str, &str)] = &[
    ("0", "Symbol"),
    ("1", "Market Snapshot Time"),
    ("2", "Bid Side Levels"),
    ("3", "Ask Side Levels"),
];

const BOOK_PRICE_LEVEL_FIELDS: &[(&str, &str)] = &[
    ("0", "Price"),
    ("1", "Aggregate Size"),
    ("2", "Market Maker Count"),
    ("3", "Market Makers"),
];

const BOOK_MARKET_MAKER_FIELDS: &[(&str, &str)] =
    &[("0", "Market Maker ID"), ("1", "Size"), ("2", "Quote Time")];

const CHART_EQUITY_FIELDS: &[(&str, &str)] = &[
    ("0", "Key"),
    ("1", "Open Price"),
    ("2", "High Price"),
    ("3", "Low Price"),
    ("4", "Close Price"),
    ("5", "Volume"),
    ("6", "Sequence"),
    ("7", "Chart Time"),
    ("8", "Chart Day"),
];

const CHART_FUTURES_FIELDS: &[(&str, &str)] = &[
    ("0", "Key"),
    ("1", "Chart Time"),
    ("2", "Open Price"),
    ("3", "High Price"),
    ("4", "Low Price"),
    ("5", "Close Price"),
    ("6", "Volume"),
];

const SCREENER_FIELDS: &[(&str, &str)] = &[
    ("0", "Symbol"),
    ("1", "Timestamp"),
    ("2", "Sort Field"),
    ("3", "Frequency"),
    ("4", "Items"),
];

const ACCT_ACTIVITY_FIELDS: &[(&str, &str)] = &[
    ("0", "Subscription Key"),
    ("1", "Account"),
    ("2", "Message Type"),
    ("3", "Message Data"),
];

fn extract_streamer_info(prefs: &Value) -> Result<Value> {
    let value = prefs
        .get("streamerInfo")
        .ok_or_else(|| anyhow!("userPreference missing streamerInfo"))?;
    if let Some(array) = value.as_array() {
        array
            .first()
            .cloned()
            .ok_or_else(|| anyhow!("streamerInfo array is empty"))
    } else {
        Ok(value.clone())
    }
}

fn authorization_url(app: &AppConfig) -> String {
    format!(
        "{}?client_id={}&redirect_uri={}",
        OAUTH_AUTHORIZE_URL,
        urlencoding::encode(&app.client_id),
        urlencoding::encode(&app.callback_url)
    )
}

fn parse_auth_code(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        bail!("Missing callback URL or code. Next: schwab-cli auth url");
    }
    if !trimmed.contains("code=") {
        return Ok(urlencoding::decode(trimmed)?.to_string());
    }
    let query = trimmed.split('?').nth(1).unwrap_or(trimmed);
    for part in query.split('&') {
        if let Some((key, value)) = part.split_once('=') {
            if key == "code" {
                return Ok(urlencoding::decode(value)?.to_string());
            }
        }
    }
    bail!("Could not find code= in callback URL. Next: schwab-cli auth exchange --code <code>")
}

fn basic_auth_header(app: &AppConfig) -> String {
    format!(
        "Basic {}",
        STANDARD.encode(format!("{}:{}", app.client_id, app.client_secret))
    )
}

fn auth_status_payload() -> Value {
    let app_present = app_config_path().exists();
    let token = load_token().ok();
    json!({
        "app_config_present": app_present,
        "app_config_path": app_config_path(),
        "token_present": token.is_some(),
        "token_path": token_path(),
        "token": token.as_ref().map(redacted_token_payload),
        "account_map_present": account_map_path().exists(),
        "account_map_path": account_map_path(),
    })
}

fn redacted_token_payload(token: &TokenStore) -> Value {
    redacted_token_payload_at(token, Utc::now())
}

fn redacted_token_payload_at(token: &TokenStore, now: DateTime<Utc>) -> Value {
    let access_seconds_remaining = (token.access_expires_at - now).num_seconds();
    let refresh_seconds_remaining = (token.refresh_expires_at - now).num_seconds();
    let status = if refresh_seconds_remaining <= 0 {
        "expired"
    } else if refresh_seconds_remaining <= REFRESH_TOKEN_RENEWAL_SKEW_SECONDS {
        "renew_soon"
    } else if access_seconds_remaining <= ACCESS_REFRESH_SKEW_SECONDS {
        "access_renew_soon"
    } else {
        "ok"
    };
    let next = match status {
        "expired" => "schwab-cli auth login",
        "renew_soon" | "access_renew_soon" => "schwab-cli auth refresh",
        _ => "schwab-cli account-numbers",
    };
    json!({
        "status": status,
        "token_type": token.token_type,
        "scope": token.scope,
        "access_expires_at": token.access_expires_at,
        "refresh_expires_at": token.refresh_expires_at,
        "access_seconds_remaining": access_seconds_remaining,
        "refresh_seconds_remaining": refresh_seconds_remaining,
        "auto_refresh_note": "API commands refresh automatically while the refresh token is valid. Install keepalive to refresh daily even when no Schwab command is run.",
        "has_access_token": !token.access_token.is_empty(),
        "has_refresh_token": !token.refresh_token.is_empty(),
        "next": next
    })
}

fn version_payload() -> Value {
    json!({
        "name": "schwab-cli",
        "version": env!("CARGO_PKG_VERSION"),
        "repo_root": repo_root(),
        "docs_catalog": endpoint_catalog_path(),
        "trader_base": TRADER_BASE,
        "market_base": MARKET_BASE,
        "coverage": schwab_coverage_payload(),
    })
}

fn doctor_payload() -> Value {
    let path_var = std::env::var("PATH").unwrap_or_default();
    let path_entries: Vec<&str> = path_var.split(':').collect();
    let user_bin = user_bin_path();
    let global_bin = user_bin.join("schwab-cli");
    json!({
        "coverage": schwab_coverage_payload(),
        "crate_root": repo_root(),
        "config_dir": schwab_secrets_dir(),
        "data_dir": schwab_data_dir(),
        "app_config": {"path": app_config_path(), "exists": app_config_path().exists()},
        "token": auth_status_payload().get("token").cloned().unwrap_or(Value::Null),
        "account_map": {"path": account_map_path(), "exists": account_map_path().exists()},
        "docs": {"catalog": endpoint_catalog_path(), "exists": endpoint_catalog_path().exists()},
        "global_bin": {"path": global_bin, "exists": global_bin.exists(), "user_bin_on_path": path_entries.iter().any(|entry| Path::new(entry) == user_bin)},
        "auth_keepalive": keepalive_status_payload(),
        "live_trading_env_enabled": std::env::var("SCHWAB_CLI_ALLOW_LIVE_TRADING").ok().as_deref() == Some("1")
    })
}

fn keepalive_status_payload() -> Value {
    let path = keepalive_plist_path();
    let token = load_token().ok();
    match path {
        Ok(path) => json!({
            "label": KEEPALIVE_LABEL,
            "plist_path": path,
            "installed": path.exists(),
            "schedule": {"hour": DEFAULT_KEEPALIVE_HOUR, "minute": DEFAULT_KEEPALIVE_MINUTE},
            "token": token.as_ref().map(redacted_token_payload),
            "next": if path.exists() {
                "schwab-cli auth status"
            } else {
                "schwab-cli auth keepalive install"
            }
        }),
        Err(err) => json!({
            "label": KEEPALIVE_LABEL,
            "installed": false,
            "error": err.to_string(),
            "token": token.as_ref().map(redacted_token_payload),
            "next": "Set HOME or run schwab-cli auth keepalive install manually"
        }),
    }
}

fn schwab_coverage_payload() -> Value {
    json!({
        "scope": "Schwab Trader API accounts authorized during OAuth consent",
        "excludes": ["Accounts not offered in OAuth consent", "Schwab banking", "Many workplace retirement-plan surfaces", "Direct Treasury/CD/fixed-income order entry"],
        "note": SCHWAB_COVERAGE_NOTE,
        "source": "Schwab OAuth consent and published Trader API surface"
    })
}

fn emit(cli: &Cli, payload: Value, start: Instant, next: Vec<&str>) -> Result<()> {
    if cli.agent {
        let serialized = serde_json::to_string_pretty(&payload)?;
        let line_count = serialized.lines().count();
        let data = if serialized.len() > AGENT_MAX_BYTES || line_count > AGENT_MAX_LINES {
            let overflow_path = overflow_path();
            write_json(&overflow_path, &payload)?;
            let preview = serialized
                .lines()
                .take(AGENT_MAX_LINES)
                .collect::<Vec<_>>()
                .join("\n");
            json!({"truncated": true, "preview": preview, "overflow_path": overflow_path})
        } else {
            payload
        };
        let envelope = json!({
            "ok": true,
            "data": data,
            "meta": {"duration_ms": start.elapsed().as_millis(), "emitted_at_utc": Utc::now().to_rfc3339()},
            "next": next,
        });
        println!(
            "{}",
            if cli.compact {
                serde_json::to_string(&envelope)?
            } else {
                serde_json::to_string_pretty(&envelope)?
            }
        );
    } else {
        println!(
            "{}",
            if cli.compact {
                serde_json::to_string(&payload)?
            } else {
                serde_json::to_string_pretty(&payload)?
            }
        );
    }
    Ok(())
}

fn push_opt(params: &mut Vec<(String, String)>, key: &str, value: &Option<String>) {
    if let Some(value) = value {
        params.push((key.to_string(), value.clone()));
    }
}

fn push_bool_opt(params: &mut Vec<(String, String)>, key: &str, value: Option<bool>) {
    if let Some(value) = value {
        params.push((key.to_string(), value.to_string()));
    }
}

fn push_num_opt<T: ToString>(params: &mut Vec<(String, String)>, key: &str, value: Option<T>) {
    if let Some(value) = value {
        params.push((key.to_string(), value.to_string()));
    }
}

fn push_float_opt(params: &mut Vec<(String, String)>, key: &str, value: Option<f64>) {
    if let Some(value) = value {
        params.push((key.to_string(), value.to_string()));
    }
}

fn normalize_transaction_types(input: &[String]) -> Vec<String> {
    if input.is_empty() {
        return TRANSACTION_TYPES.iter().map(|v| (*v).to_string()).collect();
    }
    input
        .iter()
        .flat_map(|item| item.split(','))
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn normalize_schwab_datetime(value: &str, end_of_day: bool) -> String {
    let trimmed = value.trim();
    if is_date_only(trimmed) {
        if end_of_day {
            format!("{trimmed}T23:59:59.000Z")
        } else {
            format!("{trimmed}T00:00:00.000Z")
        }
    } else if trimmed.ends_with('Z') && trimmed.contains('T') && !trimmed.contains('.') {
        format!("{}.000Z", trimmed.trim_end_matches('Z'))
    } else {
        trimmed.to_string()
    }
}

fn format_schwab_datetime(value: DateTime<Utc>) -> String {
    value.format("%Y-%m-%dT%H:%M:%S.000Z").to_string()
}

fn is_date_only(value: &str) -> bool {
    value.len() == 10
        && value.as_bytes().get(4) == Some(&b'-')
        && value.as_bytes().get(7) == Some(&b'-')
        && value
            .chars()
            .enumerate()
            .all(|(idx, ch)| idx == 4 || idx == 7 || ch.is_ascii_digit())
}

fn parse_params(pairs: &[String]) -> Result<Vec<(String, String)>> {
    let mut params = Vec::new();
    for pair in pairs {
        let (key, value) = pair
            .split_once('=')
            .ok_or_else(|| anyhow!("Expected key=value for --param, got {pair}"))?;
        params.push((key.to_string(), value.to_string()));
    }
    Ok(params)
}

fn prepare_generic_path(
    client: &mut SchwabClient,
    path: &str,
    account: Option<&str>,
) -> Result<String> {
    if path.contains("{account}") || path.contains("{accountNumber}") {
        let account_id = account.ok_or_else(|| {
            anyhow!("path contains account placeholder; pass --account <alias-or-last4>")
        })?;
        let resolved = resolve_account(client, account_id)?;
        Ok(path
            .replace("{account}", &resolved.hash_value)
            .replace("{accountNumber}", &resolved.hash_value))
    } else {
        Ok(path.to_string())
    }
}

fn read_json_body(path: &Path) -> Result<Value> {
    let mut text = String::new();
    if path == Path::new("-") {
        io::stdin().read_to_string(&mut text)?;
    } else {
        text = fs::read_to_string(path)
            .with_context(|| format!("Failed to read JSON body file {}", path.display()))?;
    }
    serde_json::from_str(&text)
        .with_context(|| format!("Failed to parse JSON body from {}", path.display()))
}

fn is_live_order_mutation(method: &str, path: &str) -> bool {
    matches!(method, "POST" | "PUT" | "DELETE")
        && path.contains("/orders")
        && !path.contains("/previewOrder")
}

fn assert_live_order_allowed(yes_live_order: bool) -> Result<()> {
    if !yes_live_order
        || std::env::var("SCHWAB_CLI_ALLOW_LIVE_TRADING")
            .ok()
            .as_deref()
            != Some("1")
    {
        bail!("live order refused. Reason: order mutations require SCHWAB_CLI_ALLOW_LIVE_TRADING=1 and --yes-live-order. Next: schwab-cli orders preview --account <account> --json-file order.json");
    }
    Ok(())
}

fn base_arg(base: BaseArg) -> ApiBase {
    match base {
        BaseArg::Trader => ApiBase::Trader,
        BaseArg::Market => ApiBase::Market,
    }
}

fn base_url(base: ApiBase) -> &'static str {
    match base {
        ApiBase::Trader => TRADER_BASE,
        ApiBase::Market => MARKET_BASE,
    }
}

fn public_account_entries(entries: &[AccountEntry]) -> Vec<Value> {
    entries.iter().map(public_account_entry).collect()
}

fn public_account_entry(entry: &AccountEntry) -> Value {
    json!({
        "alias": entry.alias,
        "label": account_label(entry),
        "last4": entry.last4,
        "hash_value": entry.hash_value,
        "nick_name": entry.nick_name,
        "display_acct_id": entry.display_acct_id,
        "account_type": entry.account_type,
    })
}

fn cash_status_payload(
    account: &AccountEntry,
    payload: &Value,
    include_raw: bool,
) -> Result<Value> {
    let securities = payload
        .get("securitiesAccount")
        .ok_or_else(|| anyhow!("account response missing securitiesAccount"))?;
    let account_type = securities
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or("UNKNOWN");
    let initial = securities
        .get("initialBalances")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let current = securities
        .get("currentBalances")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let projected = securities
        .get("projectedBalances")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let mut root = json!({
        "account": public_account_entry(account),
        "fetched_at_utc": Utc::now().to_rfc3339(),
        "source": {
            "endpoint": "GET /accounts/{accountNumber}",
            "positions_requested": false,
            "note": "Balance-only command. It does not run order preview or place orders."
        },
        "account_type": account_type,
        "account_flags": {
            "is_day_trader": securities.get("isDayTrader"),
            "is_closing_only_restricted": securities.get("isClosingOnlyRestricted"),
            "pfcb_flag": securities.get("pfcbFlag")
        },
        "balances": {
            "initial": compact_balance_fields(&initial),
            "current": compact_balance_fields(&current),
            "projected": compact_balance_fields(&projected)
        },
        "interpretation": cash_interpretation(account_type, &initial, &current, &projected),
        "next": [
            "Use schwab-cli orders preview --account <account> --json-file order.json for a specific order preflight.",
            "If Schwab web shows incoming cash on hold but this command does not, wait for the transfer to clear or verify manually in Schwab web."
        ]
    });
    if include_raw {
        root["raw_securities_account"] = securities.clone();
    }
    Ok(root)
}

fn compact_balance_fields(balance: &Value) -> Value {
    const FIELDS: &[&str] = &[
        "cashBalance",
        "cashAvailableForTrading",
        "cashAvailableForWithdrawal",
        "availableFunds",
        "availableFundsNonMarginableTrade",
        "buyingPower",
        "buyingPowerNonMarginableTrade",
        "stockBuyingPower",
        "optionBuyingPower",
        "dayTradingBuyingPower",
        "sma",
        "pendingDeposits",
        "unsettledCash",
        "cashReceipts",
        "totalCash",
        "moneyMarketFund",
        "marginBalance",
        "equity",
        "liquidationValue",
        "longMarketValue",
        "longMarginValue",
        "maintenanceRequirement",
        "maintenanceCall",
        "regTCall",
    ];
    let mut out = Map::new();
    for field in FIELDS {
        if let Some(value) = balance.get(*field) {
            out.insert((*field).to_string(), value.clone());
        }
    }
    Value::Object(out)
}

fn cash_interpretation(
    account_type: &str,
    initial: &Value,
    current: &Value,
    projected: &Value,
) -> Value {
    let cash_balance = first_f64(&[
        current.get("cashBalance"),
        initial.get("cashBalance"),
        current.get("totalCash"),
    ]);
    let withdrawable_cash = first_f64(&[
        current.get("cashAvailableForWithdrawal"),
        initial.get("cashAvailableForWithdrawal"),
    ]);
    let available_funds = first_f64(&[
        current.get("availableFunds"),
        projected.get("availableFunds"),
    ]);
    let non_margin_trade_capacity = first_f64(&[
        current.get("availableFundsNonMarginableTrade"),
        current.get("cashAvailableForTrading"),
        projected.get("availableFundsNonMarginableTrade"),
    ]);
    let buying_power_non_marginable_trade = first_f64(&[
        current.get("buyingPowerNonMarginableTrade"),
        projected.get("buyingPowerNonMarginableTrade"),
    ]);
    let stock_buying_power = first_f64(&[
        projected.get("stockBuyingPower"),
        current.get("stockBuyingPower"),
        projected.get("buyingPower"),
        current.get("buyingPower"),
    ]);
    let option_buying_power = first_f64(&[
        projected.get("optionBuyingPower"),
        current.get("optionBuyingPower"),
    ]);
    let pending_deposits = first_f64(&[
        current.get("pendingDeposits"),
        initial.get("pendingDeposits"),
    ]);
    let unsettled_cash = first_f64(&[current.get("unsettledCash"), initial.get("unsettledCash")]);
    let margin_balance = first_f64(&[current.get("marginBalance"), initial.get("marginBalance")]);
    let margin_included = match (stock_buying_power, non_margin_trade_capacity) {
        (Some(stock), Some(non_margin)) => stock > non_margin + 0.01,
        _ => account_type == "MARGIN",
    };

    let mut warnings = Vec::new();
    if account_type == "MARGIN" {
        warnings
            .push("Account is margin-enabled. Do not treat margin-inclusive buying power as cash.");
    }
    if margin_included {
        warnings.push("stock_buying_power appears to include margin capacity; prefer non_margin_trade_capacity for cash-only sizing.");
    }
    warnings.push("Schwab Trader API may not expose same-day held ACH cash shown by the web UI as Cash on Hold.");
    warnings.push("For a concrete trade, use orders preview; cash status intentionally does not run hidden order previews.");

    json!({
        "cash_balance": cash_balance,
        "withdrawable_cash": withdrawable_cash,
        "available_funds": available_funds,
        "non_margin_trade_capacity": non_margin_trade_capacity,
        "buying_power_non_marginable_trade": buying_power_non_marginable_trade,
        "stock_buying_power": stock_buying_power,
        "option_buying_power": option_buying_power,
        "pending_deposits_reported_by_api": pending_deposits,
        "unsettled_cash": unsettled_cash,
        "margin_balance": margin_balance,
        "margin_included_in_stock_buying_power": margin_included,
        "same_day_hold_visibility": "not_reliable_in_trader_api",
        "agent_rule": "Use non_margin_trade_capacity for conservative cash-only sizing. Use orders preview for order-specific validation. Never use stock_buying_power blindly when margin_included_in_stock_buying_power is true.",
        "warnings": warnings
    })
}

fn first_f64(values: &[Option<&Value>]) -> Option<f64> {
    values.iter().flatten().find_map(|value| value.as_f64())
}

fn account_label(entry: &AccountEntry) -> String {
    entry
        .alias
        .clone()
        .or_else(|| entry.nick_name.clone())
        .or_else(|| entry.account_type.clone())
        .unwrap_or_else(|| format!("...{}", entry.last4))
}

fn account_key(entry: &AccountEntry) -> String {
    entry
        .alias
        .clone()
        .unwrap_or_else(|| format!("...{}", entry.last4))
}

fn last4(value: &str) -> String {
    let chars: Vec<char> = value.chars().rev().take(4).collect();
    chars.into_iter().rev().collect()
}

fn safe_alias(value: String) -> String {
    value
        .to_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn open_url(url: &str) -> Result<()> {
    let status = ProcessCommand::new("open")
        .arg(url)
        .status()
        .context("Failed to run macOS open")?;
    if !status.success() {
        bail!("macOS open failed. Copy the authorization_url manually.");
    }
    Ok(())
}

fn load_app_config() -> Result<AppConfig> {
    read_json_typed(&app_config_path()).context(
        "Schwab app config missing. Next: schwab-cli setup --client-id ... --client-secret ...",
    )
}

fn load_token() -> Result<TokenStore> {
    read_json_typed(&token_path())
}

fn load_account_map() -> Result<AccountMap> {
    read_json_typed(&account_map_path())
}

fn read_json(path: &Path) -> Result<Value> {
    let text =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    serde_json::from_str(&text)
        .with_context(|| format!("Failed to parse JSON from {}", path.display()))
}

fn read_json_typed<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let text =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    serde_json::from_str(&text)
        .with_context(|| format!("Failed to parse JSON from {}", path.display()))
}

fn write_json(path: &Path, payload: &Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_string_pretty(payload)? + "\n")
        .with_context(|| format!("Failed to write {}", path.display()))
}

fn write_secret_json<T: Serialize>(path: &Path, payload: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let text = serde_json::to_string_pretty(payload)? + "\n";
    let mut options = OpenOptions::new();
    options.create(true).truncate(true).write(true);
    #[cfg(unix)]
    options.mode(0o600);
    let mut file = options
        .open(path)
        .with_context(|| format!("Failed to write {}", path.display()))?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

fn remove_if_exists(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_file(path).with_context(|| format!("Failed to remove {}", path.display()))?;
    }
    Ok(())
}

fn validate_keepalive_time(hour: u8, minute: u8) -> Result<()> {
    if hour > 23 {
        bail!("keepalive hour must be 0-23");
    }
    if minute > 59 {
        bail!("keepalive minute must be 0-59");
    }
    Ok(())
}

fn keepalive_plist(args: &AuthKeepaliveInstallArgs) -> String {
    let stdout = schwab_data_dir()
        .join("logs")
        .join("schwab-auth-refresh.log");
    let stderr = schwab_data_dir()
        .join("logs")
        .join("schwab-auth-refresh.err.log");
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>{label}</string>
  <key>ProgramArguments</key>
  <array>
    <string>{bin}</string>
    <string>--compact</string>
    <string>auth</string>
    <string>refresh</string>
  </array>
  <key>StartCalendarInterval</key>
  <dict>
    <key>Hour</key>
    <integer>{hour}</integer>
    <key>Minute</key>
    <integer>{minute}</integer>
  </dict>
  <key>RunAtLoad</key>
  <true/>
  <key>StandardOutPath</key>
  <string>{stdout}</string>
  <key>StandardErrorPath</key>
  <string>{stderr}</string>
</dict>
</plist>
"#,
        label = xml_escape(KEEPALIVE_LABEL),
        bin = xml_escape(&args.bin.display().to_string()),
        hour = args.hour,
        minute = args.minute,
        stdout = xml_escape(&stdout.display().to_string()),
        stderr = xml_escape(&stderr.display().to_string()),
    )
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn all_endpoints(catalog: &Value) -> Vec<Value> {
    let mut endpoints = Vec::new();
    for product_key in ["accounts_and_trading", "market_data"] {
        if let Some(items) = catalog
            .pointer(&format!("/products/{product_key}/endpoints"))
            .and_then(Value::as_array)
        {
            for item in items {
                endpoints.push(item.clone());
            }
        }
    }
    endpoints
}

fn model_names(path: &Path) -> Result<Vec<String>> {
    let models: Vec<Value> = read_json_typed(path)?;
    Ok(models
        .iter()
        .filter_map(|model| {
            model
                .get("name")
                .and_then(Value::as_str)
                .map(ToString::to_string)
        })
        .collect())
}

fn find_models(path: &Path, query: &str) -> Result<Vec<Value>> {
    let models: Vec<Value> = read_json_typed(path)?;
    let needle = query.to_lowercase();
    Ok(models
        .into_iter()
        .filter(|model| {
            model
                .get("name")
                .and_then(Value::as_str)
                .map(|name| name.eq_ignore_ascii_case(query))
                .unwrap_or(false)
                || serde_json::to_string(model)
                    .unwrap_or_default()
                    .to_lowercase()
                    .contains(&needle)
        })
        .collect())
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn schwab_secrets_dir() -> PathBuf {
    if let Some(path) = std::env::var_os("SCHWAB_CLI_CONFIG_DIR") {
        return PathBuf::from(path);
    }
    if let Some(path) = std::env::var_os("XDG_CONFIG_HOME") {
        return PathBuf::from(path).join("schwab-cli");
    }
    home_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".config")
        .join("schwab-cli")
}

fn schwab_data_dir() -> PathBuf {
    if let Some(path) = std::env::var_os("SCHWAB_CLI_DATA_DIR") {
        return PathBuf::from(path);
    }
    if let Some(path) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(path).join("schwab-cli");
    }
    home_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".local")
        .join("share")
        .join("schwab-cli")
}

fn app_config_path() -> PathBuf {
    schwab_secrets_dir().join("app.json")
}

fn token_path() -> PathBuf {
    schwab_secrets_dir().join("token.json")
}

fn account_map_path() -> PathBuf {
    schwab_secrets_dir().join("accounts.json")
}

fn keepalive_plist_path() -> Result<PathBuf> {
    Ok(home_dir()?
        .join("Library")
        .join("LaunchAgents")
        .join(format!("{KEEPALIVE_LABEL}.plist")))
}

fn home_dir() -> Result<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .filter(|path| !path.as_os_str().is_empty())
        .ok_or_else(|| anyhow!("HOME is not set"))
}

fn user_bin_path() -> PathBuf {
    home_dir()
        .unwrap_or_else(|_| PathBuf::from("/usr/local"))
        .join("bin")
}

fn docs_dir() -> PathBuf {
    repo_root().join("docs").join("schwab-api")
}

fn endpoint_catalog_path() -> PathBuf {
    docs_dir().join("endpoint-catalog.json")
}

fn trading_models_path() -> PathBuf {
    docs_dir().join("schemas").join("trading-models.json")
}

fn market_models_path() -> PathBuf {
    docs_dir().join("schemas").join("market-data-models.json")
}

fn oauth_doc_path() -> PathBuf {
    docs_dir().join("oauth.md")
}

fn trading_rest_doc_path() -> PathBuf {
    docs_dir().join("trading-rest.md")
}

fn market_rest_doc_path() -> PathBuf {
    docs_dir().join("market-data-rest.md")
}

fn streamer_doc_path() -> PathBuf {
    docs_dir().join("streamer.md")
}

fn order_examples_doc_path() -> PathBuf {
    docs_dir().join("order-examples.md")
}

fn overflow_path() -> PathBuf {
    PathBuf::from("/private/tmp")
        .join("schwab-cli-output")
        .join(format!(
            "schwab-cli-{}-{}.json",
            Utc::now().format("%Y%m%dT%H%M%SZ"),
            Uuid::new_v4()
        ))
}

#[allow(dead_code)]
fn _params_map(params: Vec<(String, String)>) -> HashMap<String, String> {
    params.into_iter().collect()
}

#[allow(dead_code)]
fn _sorted_object(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted = BTreeMap::new();
            for (key, value) in map {
                sorted.insert(key.clone(), _sorted_object(value));
            }
            json!(sorted)
        }
        Value::Array(items) => Value::Array(items.iter().map(_sorted_object).collect()),
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_url_uses_registered_callback() {
        let app = AppConfig {
            client_id: "abc 123".to_string(),
            client_secret: "secret".to_string(),
            callback_url: DEFAULT_CALLBACK_URL.to_string(),
        };
        let url = authorization_url(&app);
        assert!(url.starts_with(OAUTH_AUTHORIZE_URL));
        assert!(url.contains("client_id=abc%20123"));
        assert!(url.contains("redirect_uri=https%3A%2F%2F127.0.0.1%3A8182%2Fcallback"));
    }

    #[test]
    fn callback_parser_accepts_url_or_raw_code() {
        let from_url =
            parse_auth_code("https://127.0.0.1:8182/callback?code=abc%40123&session=x").unwrap();
        let from_code = parse_auth_code("abc%40123").unwrap();
        assert_eq!(from_url, "abc@123");
        assert_eq!(from_code, "abc@123");
    }

    #[test]
    fn account_numbers_parse_to_secret_map() {
        let payload = json!([
            {"accountNumber": "123456783", "hashValue": "encrypted-one"},
            {"accountNumber": "000000757", "hashValue": "encrypted-two"}
        ]);
        let map = parse_account_numbers(&payload).unwrap();
        assert_eq!(map.accounts.len(), 2);
        assert_eq!(map.accounts[0].last4, "6783");
        assert_eq!(map.accounts[1].hash_value, "encrypted-two");
    }

    #[test]
    fn coverage_payload_mentions_oauth_boundary() {
        let coverage = schwab_coverage_payload();
        assert!(coverage["note"].as_str().unwrap().contains("OAuth"));
        assert!(coverage["excludes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|value| value.as_str().unwrap().contains("OAuth")));
    }

    fn test_token_at(now: DateTime<Utc>, access_seconds: i64, refresh_seconds: i64) -> TokenStore {
        TokenStore {
            access_token: "access".to_string(),
            refresh_token: "refresh".to_string(),
            token_type: "Bearer".to_string(),
            scope: Some("api".to_string()),
            id_token: None,
            access_expires_at: now + Duration::seconds(access_seconds),
            refresh_expires_at: now + Duration::seconds(refresh_seconds),
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn token_refresh_triggers_on_access_or_refresh_expiry_window() {
        let now = Utc::now();
        assert!(!should_refresh_token_at(
            &test_token_at(now, 600, REFRESH_TOKEN_RENEWAL_SKEW_SECONDS + 60),
            now
        ));
        assert!(should_refresh_token_at(
            &test_token_at(
                now,
                ACCESS_REFRESH_SKEW_SECONDS,
                REFRESH_TOKEN_RENEWAL_SKEW_SECONDS + 60
            ),
            now
        ));
        assert!(should_refresh_token_at(
            &test_token_at(now, 600, REFRESH_TOKEN_RENEWAL_SKEW_SECONDS),
            now
        ));
    }

    #[test]
    fn token_status_reports_expired_refresh_as_login_required() {
        let now = Utc::now();
        let token = TokenStore {
            refresh_expires_at: now - Duration::seconds(1),
            ..test_token_at(now, 600, 600)
        };
        let payload = redacted_token_payload_at(&token, now);
        assert_eq!(payload["status"], json!("expired"));
        assert_eq!(payload["next"], json!("schwab-cli auth login"));
        assert!(ensure_refresh_token_valid_at(&token, now).is_err());
    }

    #[test]
    fn keepalive_plist_runs_compact_auth_refresh() {
        let args = AuthKeepaliveInstallArgs {
            hour: 7,
            minute: 15,
            bin: PathBuf::from("/usr/local/bin/schwab-cli"),
        };
        let plist = keepalive_plist(&args);
        assert!(plist.contains("<string>--compact</string>"));
        assert!(plist.contains("<string>auth</string>"));
        assert!(plist.contains("<string>refresh</string>"));
        assert!(plist.contains("<integer>7</integer>"));
        assert!(plist.contains("<integer>15</integer>"));
    }

    #[test]
    fn gzip_error_body_decodes_before_display() {
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder
            .write_all(br#"{"error":"invalid_grant","error_description":"refresh token expired"}"#)
            .unwrap();
        let bytes = encoder.finish().unwrap();

        let decoded = decode_response_bytes(&bytes, "gzip");
        assert!(decoded.contains("invalid_grant"));
        assert_eq!(
            summarize_error_body(&decoded),
            "error=invalid_grant; error_description=refresh token expired"
        );
    }

    #[test]
    fn gzip_magic_decodes_even_without_content_encoding_header() {
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(b"plain schwab error").unwrap();
        let bytes = encoder.finish().unwrap();

        assert_eq!(decode_response_bytes(&bytes, ""), "plain schwab error");
    }

    #[test]
    fn account_resolver_handles_alias_last4_and_ambiguity() {
        let map = AccountMap {
            updated_at: None,
            accounts: vec![
                AccountEntry {
                    account_number: "123456783".to_string(),
                    hash_value: "hash-a".to_string(),
                    last4: "6783".to_string(),
                    alias: Some("individual".to_string()),
                    nick_name: None,
                    display_acct_id: None,
                    account_type: None,
                },
                AccountEntry {
                    account_number: "999996783".to_string(),
                    hash_value: "hash-b".to_string(),
                    last4: "6783".to_string(),
                    alias: Some("roth".to_string()),
                    nick_name: None,
                    display_acct_id: None,
                    account_type: None,
                },
            ],
        };
        assert_eq!(
            resolve_from_map(&map, "individual").unwrap().hash_value,
            "hash-a"
        );
        assert_eq!(resolve_from_map(&map, "roth").unwrap().hash_value, "hash-b");
        assert!(resolve_from_map(&map, "6783")
            .unwrap_err()
            .to_string()
            .contains("ambiguous"));
    }

    #[test]
    fn cash_status_prefers_non_margin_capacity_and_marks_margin_power() {
        let account = AccountEntry {
            account_number: "123456783".to_string(),
            hash_value: "hash-a".to_string(),
            last4: "6783".to_string(),
            alias: Some("individual".to_string()),
            nick_name: Some("Individual".to_string()),
            display_acct_id: Some("...783".to_string()),
            account_type: Some("BROKERAGE".to_string()),
        };
        let payload = json!({
            "securitiesAccount": {
                "type": "MARGIN",
                "isDayTrader": false,
                "isClosingOnlyRestricted": false,
                "pfcbFlag": false,
                "initialBalances": {
                    "cashBalance": 367.93,
                    "pendingDeposits": 0.0
                },
                "currentBalances": {
                    "cashBalance": 367.93,
                    "availableFunds": 6391.50,
                    "availableFundsNonMarginableTrade": 6391.50,
                    "buyingPower": 17004.0,
                    "buyingPowerNonMarginableTrade": 6391.50,
                    "pendingDeposits": 0.0,
                    "marginBalance": 0.0
                },
                "projectedBalances": {
                    "availableFunds": 6391.50,
                    "availableFundsNonMarginableTrade": 6391.50,
                    "stockBuyingPower": 17004.0
                }
            }
        });
        let status = cash_status_payload(&account, &payload, false).unwrap();
        assert_eq!(status["interpretation"]["cash_balance"], json!(367.93));
        assert_eq!(
            status["interpretation"]["non_margin_trade_capacity"],
            json!(6391.50)
        );
        assert_eq!(
            status["interpretation"]["stock_buying_power"],
            json!(17004.0)
        );
        assert_eq!(
            status["interpretation"]["margin_included_in_stock_buying_power"],
            json!(true)
        );
        assert_eq!(status.get("raw_securities_account"), None);
        assert!(status["source"]["note"]
            .as_str()
            .unwrap()
            .contains("does not run order preview"));
    }

    #[test]
    fn live_order_safety_requires_flag_and_env() {
        std::env::remove_var("SCHWAB_CLI_ALLOW_LIVE_TRADING");
        assert!(assert_live_order_allowed(true)
            .unwrap_err()
            .to_string()
            .contains("live order refused"));
        std::env::set_var("SCHWAB_CLI_ALLOW_LIVE_TRADING", "1");
        assert!(assert_live_order_allowed(false)
            .unwrap_err()
            .to_string()
            .contains("live order refused"));
        assert!(assert_live_order_allowed(true).is_ok());
        std::env::remove_var("SCHWAB_CLI_ALLOW_LIVE_TRADING");
    }

    #[test]
    fn generic_order_mutation_detection_excludes_preview() {
        assert!(is_live_order_mutation("POST", "/accounts/hash/orders"));
        assert!(is_live_order_mutation(
            "DELETE",
            "/accounts/hash/orders/123"
        ));
        assert!(!is_live_order_mutation(
            "POST",
            "/accounts/hash/previewOrder"
        ));
        assert!(!is_live_order_mutation("GET", "/accounts/hash/orders"));
    }

    #[test]
    fn schwab_datetime_normalization_accepts_date_only_and_short_utc() {
        assert_eq!(
            normalize_schwab_datetime("2026-04-01", false),
            "2026-04-01T00:00:00.000Z"
        );
        assert_eq!(
            normalize_schwab_datetime("2026-04-01", true),
            "2026-04-01T23:59:59.000Z"
        );
        assert_eq!(
            normalize_schwab_datetime("2026-04-01T12:30:00Z", false),
            "2026-04-01T12:30:00.000Z"
        );
    }

    #[test]
    fn streamer_decoder_names_equity_fields_and_preserves_raw() {
        let decoded = decode_streamer_message(json!({
            "data": [{
                "service": "LEVELONE_EQUITIES",
                "content": [{
                    "key": "AAPL",
                    "1": 183.75,
                    "2": 183.80,
                    "19": 199.62
                }]
            }]
        }));
        let item = &decoded["data"][0]["content"][0];
        assert_eq!(item["bid_price"], json!(183.75));
        assert_eq!(item["ask_price"], json!(183.80));
        assert_eq!(item["field_52_week_high"], json!(199.62));
        assert_eq!(item["_raw_fields"]["1"], json!(183.75));
    }

    #[test]
    fn streamer_decoder_parses_account_activity_json_message_data() {
        let decoded = decode_streamer_message(json!({
            "data": [{
                "service": "ACCT_ACTIVITY",
                "content": [{
                    "seq": 1,
                    "key": "Account Activity",
                    "1": "123456783",
                    "2": "OrderFill",
                    "3": "{\"orderId\":42}"
                }]
            }]
        }));
        let item = &decoded["data"][0]["content"][0];
        assert_eq!(item["account"], json!("123456783"));
        assert_eq!(item["message_type"], json!("OrderFill"));
        assert_eq!(item["message_data_json"]["orderId"], json!(42));
    }

    #[test]
    fn streamer_fields_payload_supports_all_services() {
        let all = streamer_field_maps_payload(None).unwrap();
        assert_eq!(all.as_object().unwrap().len(), 13);
        let equities = streamer_field_maps_payload(Some("LEVELONE_EQUITIES")).unwrap();
        assert_eq!(equities["LEVELONE_EQUITIES"].as_array().unwrap().len(), 52);
    }

    #[test]
    fn numeric_decoder_handles_positional_arrays_for_nested_book_records() {
        let mut value = json!([101.25, 400, 2]);
        decode_numeric_object(&mut value, BOOK_PRICE_LEVEL_FIELDS);
        assert_eq!(value["price"], json!(101.25));
        assert_eq!(value["aggregate_size"], json!(400));
        assert_eq!(value["market_maker_count"], json!(2));
    }
}
