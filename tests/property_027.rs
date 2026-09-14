use forgequeue::validation;

#[test]
fn queue_027_accepts_safe_identifier() {
    assert!(validation::queue("queue-027.events").is_ok());
}

