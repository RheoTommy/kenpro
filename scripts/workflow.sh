#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat << 'USAGE'
Usage:
  workflow.sh -n <name> [-k <k>] [-e <eps>] [-m <min_points>] [--x-col <i>] [--y-col <j>] [--input <path>] [--outdir <dir>]

Description:
  - Reads points from testcases/input/<name>.csv (or --input path)
  - Generates k-distance plot at testcases/output/<name>_kdist.png (or --outdir)
  - If -e/--eps is provided, also runs DBSCAN and plots results to
    testcases/output/<name>.csv and testcases/output/<name>.png

Options:
  -n, --name <name>         Testcase name (required)
  -k, --k <k>               k for k-distance (default 4)
  -e, --eps <eps>           Epsilon for DBSCAN (optional)
  -m, --min-points <m>      min_points for DBSCAN (default: k)
      --x-col <i>           X column index for plotting (default 0; excludes cid)
      --y-col <j>           Y column index for plotting (default 1; excludes cid)
      --input <path>        Override input CSV path
      --outdir <dir>        Output directory (default testcases/output)
  -h, --help                Show this help
USAGE
}

name=""
k=4
eps=""
min_points=""
x_col=0
y_col=1
input_override=""
outdir="testcases/output"

while [[ $# -gt 0 ]]; do
  case "$1" in
    -n|--name) name="$2"; shift 2;;
    -k|--k) k="$2"; shift 2;;
    -e|--eps) eps="$2"; shift 2;;
    -m|--min-points) min_points="$2"; shift 2;;
    --x-col) x_col="$2"; shift 2;;
    --y-col) y_col="$2"; shift 2;;
    --input) input_override="$2"; shift 2;;
    --outdir) outdir="$2"; shift 2;;
    -h|--help) usage; exit 0;;
    *) echo "Unknown arg: $1" >&2; usage; exit 2;;
  esac
done

if [[ -z "$name" ]]; then
  echo "error: --name is required" >&2
  usage; exit 2
fi

input=${input_override:-"testcases/input/${name}.csv"}
if [[ ! -f "$input" ]]; then
  echo "error: input file not found: $input" >&2
  exit 2
fi

mkdir -p "$outdir"

# 1) k-distance plot
kdist_png="$outdir/${name}_kdist.png"
echo "[1/2] Generating k-distance plot: $kdist_png (k=$k)"
cargo run --manifest-path rust/Cargo.toml --bin k_dist -- "$input" "$kdist_png" -k "$k"
echo "  -> Wrote $kdist_png"

# 2) If eps provided, run DBSCAN and plot
if [[ -n "$eps" ]]; then
  mp="$k"
  if [[ -n "$min_points" ]]; then mp="$min_points"; fi
  clustered_csv="$outdir/${name}.csv"
  clustered_png="$outdir/${name}.png"

  echo "[2/2] Running DBSCAN: eps=$eps, min_points=$mp"
  cargo run --manifest-path rust/Cargo.toml --bin dbscan -- "$input" "$clustered_csv" "$mp" "$eps"
  echo "  -> Wrote $clustered_csv"

  echo "Plotting clustered result: $clustered_png (x_col=$x_col, y_col=$y_col)"
  cargo run --manifest-path rust/Cargo.toml --bin plot -- "$clustered_csv" "$clustered_png" --x-col "$x_col" --y-col "$y_col"
  echo "  -> Wrote $clustered_png"
else
  echo "No eps provided. Inspect $kdist_png and rerun with -e <eps> to complete DBSCAN and plotting."
fi

