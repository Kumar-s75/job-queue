use forgequeue::backoff;

#[test]
fn backoff_010_is_positive_and_bounded() {
    let delay = backoff::seconds(10);
    assert!((2..=900).contains(&delay));
}

