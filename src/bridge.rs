
pub fn gitlab_webhook_to_hangout_message(payload: ::gitlab::WebhookPayload) -> ::hangoutchat::Message {
    let prev_assignees: Vec<String> = payload.changes.assignees.previous.iter().map({ |ref x| format!("@{}", x.username) }).collect();
    let current_assignees: Vec<String> = payload.changes.assignees.current.iter().map({ |ref x| format!("@{}", x.username) }).collect();
    
    let txt = match (prev_assignees, current_assignees) {
        (ref prev, ref current) if prev != current => { 
            let current_assignees_str = if current.len() > 0 { current.join(",") } else { "no-one".to_string() };
            format!("<{}|*{}* #{} (\"{}\")> assigneed to *_{}_*.", payload.object_attributes.url, payload.project.name, payload.object_attributes.iid, payload.object_attributes.title, current_assignees_str)
        },
        _ => format!("Unexpected message payload: {:?}", payload)
    };

    let avatared_assignee = payload.changes.assignees.current.iter().find(|ref x| x.avatar_url.is_some());

    let sender = ::hangoutchat::Sender {
        name: Option::None, 
        displayName: Option::None, 
        avatarUrl: match avatared_assignee { Some(ref x) => x.avatar_url.clone(), None => Option::None }, 
        email: Option::None
    };
    let msg = ::hangoutchat::Message { text: txt, sender: sender };
    return msg;
}

#[cfg(test)]
mod tests {
    use ::serde_json;
    use ::gitlab;
    use std::fs::File;
    use std::io::Read;
    use super::*;

    #[test]
    fn gitlab_issue_to_hangout_message() {
        let mut file = File::open("./fixtures/gitlab-issue-event.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let webhook: gitlab::WebhookPayload = serde_json::from_str(&data).unwrap();
        let msg = gitlab_webhook_to_hangout_message(webhook);
        assert_eq!(msg.text, "<https://gitlab.com/mpapp-private/manuscripts-api/issues/1|*manuscripts-api* #1 (\"Create CI configuration for the project.\")> assigneed to *_@abarmawi_*.");
    }

    #[test]
    fn gitlab_pipeline_event_to_hangout_message() {
        let mut file = File::open("./fixtures/gitlab-pipeline-event.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let webhook: gitlab::WebhookPayload = serde_json::from_str(&data).unwrap();
        let msg = gitlab_webhook_to_hangout_message(webhook);
        assert_eq!(msg.text, "<https://gitlab.com/mpapp-private/manuscripts-api/issues/1|*manuscripts-api* #1 (\"Create CI configuration for the project.\")> assigneed to *_@abarmawi_*.");
    }
}