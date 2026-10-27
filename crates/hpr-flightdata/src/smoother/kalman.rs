//! The smoother's linear algebra: a Kalman filter's scalar update and the Rauch–Tung–Striebel
//! pass back, on a state of [`N`] numbers.
//!
//! The update takes one scalar measurement at a time, so no matrix is inverted going forward; the
//! pass back solves with each step's predicted covariance by a Cholesky factorization, the matrix
//! first scaled to a unit diagonal, which keeps a covariance whose entries span many orders of
//! magnitude (a height pinned to a millimeter on the pad beside a diffuse GNSS offset) as well
//! conditioned as its correlations allow.
//!
//! The equations are the textbook ones (S. Särkkä and L. Svensson, *Bayesian Filtering and
//! Smoothing*, 2nd ed., Cambridge University Press, 2023: the Kalman filter, Thm 6.6, eq. 6.21;
//! the Rauch–Tung–Striebel smoother, Thm 12.2, eq. 12.6, pp. 255–256, after H. E. Rauch, F. Tung
//! and C. T. Striebel, *AIAA Journal* 3(8), 1445–1450, 1965).


#![allow(clippy::needless_range_loop, reason = "matrix code reads as its equations do, by index")]

/// The number of states.
pub(super) const N: usize = 12;

/// A state.
pub(super) type Vector = [f64; N];

/// A matrix, by rows.
pub(super) type Matrix = [[f64; N]; N];

/// The number of entries in a symmetric matrix's upper triangle.
pub(super) const PACKED: usize = N * (N + 1) / 2;

/// The zero matrix.
pub(super) const ZERO: Matrix = [[0.0; N]; N];

/// The identity.
pub(super) fn identity() -> Matrix {
    let mut m = ZERO;
    for (i, row) in m.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    m
}

/// `a b`.
pub(super) fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut out = ZERO;
    for (i, row) in out.iter_mut().enumerate() {
        for (k, &aik) in a[i].iter().enumerate() {
            if aik == 0.0 {
                continue;
            }
            for (j, cell) in row.iter_mut().enumerate() {
                *cell += aik * b[k][j];
            }
        }
    }
    out
}

/// `a bᵀ`.
pub(super) fn mul_transposed(a: &Matrix, b: &Matrix) -> Matrix {
    let mut out = ZERO;
    for (i, row) in out.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = a[i].iter().zip(&b[j]).map(|(x, y)| x * y).sum();
        }
    }
    out
}

/// `a x`.
pub(super) fn apply(a: &Matrix, x: &Vector) -> Vector {
    let mut out = [0.0; N];
    for (i, cell) in out.iter_mut().enumerate() {
        *cell = a[i].iter().zip(x).map(|(m, v)| m * v).sum();
    }
    out
}

/// `F P Fᵀ + Q`.
pub(super) fn propagate(f: &Matrix, p: &Matrix, q: &Matrix) -> Matrix {
    let fp = mul(f, p);
    let mut out = mul_transposed(&fp, f);
    for (row, q_row) in out.iter_mut().zip(q) {
        for (cell, &qv) in row.iter_mut().zip(q_row) {
            *cell += qv;
        }
    }
    symmetrize(&mut out);
    out
}

/// Makes `p` exactly symmetric, each pair the mean of the two.
pub(super) fn symmetrize(p: &mut Matrix) {
    for i in 0..N {
        for j in (i + 1)..N {
            let mean = 0.5 * (p[i][j] + p[j][i]);
            p[i][j] = mean;
            p[j][i] = mean;
        }
    }
}

/// The upper triangle of a symmetric matrix, row by row.
pub(super) fn pack(p: &Matrix, out: &mut Vec<f64>) {
    for (i, row) in p.iter().enumerate() {
        out.extend_from_slice(&row[i..]);
    }
}

/// The symmetric matrix whose upper triangle `packed` holds.
pub(super) fn unpack(packed: &[f64]) -> Matrix {
    let mut p = ZERO;
    let mut at = 0;
    for i in 0..N {
        for j in i..N {
            p[i][j] = packed[at];
            p[j][i] = packed[at];
            at += 1;
        }
    }
    p
}

