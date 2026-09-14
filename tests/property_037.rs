use forgequeue::validation;

#[test]
fn queue_037_accepts_safe_identifier() {
    assert!(validation::queue("queue-037.events").is_ok());
}

