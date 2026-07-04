pub fn reduce_text(value: u64) -> String {
    let text = if value > 1_000_000_000_000 {
        let adjusted_value = (value / 10_000_000_000) as f64 / 100.0;
        format!("{:.6} T", adjusted_value.to_string())
    } else if value > 1_000_000_000 {
        let adjusted_value = (value / 10_000_000) as f64 / 100.0;
        format!("{:.6} B", adjusted_value.to_string())
    } else if value > 1_000_000 {
        let adjusted_value = (value / 10_000) as f64 / 100.0;
        format!("{:.6} M", adjusted_value.to_string())
    } else {
        format!("{}", value)
    };
    text
}
