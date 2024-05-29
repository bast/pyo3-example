use pyo3::prelude::*;
use rayon::prelude::*;

/// some docstring
#[pyfunction]
pub fn perfect_numbers(n: usize) -> Vec<usize> {
    (2..n)
        .into_par_iter()
        .filter(|&n| number_is_perfect(n))
        .collect()
}

#[pymodule]
fn perfect(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_function(wrap_pyfunction!(perfect_numbers, m)?)?;

    Ok(())
}

// fn sum_proper_divisors(n: usize) -> usize {
//     let mut s = 0;
//     for i in 1..n {
//         if n % i == 0 {
//             s += i;
//         }
//     }
//     s
// }

fn sum_proper_divisors_faster(n: usize) -> usize {
    let m = (n as f64).sqrt() as usize;
    let mut s = 0;
    for i in 1..(m + 1) {
        if n % i == 0 {
            s += i;
            s += n / i;
        }
    }
    s - n
}

fn number_is_perfect(n: usize) -> bool {
    sum_proper_divisors_faster(n) == n
}
