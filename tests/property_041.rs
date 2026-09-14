use forgequeue::lease;

#[test]
fn lease_041_always_stays_in_range() {
    let value = lease::clamp(617);
    assert!((lease::MIN_SECONDS..=lease::MAX_SECONDS).contains(&value));
}

