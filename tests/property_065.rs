use forgequeue::backoff;

#[test]
fn backoff_065_is_positive_and_bounded() {
    let delay = backoff::seconds(65);
    assert!((2..=900).contains(&delay));
}

