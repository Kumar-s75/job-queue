use forgequeue::backoff;

#[test]
fn backoff_045_is_positive_and_bounded() {
    let delay = backoff::seconds(45);
    assert!((2..=900).contains(&delay));
}

