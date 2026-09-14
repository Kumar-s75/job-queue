use forgequeue::backoff;

#[test]
fn backoff_060_is_positive_and_bounded() {
    let delay = backoff::seconds(60);
    assert!((2..=900).contains(&delay));
}

