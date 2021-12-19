use std::iter::Sum;
use num::Integer;

#[inline]
pub fn matrix_times_vector<T: Integer + Sum + Copy, const N: usize, const M: usize>(matrix: [[T; M]; N], vector: [T; N]) -> [T; N] {
    let mut res = [T::zero(); N];
    for i in 0..N {
        res[i] = matrix[i].iter()
            .enumerate()
            .map(|(j, v)| *v * vector[j])
            .sum();
    }

    res
}