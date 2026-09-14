use forgequeue::validation;

#[test]
fn queue_047_accepts_safe_identifier() {
    assert!(validation::queue("queue-047.events").is_ok());
}

