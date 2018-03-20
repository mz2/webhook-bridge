pub fn gitlab_issue_to_hangout_message(payload: ::gitlab::IssuePayload) -> ::hangoutchat::Message {
    let (txt, sender) = if let Some(assignees) = payload.changes.assignees {
        let prev_assignees: Vec<String> = assignees.previous.iter().map({ |ref x| format!("@{}", x.username) }).collect();
        let current_assignees: Vec<String> = assignees.current.iter().map({ |ref x| format!("@{}", x.username) }).collect();
        
        let txt = match (prev_assignees, current_assignees) {
            (ref prev, ref current) if prev != current => { 
                let current_assignees_str = if current.len() > 0 { current.join(",") } else { "no-one".to_string() };
                format!("<{}|*{}* #{} (\"{}\")> assigneed to *_{}_*.", payload.object_attributes.url, payload.project.name, payload.object_attributes.iid, payload.object_attributes.title, current_assignees_str)
            },
            _ => { 
                format!("Unexpected assignee combination:\n\n```{:?}```", assignees).to_string()
            }
        };

        let avatared_assignee = assignees.current.iter().find(|ref x| x.avatar_url.is_some());
        let sender = ::hangoutchat::Sender {
            name: Option::None, 
            displayName: Option::None, 
            avatarUrl: match avatared_assignee { Some(ref x) => x.avatar_url.clone(), None => Option::None }, 
            email: Option::None
        };
        (txt, Some(sender))
    }
    else {
        (format!("Unhandled message payload:\n\n```{:?}```", payload).to_string(), Option::None)
    };

    
    let msg = ::hangoutchat::Message { text: txt, sender: sender };
    return msg;
}

pub fn gitlab_pipeline_event_to_hangout_message(payload: ::gitlab::PipelineEventPayload) -> ::hangoutchat::Message {
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

pub fn gitlab_merge_request_event_to_hangout_message(payload: ::gitlab::MergeRequestEventPayload) -> ::hangoutchat::Message {
    let txt = if let Some(assignee) = payload.changes.assignee {
        format!("<{}|{} #*{}*> assigneed to *_{}_*.\n{}", 
            payload.object_attributes.url,
                        payload.project.name,
                        payload.object_attributes.iid,
                        if let Some(current_assignee) = assignee.current { current_assignee.username } else { "(Unknown)".to_string() },
                        payload.object_attributes.title)
    } else if let Some(title) = payload.changes.title {
        format!("<{}|*{}* #*{}*> title changed:\n{}.", 
            payload.object_attributes.url,
                        payload.project.name,
                        payload.object_attributes.iid,
                        title.current)
    } else {
        format!("Unexpected message payload: {:?}", payload)
    };

    let msg = ::hangoutchat::Message { text: txt, sender: Option::None };
    return msg;
}

#[cfg(test)]
mod tests {
    use ::serde_json;
    use ::gitlab;
    use std::io::Read;
    use std::fs::File;
    use super::*;

    #[test]
    fn test_issue_to_hangout_message_conversion() {
        let mut file = File::open("./fixtures/gitlab-issue-event.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let webhook: gitlab::IssuePayload = serde_json::from_str(&data).unwrap();
        let msg = gitlab_issue_to_hangout_message(webhook);
        assert_eq!(msg.text, "<https://gitlab.com/mpapp-private/manuscripts-api/issues/1|*manuscripts-api* #1 (\"Create CI configuration for the project.\")> assigneed to *_@abarmawi_*.");
    }

    #[test]
    fn test_pipeline_event_to_hangout_message_conversion() {
        let mut file = File::open("./fixtures/gitlab-pipeline-event.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let webhook: gitlab::PipelineEventPayload = serde_json::from_str(&data).unwrap();
        let msg = gitlab_pipeline_event_to_hangout_message(webhook);
        assert_eq!(msg.text, "*Pipeline failed* for *<https://gitlab.com/mpapp-private/manuscripts-api|manuscripts-api>* <https://gitlab.com/mpapp-private/manuscripts-api/commit/19c4e55aea4773b1dafd7c34e326a19448160e9c|19c4e55aea47> (_<https://gitlab.com/mpapp-private/manuscripts-api/tree/1-create-ci-configuration-for-the-project|1-create-ci-configuration-for-the-project>_)\nA potentially functional first stab at docker-in-docker running of tests?\n");
    }

    #[test]
    fn test_merge_request_event_to_hangout_message_conversion() {
        let mut file = File::open("./fixtures/gitlab-merge-request-event.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let webhook: gitlab::MergeRequestEventPayload = serde_json::from_str(&data).unwrap();
        let msg = gitlab_merge_request_event_to_hangout_message(webhook);
        assert_eq!(msg.text, "<https://gitlab.com/mpapp-private/manuscripts-api/merge_requests/64|manuscripts-api #*64*> assigneed to *_alf-eaton_*.\nResolve \"change the verify email request from `GET` to `POST` & edit the verify email link in the sent email\"");
    }

    #[test]
    fn test_merge_request_event_title_update_to_hangout_message_conversion() {
        let mut file = File::open("./fixtures/gitlab-merge-request-event-title-update.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let webhook: gitlab::MergeRequestEventPayload = serde_json::from_str(&data).unwrap();
        let msg = gitlab_merge_request_event_to_hangout_message(webhook);
        assert_eq!(msg.text, "<https://gitlab.com/mpapp-private/manuscripts-api/merge_requests/64|*manuscripts-api* #*64*> title changed:\nResolve \"change the verify email request from `GET` to `POST` & edit the verify email link in the sent email\".");
    }
}