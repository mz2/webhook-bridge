
pub fn gitlab_issue_to_hangout_message(payload: ::gitlab::WebhookIssuePayload) -> ::hangoutchat::Message {
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
    let msg = ::hangoutchat::Message { text: txt, sender: Some(sender) };
    return msg;
}

pub fn gitlab_pipeline_event_to_hangout_message(payload: ::gitlab::WebhookPipelineEventPayload) -> ::hangoutchat::Message {
    let mut truncated_sha = payload.commit.id.clone();
    truncated_sha.truncate(12);
    
    let txt = format!("*Pipeline {}* for *<{}|{}>* <{}|{}> (_<{}/tree/{}|{}>_)\n{}", 
                      payload.object_attributes.status, 
                      payload.project.web_url, 
                      payload.project.name, 
                      payload.commit.url, 
                      truncated_sha, 
                      payload.project.web_url,
                      payload.object_attributes.git_ref,
                      payload.object_attributes.git_ref, 
                      payload.commit.message);
    let msg = ::hangoutchat::Message { text: txt, sender: Option::None };
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
    fn test_issue_to_hangout_message_conversion() {
        let mut file = File::open("./fixtures/gitlab-issue-event.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let webhook: gitlab::WebhookIssuePayload = serde_json::from_str(&data).unwrap();
        let msg = gitlab_issue_to_hangout_message(webhook);
        assert_eq!(msg.text, "<https://gitlab.com/mpapp-private/manuscripts-api/issues/1|*manuscripts-api* #1 (\"Create CI configuration for the project.\")> assigneed to *_@abarmawi_*.");
    }

    #[test]
    fn test_pipeline_event_to_hangout_message_conversion() {
        let mut file = File::open("./fixtures/gitlab-pipeline-event.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let webhook: gitlab::WebhookPipelineEventPayload = serde_json::from_str(&data).unwrap();
        let msg = gitlab_pipeline_event_to_hangout_message(webhook);
        assert_eq!(msg.text, "*Pipeline failed* for *<https://gitlab.com/mpapp-private/manuscripts-api|manuscripts-api>* <https://gitlab.com/mpapp-private/manuscripts-api/commit/19c4e55aea4773b1dafd7c34e326a19448160e9c|A potentially functional first stab at docker-in-docker running of tests?\n>");
    }
}