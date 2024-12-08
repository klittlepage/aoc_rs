pub mod matrix;
pub mod table;

#[must_use]
pub fn nth_difference_i64(input: &[i64], n: usize) -> Vec<i64> {
    let vals = input.iter();
    let next_vals = input.iter().skip(n);
    vals.zip(next_vals).map(|(cur, next)| next - cur).collect()
}

#[must_use]
pub fn all_positive_i64(input: &[i64]) -> bool {
    input.iter().all(|x| x.is_positive())
}

#[must_use]
pub fn all_negative_i64(input: &[i64]) -> bool {
    input.iter().all(|x| x.is_negative())
}
