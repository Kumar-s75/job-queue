use forgequeue::lease;

#[test]
fn lease_016_always_stays_in_range() {
    let value = lease::clamp(-2808);
    assert!((lease::MIN_SECONDS..=lease::MAX_SECONDS).contains(&value));
}

