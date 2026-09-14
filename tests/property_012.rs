use forgequeue::validation;

#[test]
fn queue_012_accepts_safe_identifier() {
    assert!(validation::queue("queue-012.events").is_ok());
}

