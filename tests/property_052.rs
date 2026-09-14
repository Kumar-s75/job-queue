use forgequeue::validation;

#[test]
fn queue_052_accepts_safe_identifier() {
    assert!(validation::queue("queue-052.events").is_ok());
}

