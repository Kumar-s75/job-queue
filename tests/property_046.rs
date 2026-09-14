use forgequeue::lease;

#[test]
fn lease_046_always_stays_in_range() {
    let value = lease::clamp(1302);
    assert!((lease::MIN_SECONDS..=lease::MAX_SECONDS).contains(&value));
}

