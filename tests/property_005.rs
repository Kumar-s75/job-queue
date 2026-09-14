use forgequeue::backoff;

#[test]
fn backoff_005_is_positive_and_bounded() {
    let delay = backoff::seconds(5);
    assert!((2..=900).contains(&delay));
}

