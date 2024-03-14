use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn you_must_be_logged_in_to_access_admin_dashboard() {
    let app = spawn_app().await;
    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn logout_clears_session_state() {
    let app = spawn_app().await;

    // Login
    let response = app.test_user.login(&app).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    // Follow the redirect
    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));

    // Logout
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    // Follow the redirect
    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"You have successfully logged out"#));

    // Attempt to access the admin dashboard again
    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");
}
