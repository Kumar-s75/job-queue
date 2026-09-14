use forgequeue::backoff;

#[test]
fn backoff_035_is_positive_and_bounded() {
    let delay = backoff::seconds(35);
    assert!((2..=900).contains(&delay));
}

