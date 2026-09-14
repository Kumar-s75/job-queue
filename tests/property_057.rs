use forgequeue::validation;

#[test]
fn queue_057_accepts_safe_identifier() {
    assert!(validation::queue("queue-057.events").is_ok());
}

