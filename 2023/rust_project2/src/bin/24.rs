use anyhow::{anyhow, Context as _};
use itertools::Itertools as _;
use nalgebra::{matrix, vector, Matrix, Matrix2, Matrix4, RowVector4, Vector, Vector3};
advent_of_code::solution!(24);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Hailstone {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}
fn parse(input: &str) -> Vec<Hailstone> {
    return input
        .split("\r\n")
        .map(|line| parse_line(line))
        .collect_vec();
}
fn parse_line(line: &str) -> Hailstone {
    let args = line.split(" @ ").collect_vec();
    let position = args[0]
        .split(", ")
        .map(|num| num.trim().parse::<f64>().unwrap())
        .collect_vec();
    let velocity: Vec<f64> = args[1]
        .split(", ")
        .map(|num| return num.trim().parse::<f64>().unwrap())
        .collect_vec();

    return Hailstone {
        position: vector![position[0], position[1], position[2]],
        velocity: vector![velocity[0], velocity[1], velocity[2]],
    };
}

fn solve_linear_system<T, D, S1, S2>(
    mut a: Matrix<T, D, D, S1>,
    mut b: Vector<T, D, S2>,
) -> Option<Vector<T, D, S2>>
where
    T: num::Num
        + num::traits::NumAssign
        + num::traits::Signed
        + PartialOrd
        + Clone
        + nalgebra::Scalar,
    D: nalgebra::Dim,
    S1: nalgebra::RawStorageMut<T, D, D> + std::fmt::Debug,
    S2: nalgebra::RawStorageMut<T, D, nalgebra::U1>,
{
    assert!(a.nrows() == a.ncols());
    assert!(a.nrows() == b.nrows());
    let dim = a.nrows();

    // Guassian elimination with back-substitution
    for i in 0..dim {
        let rows = a.index((i.., i));
        let (pivot_row, _) = rows
            .iter()
            .enumerate()
            // i-based indexing
            .map(|(j, v)| (j + i, v))
            //there’s no .max() for PartialOrd, let’s do this manually
            .fold::<Option<_>, _>(None, |prev, (j, v)| {
                if let Some((j_acc, prev)) = prev {
                    let v = num::abs(v.clone());

                    if v.partial_cmp(&prev)? == std::cmp::Ordering::Greater {
                        Some((j, v))
                    } else {
                        Some((j_acc, prev))
                    }
                } else {
                    Some((j, v.clone()))
                }
            })?;

        let pivot_el = a[(pivot_row, i)].clone();

        if pivot_el == T::zero() {
            return None;
        }

        if pivot_row != i {
            a.swap_rows(pivot_row, i);
            b.swap_rows(pivot_row, i);
        }

        // divide the row by pivot
        if i < dim - 1 {
            for el in a.index_mut((i, i + 1..)).iter_mut() {
                *el /= pivot_el.clone();
            }
        }
        *b.index_mut(i) /= pivot_el.clone();

        // subtract from the rows below
        for j in i + 1..dim {
            let factor = a[(j, i)].clone();
            for k in i + 1..dim {
                let correction = factor.clone() * a[(i, k)].clone();
                a[(j, k)] -= correction;
            }
            let correction = factor.clone() * b[i].clone();
            b[j] -= correction;
        }
    }

    // back-substitution
    for i in (0..dim - 1).rev() {
        for j in i + 1..dim {
            let c = b[j].clone() * a[(i, j)].clone();
            b[i] -= c;
        }
    }

    Some(b)
}
pub fn part_one(_input: &str) -> Option<u32> {
    None
}
pub fn part_two(_input: &str) -> Option<u32> {
    let a = part2(&parse(_input));
    dbg!(a);
    None
}

