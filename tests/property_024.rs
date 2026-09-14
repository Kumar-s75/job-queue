use forgequeue::priority;

#[test]
fn priority_024_range_contains_default() {
    assert!(priority::MIN <= 0 && priority::MAX >= 0);
    assert!(priority::MIN < priority::MAX);
}

