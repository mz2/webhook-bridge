#[post("/gitlab/webhook", format = "application/json", data = "<payload>")]
fn convert_gitlab_webhook(payload: ::rocket_contrib::Json<::gitlab::WebhookPayload>) {
    let sender = ::hangoutchat::MessageSender {
        space: "AAAA0gIpT4c".to_string(),
        key: "AIzaSyDdI0hCZtE6vySjMm-WEfRq3CPzqKqqsHI".to_string(),
        token: "6LhzSUAX7eEjfR745yIHyydtoSXhlmD0PGKHlDTfOec%3D".to_string()
    };

    sender.send(::bridge::gitlab_webhook_to_hangout_message(payload.into_inner()));
}
