use forgequeue::backoff;

#[test]
fn backoff_055_is_positive_and_bounded() {
    let delay = backoff::seconds(55);
    assert!((2..=900).contains(&delay));
}

