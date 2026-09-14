use forgequeue::validation;

#[test]
fn queue_002_accepts_safe_identifier() {
    assert!(validation::queue("queue-002.events").is_ok());
}

