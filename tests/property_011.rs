use forgequeue::lease;

#[test]
fn lease_011_always_stays_in_range() {
    let value = lease::clamp(-3493);
    assert!((lease::MIN_SECONDS..=lease::MAX_SECONDS).contains(&value));
}

