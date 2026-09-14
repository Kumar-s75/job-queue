use forgequeue::validation;

#[test]
fn queue_007_accepts_safe_identifier() {
    assert!(validation::queue("queue-007.events").is_ok());
}

