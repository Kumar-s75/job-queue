use forgequeue::validation;

#[test]
fn queue_042_accepts_safe_identifier() {
    assert!(validation::queue("queue-042.events").is_ok());
}

