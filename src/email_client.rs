use crate::domain::SubscriberEmail;
use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
pub struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String,
}

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    authorization_token: String,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail, authorization_token: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recepient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email", self.base_url);
        let request_body = SendEmailRequest {
            from: self.sender.as_ref().to_owned(),
            to: recepient.as_ref().to_owned(),
            subject: subject.to_owned(),
            html_body: html_content.to_owned(),
            text_body: text_content.to_owned(),
        };
        self.http_client
            .post(&url)
            .header("X-Postmark-Server-Token", &self.authorization_token)
            .json(&request_body)
            .send()
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{header, header_exists, path, method};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // Arrange
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(mock_server.uri(), sender, Faker.fake());
        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;
        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject = Sentence(1..2).fake::<String>();
        let content = Paragraph(1..10).fake::<String>();
        // Act
        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;
        // Assert
    }
}
