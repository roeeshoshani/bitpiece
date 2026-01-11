/// Common test utilities shared across test modules.

/// Expects the provided closure to panic. If it doesn't panic, this function panics.
pub fn expect_panic<F: FnOnce() + std::panic::UnwindSafe>(f: F) {
    let result = std::panic::catch_unwind(f);
    result.expect_err("expected the code to panic");
}

/// Expects the provided closure to panic with a specific message substring.
#[allow(dead_code)]
pub fn expect_panic_with_message<F: FnOnce() + std::panic::UnwindSafe>(
    f: F,
    expected_substr: &str,
) {
    let result = std::panic::catch_unwind(f);
    match result {
        Ok(_) => panic!("expected the code to panic"),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                panic!("panic payload was not a string");
            };
            assert!(
                msg.contains(expected_substr),
                "expected panic message to contain '{}', but got '{}'",
                expected_substr,
                msg
            );
        }
    }
}
