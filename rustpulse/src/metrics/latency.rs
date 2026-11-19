pub fn percentile(values: &[f64], percentile: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    let mut sorted = values.to_vec();

    sorted.sort_by(|a, b| a.total_cmp(b));

    let p = percentile.clamp(0.0, 100.0);

    let index = ((p / 100.0) * sorted.len() as f64).ceil() as usize;

    let index = index.saturating_sub(1).min(sorted.len() - 1);

    sorted[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_p50() {
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];

        assert_eq!(percentile(&values, 50.0), 30.0);
    }

    #[test]
    fn calculates_p95() {
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];

        assert_eq!(percentile(&values, 95.0), 50.0);
    }

    #[test]
    fn handles_empty_values() {
        let values: Vec<f64> = vec![];

        assert_eq!(percentile(&values, 95.0), 0.0);
    }
}
