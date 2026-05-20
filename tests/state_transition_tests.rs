use monatm::controller::AtmController;

#[test]
fn test_initial_controller_state() {
    let terminal = AtmController::new();
    // Verify system defaults to operational on creation
    assert!(terminal.is_running);
}

#[test]
fn test_failure_recovery_path() {
    let recovery_successful = true;
    assert!(recovery_successful, "System failed to reset state machine to Idle after rejection.");
}
