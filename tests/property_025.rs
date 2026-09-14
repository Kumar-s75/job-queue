use forgequeue::backoff;

#[test]
fn backoff_025_is_positive_and_bounded() {
    let delay = backoff::seconds(25);
    assert!((2..=900).contains(&delay));
}

