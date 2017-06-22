pub fn scalar_sum(a: f64, b: f64) -> f64 {
    a + b
}

pub fn vec_sum(v: &[f64]) -> f64 {
    v.iter().fold(0.0, |a, e| a + e)
}

pub fn vec_mean(v: &[f64]) -> f64 {
    v.iter()
        .enumerate()
        .fold(0.0, |m, (n, e)| m + (e - m) / (n + 1) as f64)
}

pub fn vec_cumsum(v: &[f64]) -> Vec<f64> {
    v.iter()
        .scan(0.0, |s, e| {
            *s += *e;
            Some(*s)
        })
        .collect()
}

pub fn vec_cumsum_mut(v: &[f64], out: &mut [f64]) {
    assert_eq!(v.len(), out.len());

    // Do the slice dance to avoid bound checking later
    let len = out.len();
    let v = &v[..len];
    let mut out = &mut out[..len];

    let mut acc = 0.0;
    for i in 0..len {
        acc += v[i];
        out[i] = acc;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn scalar_sum() {
        use super::scalar_sum;

        assert_eq!(scalar_sum(-10.0, 16.0), 6.0);
    }

    #[test]
    fn vec_sum() {
        use super::vec_sum;

        let v = vec![1.0, 2.0, 3.0, -10.0];
        assert_eq!(vec_sum(&v), -4.0);
    }

    #[test]
    fn vec_mean() {
        use super::vec_mean;

        let v1 = vec![1.0, 2.0, 3.0, -10.0];
        assert_eq!(vec_mean(&v1), -1.0);
        let v2 = vec![-1.0, 1.0, -1.0, 1.0];
        assert_eq!(vec_mean(&v2), 0.0);
    }

    #[test]
    fn vec_cumsum() {
        use super::vec_cumsum;

        let v = vec![1.0, 2.0, 3.0, -10.0];
        assert_eq!(vec_cumsum(&v), vec![1.0, 3.0, 6.0, -4.0]);
    }

    #[test]
    fn vec_cumsum_mut() {
        use super::vec_cumsum_mut;

        let v = vec![1.0, 2.0, 3.0, -10.0];
        let mut cs = vec![0.0; 4];
        vec_cumsum_mut(&v, &mut cs);
        assert_eq!(cs, vec![1.0, 3.0, 6.0, -4.0]);
    }
}