/// One scalar measurement's update, in place: the innovation `y` (the measurement less its
/// prediction), the row `h` of its derivatives by the state and its variance `r`. Returns the
/// innovation's variance `S = h P hᵀ + r`.
///
/// `K = P hᵀ/S`, its entries for the `held` states set to zero, `x ← x + K y`, and the covariance
/// in Joseph's form,
/// `P ← (I − K h) P (I − K h)ᵀ + K r Kᵀ` (Särkkä and Svensson, eq. 6.21's note; R. S. Bucy and
/// P. D. Joseph, 1968), which stays symmetric and positive semi-definite over the hundreds of
/// thousands of updates of a long, fast log where the shorter `P − K S Kᵀ` drifts, and which holds
/// for any gain, so for the held states' zero too. A held state is a Schmidt "consider" state
/// (S. F. Schmidt, 1966; Simon, *Optimal State Estimation*, Wiley, 2006, §7.6): its estimate is
/// never moved, but its uncertainty is carried, and the others' variances include it.
pub(super) fn update(
    x: &mut Vector,
    p: &mut Matrix,
    h: &Vector,
    y: f64,
    r: f64,
    held: &[bool; N],
) -> f64 {
    let ph = apply(p, h);
    let s = h.iter().zip(&ph).map(|(a, b)| a * b).sum::<f64>() + r;
    let k: Vector = std::array::from_fn(|i| if held[i] { 0.0 } else { ph[i] / s });
    for i in 0..N {
        x[i] += k[i] * y;
    }
    // (I − K h) P (I − K h)ᵀ + K r Kᵀ = P − K (hP) − (P hᵀ) Kᵀ + K (h P hᵀ + r) Kᵀ, with
    // hP = (P hᵀ)ᵀ and h P hᵀ + r = S.
    for i in 0..N {
        for j in 0..N {
            p[i][j] += -k[i] * ph[j] - ph[i] * k[j] + k[i] * k[j] * s;
        }
    }
    symmetrize(p);
    s
}

/// The innovation's variance `h P hᵀ + r`, without updating.
pub(super) fn innovation_variance(p: &Matrix, h: &Vector, r: f64) -> f64 {
    let ph = apply(p, h);
    h.iter().zip(&ph).map(|(a, b)| a * b).sum::<f64>() + r
}

