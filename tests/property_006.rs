use forgequeue::lease;

#[test]
fn lease_006_always_stays_in_range() {
    let value = lease::clamp(-4178);
    assert!((lease::MIN_SECONDS..=lease::MAX_SECONDS).contains(&value));
}

