# Density-based clustering

## Goal

Implements the algorithm DBSCAN (Density-Based Spatial Clustering of Applications with Noise) for clustering spatial
data.

## References

Ester, M., Kriegel, H.-P., Sander, J., & Xu, X. (1996). **A Density-Based Algorithm for Discovering Clusters in Large
Spatial Databases with Noise**. *Proceedings of the 2nd International Conference on Knowledge Discovery and Data
Mining (KDD '96)*, 226–231.

## Dataset

### Point set (input)

```
x11, x12, x13, ..., x1D
...
xN1, xN2, xN3, ..., xND
```

### Clustered set (output)

```
cid_1, x11, x12, ..., x1D
...
cid_N, xN1, xN2, ..., xND
```

- `cid` is the cluster ID for the point on that line.
- `cid == -1` denotes NOISE.

## CLI Usage

This repo provides three Rust binaries for end-to-end clustering workflows.

1) DBSCAN
- Run DBSCAN over a points CSV and produce clustered output.
- Command:
  - `cargo run --manifest-path rust/Cargo.toml --bin dbscan -- <input.csv> <output.csv> <min_points> <eps>`
- Input format: each line `x1,x2,...,xD` (no header)
- Output format: each line `cid,x1,x2,...,xD` with noise as `cid = -1`

2) Plot (2D)
- Visualize clustered CSV (any dimension; choose which two axes to draw).
- Command:
  - `cargo run --manifest-path rust/Cargo.toml --bin plot -- <clustered.csv> <out.png> [--x-col 0] [--y-col 1] [--width 1000] [--height 800] [--point-size 2] [--title "Clustering Plot"]`
- Input format: each line `cid,x1,x2,...`

3) k-distance plot
- Compute the k-th nearest neighbor distance for each point and plot the sorted curve (helpful for picking `eps`).
- Command:
  - `cargo run --manifest-path rust/Cargo.toml --bin k_dist -- <input.csv> <out.png> [-k 4] [--width 1200] [--height 800] [--title "k-distance plot"]`
- Input format: each line `x1,x2,...`

Notes
- DBSCAN uses the R-tree query engine (rstar) with runtime dispatch for 1..=16 dimensions.
- k-distance also uses the real query engine; `k` is the k-th neighbor excluding the point itself.

## Workflow Script

For convenience, use `scripts/workflow.sh` to produce a k-distance plot, then (optionally) run DBSCAN and plot results.

- Step 1: Generate k-distance plot
  - `bash scripts/workflow.sh -n <name> [-k 4]`
  - Reads `testcases/input/<name>.csv`
  - Writes `testcases/output/<name>_k<k>_kdist.png`
- Step 2: Pick `eps` from the elbow of the curve
  - Then run:
    - `bash scripts/workflow.sh -n <name> -e <eps> [-k 4] [-m <min_points>] [--x-col 0] [--y-col 1]`
  - Writes clustered CSV to `testcases/output/<name>_eps<eps>_k<k>.csv` and PNG to `testcases/output/<name>_eps<eps>_k<k>.png`

Examples
- `bash scripts/workflow.sh -n moons -k 4`
- `bash scripts/workflow.sh -n moons -k 4 -e 0.15 -m 4 --x-col 0 --y-col 1`

## Mise Setup (optional)

You can use `mise` to install pinned toolchains defined in this repo.

- Install mise (choose one):
  - macOS (Homebrew): `brew install mise`
  - Linux/macOS (curl): `curl https://mise.jdx.dev/install.sh | sh`
  - Docs: https://mise.jdx.dev/

- Trust and install toolchains:
  - `mise trust`          # trust the repo configuration
  - `mise install`        # install pinned toolchains (e.g., Rust from rust/mise.toml)

Notes
- This repo currently does not include root-level mise tasks. Use the Cargo commands shown above to run the binaries.
- Toolchain pins live under `rust/mise.toml` and `python/mise.toml`.
