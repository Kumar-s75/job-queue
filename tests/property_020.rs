use forgequeue::backoff;

#[test]
fn backoff_020_is_positive_and_bounded() {
    let delay = backoff::seconds(20);
    assert!((2..=900).contains(&delay));
}

