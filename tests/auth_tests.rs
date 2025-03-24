use rusty_store::auth::Auth;

#[test]
fn test_add_user() {
    let mut auth = Auth::new();
    auth.add_user("admin".to_string(), "password".to_string());
    assert!(auth.authenticate("admin", "password"));
}

#[test]
fn test_authenticate_valid_user() {
    let mut auth = Auth::new();
    auth.add_user("admin".to_string(), "password".to_string());
    assert!(auth.authenticate("admin", "password"));
}

#[test]
fn test_authenticate_invalid_user() {
    let mut auth = Auth::new();
    auth.add_user("admin".to_string(), "password".to_string());
    assert!(!auth.authenticate("admin", "wrong_password"));
    assert!(!auth.authenticate("unknown_user","password"));
}
