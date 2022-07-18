use crate::helpers::spawn_app;

#[actix_rt::test]
async fn an_error_flash_message_is_set_on_failure() {
    // Arrange
    let app = spawn_app().await;
    // Act
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password"
    });
    let response = app.post_login(&login_body).await;
    let flash_cookie = response.cookies().find(|c| c.name() == "_flash").unwrap();
    // Assert
    assert_eq!(flash_cookie.value(), "Authentication failed.");
    assert_is_redirect_to(&response, "/login");
    // Act - Part 2
    let _ = app.get_login_html().await;
    // Act - Part 3 - Reload the login page
    let html_page = app.get_login_html().await;
    assert!(!html_page.contains(r#"<p><i>Authentication failed.</i></p>"#));
}

// Helper function - this check is needed several times through out our tests
pub fn assert_is_redirect_to(response: &reqwest::Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), location);
}