pub fn seconds(attempt: i32) -> i64 { 2_i64.saturating_pow(attempt.clamp(1, 10) as u32).min(900) }
