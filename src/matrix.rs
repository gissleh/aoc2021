#[inline]
pub fn matrix_times_vector<const N: usize, const M: usize>(matrix: [[u64; M]; N], vector: [u64; N]) -> [u64; N] {
    let mut res = [0u64; N];
    for i in 0..N {
        res[i] = matrix[i].iter().enumerate().map(|(j, v)| *v * vector[j]).sum();
    }

    res
}