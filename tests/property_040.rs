use forgequeue::backoff;

#[test]
fn backoff_040_is_positive_and_bounded() {
    let delay = backoff::seconds(40);
    assert!((2..=900).contains(&delay));
}

