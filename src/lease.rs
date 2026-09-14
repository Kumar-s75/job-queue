pub const MIN_SECONDS:i64=5;
pub const MAX_SECONDS:i64=3600;
pub fn clamp(seconds:i64)->i64{seconds.clamp(MIN_SECONDS,MAX_SECONDS)}
