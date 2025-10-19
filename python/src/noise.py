#!/usr/bin/env python3
"""
NumPy-based noise utilities for notebooks (no CLI).

- Adds uniform noise to a fraction of rows in an array.
- The uniform range per dimension is derived from the data's min/max.
"""

from __future__ import annotations

from typing import Optional

import numpy as np


def uniform_bounds(X: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    """Compute per-dimension (min, max).

    Parameters
    - X: array of shape (N, D)

    Returns
    - mins: array of shape (D,)
    - maxs: array of shape (D,)
    """
    X = np.asarray(X, dtype=float)
    if X.ndim != 2 or X.shape[0] == 0:
        raise ValueError("X must be a non-empty 2D array (N, D)")
    return X.min(axis=0), X.max(axis=0)


def add_uniform_noise(
    X: np.ndarray,
    *,
    p: float | None = None,
    n: int | None = None,
    seed: Optional[int] = None,
    rng: Optional[np.random.Generator] = None,
) -> tuple[np.ndarray, np.ndarray]:
    """Add new points sampled uniformly over X's bounding box.

    The bounding box is the smallest axis-aligned hyper-rectangle [mins, maxs]
    that contains all input points. New points are sampled i.i.d. from the
    uniform distribution over that box and then appended to X.

    Either `p` or `n` must be specified:
    - p: fraction of X's row count to add (k = round(p * N))
    - n: exact number of points to add

    Parameters
    - X: array of shape (N, D)
    - p: fraction in [0, 1] of N to add
    - n: number of points to add (non-negative int)
    - seed: random seed if `rng` is not provided
    - rng: optional numpy.random.Generator

    Returns
    - Y: array of shape (N + k, D), original X with k new points appended
    - idx_new: indices (length k) of the newly appended rows in Y (sorted)
    """
    X = np.asarray(X, dtype=float)
    if X.ndim != 2 or X.shape[0] == 0:
        raise ValueError("X must be a non-empty 2D array (N, D)")

    if (p is None) == (n is None):
        raise ValueError("Specify exactly one of p or n")

    N, D = X.shape
    if p is not None:
        if not (0.0 <= p <= 1.0):
            raise ValueError("p must be within [0, 1]")
        k = int(round(p * N))
    else:
        if n is None or n < 0:
            raise ValueError("n must be a non-negative integer")
        k = int(n)

    if k == 0:
        return X.copy(), np.empty((0,), dtype=int)

    if rng is None:
        rng = np.random.default_rng(seed)

    mins, maxs = uniform_bounds(X)
    # Broadcast mins/maxs to (k, D) and sample uniformly within the box.
    low = np.broadcast_to(mins, (k, D))
    high = np.broadcast_to(maxs, (k, D))
    noise_pts = rng.uniform(low=low, high=high, size=(k, D))

    Y = np.concatenate([X, noise_pts], axis=0)
    idx_new = np.arange(N, N + k, dtype=int)
    return Y, idx_new