/// Solves `A X = B` for symmetric positive-definite `A`, by a Cholesky factorization of `A`
/// scaled to a unit diagonal, `D A D` with `D = diag(1/√A_ii)`. `None` if `A` isn't positive
/// definite to working precision.
pub(super) fn solve(a: &Matrix, b: &Matrix) -> Option<Matrix> {
    let mut d = [0.0; N];
    for i in 0..N {
        if a[i][i].is_nan() || a[i][i] <= 0.0 {
            return None;
        }
        d[i] = 1.0 / a[i][i].sqrt();
    }
    // L, lower, of D A D = L Lᵀ.
    let mut l = ZERO;
    for i in 0..N {
        for j in 0..=i {
            let mut sum = a[i][j] * d[i] * d[j];
            for k in 0..j {
                sum -= l[i][k] * l[j][k];
            }
            if i == j {
                if sum.is_nan() || sum <= 0.0 {
                    return None;
                }
                l[i][i] = sum.sqrt();
            } else {
                l[i][j] = sum / l[j][j];
            }
        }
    }
    // A X = B ⇔ (D A D)(D⁻¹ X) = D B.
    let mut x = ZERO;
    for col in 0..N {
        let mut y = [0.0; N];
        for i in 0..N {
            let mut sum = d[i] * b[i][col];
            for k in 0..i {
                sum -= l[i][k] * y[k];
            }
            y[i] = sum / l[i][i];
        }
        for i in (0..N).rev() {
            let mut sum = y[i];
            for k in (i + 1)..N {
                sum -= l[k][i] * x[k][col];
            }
            x[i][col] = sum / l[i][i];
        }
    }
    for (i, row) in x.iter_mut().enumerate() {
        for cell in row.iter_mut() {
            *cell *= d[i];
        }
    }
    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spd(seed: u64) -> Matrix {
        // A random lower triangle times its transpose, plus a diagonal spanning ten decades.
        let mut state = seed;
        let mut next = || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((state >> 11) as f64 / (1_u64 << 53) as f64) - 0.5
        };
        let mut l = ZERO;
        for i in 0..N {
            for j in 0..=i {
                l[i][j] = next();
            }
            l[i][i] += 2.0;
        }
        let mut a = mul_transposed(&l, &l);
        let mut scale = [0.0; N];
        for (i, s) in scale.iter_mut().enumerate() {
            *s = 10_f64.powf(i as f64 * 10.0 / N as f64 - 5.0);
        }
        for i in 0..N {
            for j in 0..N {
                a[i][j] *= scale[i] * scale[j];
            }
        }
        a
    }

    #[test]
    fn solve_inverts_a_badly_scaled_spd_matrix() {
        // Its entries span twenty decades, so the residual is held on the scale of each row and
        // column: `(A X − I)ᵢⱼ √(Aⱼⱼ/Aᵢᵢ)`.
        let a = spd(7);
        let x = solve(&a, &identity()).expect("positive definite");
        let product = mul(&a, &x);
        for i in 0..N {
            for j in 0..N {
                let want = if i == j { 1.0 } else { 0.0 };
                let scaled = (product[i][j] - want) * (a[j][j] / a[i][i]).sqrt();
                assert!(scaled.abs() < 1e-12, "{i},{j}: {scaled}");
            }
        }
    }

    #[test]
    fn solve_refuses_a_matrix_that_isnt_positive_definite() {
        let mut a = identity();
        a[3][3] = -1.0;
        assert!(solve(&a, &identity()).is_none());
        let mut b = identity();
        b[0][1] = 2.0;
        b[1][0] = 2.0;
        assert!(solve(&b, &identity()).is_none());
    }

    #[test]
    fn a_scalar_update_matches_the_closed_form() {
        // One state measured directly: the posterior variance is P R/(P + R).
        let mut x = [0.0; N];
        let mut p = identity();
        p[0][0] = 4.0;
        let mut h = [0.0; N];
        h[0] = 1.0;
        let s = update(&mut x, &mut p, &h, 2.0, 1.0, &[false; N]);
        assert_eq!(s, 5.0);
        assert!((x[0] - 1.6).abs() < 1e-15);
        assert!((p[0][0] - 0.8).abs() < 1e-15);
        assert_eq!(p[1][1], 1.0);
    }

    #[test]
    fn a_held_state_keeps_its_estimate_and_carries_its_variance() {
        // z = x₀ + x₁ with x₁ held: x₀ takes the whole innovation's share P₀₀/S, and its variance
        // is P₀₀ − P₀₀²/S + ... = P₀₀ (P₁₁ + R)/S, the held one's spread left in it.
        let mut x = [0.0; N];
        let mut p = identity();
        p[0][0] = 4.0;
        p[1][1] = 2.0;
        let mut h = [0.0; N];
        h[0] = 1.0;
        h[1] = 1.0;
        let mut held = [false; N];
        held[1] = true;
        let s = update(&mut x, &mut p, &h, 7.0, 1.0, &held);
        assert_eq!(s, 7.0);
        assert!((x[0] - 4.0).abs() < 1e-15);
        assert_eq!(x[1], 0.0);
        assert!((p[0][0] - 4.0 * 3.0 / 7.0).abs() < 1e-14, "{}", p[0][0]);
        assert!((p[1][1] - 2.0).abs() < 1e-15);
        assert!((p[0][1] + 8.0 / 7.0).abs() < 1e-14, "{}", p[0][1]);
    }

    #[test]
    fn packing_round_trips() {
        let mut a = spd(3);
        symmetrize(&mut a);
        let mut packed = Vec::new();
        pack(&a, &mut packed);
        assert_eq!(packed.len(), PACKED);
        assert_eq!(unpack(&packed), a);
    }
}
