use forgequeue::backoff;

#[test]
fn backoff_050_is_positive_and_bounded() {
    let delay = backoff::seconds(50);
    assert!((2..=900).contains(&delay));
}

