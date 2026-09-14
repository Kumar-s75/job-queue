use forgequeue::backoff;

#[test]
fn backoff_030_is_positive_and_bounded() {
    let delay = backoff::seconds(30);
    assert!((2..=900).contains(&delay));
}

