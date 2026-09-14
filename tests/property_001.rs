use forgequeue::lease;

#[test]
fn lease_001_always_stays_in_range() {
    let value = lease::clamp(-4863);
    assert!((lease::MIN_SECONDS..=lease::MAX_SECONDS).contains(&value));
}

