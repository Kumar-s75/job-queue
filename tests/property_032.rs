use forgequeue::validation;

#[test]
fn queue_032_accepts_safe_identifier() {
    assert!(validation::queue("queue-032.events").is_ok());
}

