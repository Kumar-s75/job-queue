use forgequeue::lease;

#[test]
fn lease_061_always_stays_in_range() {
    let value = lease::clamp(3357);
    assert!((lease::MIN_SECONDS..=lease::MAX_SECONDS).contains(&value));
}

