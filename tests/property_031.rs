use forgequeue::lease;

#[test]
fn lease_031_always_stays_in_range() {
    let value = lease::clamp(-753);
    assert!((lease::MIN_SECONDS..=lease::MAX_SECONDS).contains(&value));
}

