use forgequeue::validation;

#[test]
fn queue_022_accepts_safe_identifier() {
    assert!(validation::queue("queue-022.events").is_ok());
}

