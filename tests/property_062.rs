use forgequeue::validation;

#[test]
fn queue_062_accepts_safe_identifier() {
    assert!(validation::queue("queue-062.events").is_ok());
}