// Heavily inspired by ash30342’s solution:
// https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kf4jv41/?context=3
pub fn part2(hailstones: &[Hailstone]) -> anyhow::Result<u64> {
    // To solve part 2 we need at least 3 hailstones (then we get a system of 9 non-linear
    // equations with 9 unknowns).
    //
    // If P is a position vector (x, y, z) and V is velocity (v_x, v_y, v_z), then for every
    // hailstone n we have a vector equation of the form which consists of three equations, each
    // for every coordinate:
    //
    //   P_n + (V_n - V_s) · t_n = P_s
    //
    // we need to take hailstones n ∈ [0, 3) – thus having 9 non-linear equations together and solve
    // them for P_s which is the solution position (while V_s – the velocity; and t_n – the times
    // of collision with n-th hailstone, are other unknowns).
    //
    // But if we introduce some redundancy by considering 5 hailstones instead,
    // we can reduce the system of equations to a linear one which is much easier to solve.
    //
    // So let’s assume that we actually have *at least* 5 hailstones in the input.

    let (h0, h1, h2, h3, h4) = (
        hailstones[0],
        hailstones[1],
        hailstones[2],
        hailstones[3],
        hailstones[4],
    );

    // Now, to find the linear equations that will help us solve the problem, let’s consider only
    // the x and y axes for a moment.
    //
    // For the n-th stone we can write the equations (the subscript _s means ‘solution’):
    //   x_n + v_xn · t_n = x_s + v_xs · t_n
    //   y_n + v_yn · t_n = y_s + v_ys · t_n
    // from which follows that the time t_n is:
    //   t_n = (x_s - x_1) / (v_x1 - v_xs)
    //       = (y_s - y_1) / (v_y1 - v_ys)
    // and if we multiply both sides by (v_(xy)1 - v(xy)s) and then multiply all the elements we
    // get:
    //
    //   x_s v_ys - y_s v_xs = x_s v_yn - y_s v_xn + x_n v_ys - y_n v_xs - x_n v_yn + y_n v_xn
    //
    // where the left-hand side (x_s v_ys - y_s v_xs) does not depend on the hailstone – it’s
    // constant. Thus we can now take the right side for n = 0 and n = 1 and set them equal…
    //
    // This gives us a linear equation of the form:
    //
    //   x_s (v_y1 - v_y0) + y_s (v_x0 - v_x1) + v_xs (y_0 - y_1) + v_ys (x_1 - x_0) =
    //     x_1 v_y1 - x_0 v_y0 + y_0 v_x0 - y_1 v_x1
    //
    // it has four unknowns (x_s, y_s, v_xs, v_ys) and so we need 4 equations of this type. So we
    // just pick 4 pairs of hailstones to do that.

    let make_row = |stone0: Hailstone, stone1: Hailstone| {
        RowVector4::new(
            stone1.velocity.y - stone0.velocity.y,
            stone0.velocity.x - stone1.velocity.x,
            stone0.position.y - stone1.position.y,
            stone1.position.x - stone0.position.x,
        )
    };
    let make_rh = |stone0: Hailstone, stone1: Hailstone| {
        stone1.position.x * stone1.velocity.y - stone0.position.x * stone0.velocity.y
            + stone0.position.y * stone0.velocity.x
            - stone1.position.y * stone1.velocity.x
    };

    let lh_coefficients = Matrix4::from_rows(&[
        make_row(h0, h1),
        make_row(h1, h2),
        make_row(h2, h3),
        make_row(h3, h4),
    ]);

    let rh = vector![
        make_rh(h0, h1),
        make_rh(h1, h2),
        make_rh(h2, h3),
        make_rh(h3, h4)
    ];

    // Solving these equations gives us (x_s, y_s, v_xs, v_ys):
    // equivalent to: let xy = invert(lh_coefficients) * rh;
    let xy = solve_linear_system(lh_coefficients, rh).context("singular matrix")?;
    let (x_s, y_s, v_xs) = (xy.x, xy.y, xy[2]);

    // What’s left is calculating z_s. Let’s just compute t_0, t_1, and then use:
    //   z_s + v_zs · t_n = z_n + v_zn · t_n.

    let t0 = (x_s - h0.position.x) / (h0.velocity.x - v_xs);
    let t1 = (x_s - h1.position.x) / (h1.velocity.x - v_xs);

    let z_eq_lh = matrix![1., t0; 1., t1];
    let z_eq_rh = vector![
        h0.position.z + h0.velocity.z * t0,
        h1.position.z + h1.velocity.z * t1,
    ];

    // equivalent to: let z_s = (invert(z_eq_lh) * z_eq_rh).x;
    let z_s = solve_linear_system(z_eq_lh, z_eq_rh)
        .context("singular matrix")?
        .x;

    Ok((x_s + y_s + z_s).round() as u64)
}
