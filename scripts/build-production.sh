#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"
release_dir="${repo_root}/target/release"

cargo build --release --manifest-path "${repo_root}/Cargo.toml"

ln -sfn "terminal-snake" "${release_dir}/tsnake"
printf 'Built %s and linked %s\n' "${release_dir}/terminal-snake" "${release_dir}/tsnake"
