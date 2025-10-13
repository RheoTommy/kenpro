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
