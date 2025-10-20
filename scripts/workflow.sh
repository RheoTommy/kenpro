#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat << 'USAGE'
Usage:
  workflow.sh -n <name> [-k <k>] [-e <eps>] [-m <min_points>] [--x-col <i>] [--y-col <j>] [--input <path>] [--outdir <dir>]

Description:
  - Reads points from testcases/input/<name>.csv (or --input path)
  - If -e/--eps is NOT provided, generates k-distance plot at testcases/output/<name>_k<k>_kdist.png (or --outdir)
  - If -e/--eps IS provided, skips k-distance and runs DBSCAN and plots results to
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

# Helper for timing
_now() { date +%s; }
_print_dur() { local s=$1; local e=$2; echo "$((e - s))s"; }

total_start=$(_now)

# If eps is provided, skip k-dist and only run DBSCAN+plot
if [[ -n "$eps" ]]; then
  mp="$k"
  if [[ -n "$min_points" ]]; then mp="$min_points"; fi
  eps_tag=$(printf "%g" "$eps" | tr '.' 'p')
  clustered_csv="$outdir/${name}_eps${eps_tag}_k${k}.csv"
  clustered_png="$outdir/${name}_eps${eps_tag}_k${k}.png"

  echo "Running DBSCAN: eps=$eps, min_points=$mp"
  step_start=$(_now)
  cargo run --release --manifest-path rust/Cargo.toml --bin dbscan -- "$input" "$clustered_csv" "$mp" "$eps"
  step_end=$(_now)
  echo "  -> Wrote $clustered_csv"
  echo "  -> DBSCAN time: $(_print_dur $step_start $step_end)"

  echo "Plotting clustered result: $clustered_png (x_col=$x_col, y_col=$y_col)"
  step_start=$(_now)
  cargo run --release --manifest-path rust/Cargo.toml --bin plot -- "$clustered_csv" "$clustered_png" --x-col "$x_col" --y-col "$y_col"
  step_end=$(_now)
  echo "  -> Wrote $clustered_png"
  echo "  -> Plot time:   $(_print_dur $step_start $step_end)"
else
  # Generate only k-distance plot
  kdist_png="$outdir/${name}_k${k}_kdist.png"
  echo "Generating k-distance plot: $kdist_png (k=$k)"
  step_start=$(_now)
  cargo run --release --manifest-path rust/Cargo.toml --bin k_dist -- "$input" "$kdist_png" -k "$k"
  step_end=$(_now)
  echo "  -> Wrote $kdist_png"
  echo "  -> k-dist time: $(_print_dur $step_start $step_end)"
  echo "No eps provided. Inspect $kdist_png and rerun with -e <eps> to complete DBSCAN and plotting."
fi

total_end=$(_now)
echo "Total time: $(_print_dur $total_start $total_end)"
