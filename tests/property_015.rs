use forgequeue::backoff;

#[test]
fn backoff_015_is_positive_and_bounded() {
    let delay = backoff::seconds(15);
    assert!((2..=900).contains(&delay));
}

