#!/usr/bin/env sh
set -eu

repo="${SCHWAB_CLI_REPO:-pashpashpash/schwab-cli}"
bin_dir="${SCHWAB_CLI_INSTALL_DIR:-$HOME/bin}"

os="$(uname -s)"
arch="$(uname -m)"
case "$os:$arch" in
  Darwin:arm64) target="aarch64-apple-darwin" ;;
  Darwin:x86_64) target="x86_64-apple-darwin" ;;
  Linux:x86_64) target="x86_64-unknown-linux-gnu" ;;
  *) echo "unsupported platform: $os $arch" >&2; exit 1 ;;
esac

asset="schwab-cli-$target"
api="https://api.github.com/repos/$repo/releases/latest"
url="$(curl -fsSL "$api" | sed -n 's/.*"browser_download_url": "\([^"]*'"$asset"'\)".*/\1/p' | head -n 1)"
if [ -z "$url" ]; then
  echo "could not find release asset $asset in $api" >&2
  exit 1
fi

mkdir -p "$bin_dir"
tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT
curl -fsSL "$url" -o "$tmp"
chmod +x "$tmp"
mv "$tmp" "$bin_dir/schwab-cli"
echo "installed $bin_dir/schwab-cli"
echo "run: schwab-cli version"
