use forgequeue::validation;

#[test]
fn queue_017_accepts_safe_identifier() {
    assert!(validation::queue("queue-017.events").is_ok());
}

