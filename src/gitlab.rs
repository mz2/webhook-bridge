use rocket::request::{self , FromRequest};
use rocket::Outcome::*;
use rocket::http::Status;
use rocket::{Request};

#[derive(Deserialize, Debug)]
pub struct WebhookIssuePayload {
    pub object_kind: String,
    pub project: ProjectPayload,
    pub object_attributes: IssueAttributesPayload,
    pub changes: ChangesPayload
}

#[derive(Deserialize, Debug)]
pub struct WebhookPipelineEventPayload {
    pub user: UserPayload,
    pub object_attributes: PipelineEventAttributesPayload,
    pub project: ProjectPayload,
    pub commit: CommitPayload,
    pub builds: Vec<BuildPayload>
}

#[derive(Deserialize, Debug)]
pub struct UserPayload {
    pub name: String,
    pub username: String,
    pub avatar_url: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct ProjectPayload {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub web_url: String,
    pub avatar_url: Option<String>,
    pub git_http_url: String,
    pub namespace: String,
    pub homepage: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct IssueAttributesPayload {
    pub author_id: i64,
    pub iid: i64,
    pub state: String,
    pub title: String,
    pub url: String
}

#[derive(Deserialize, Debug)]
pub struct AuthorPayload {
    pub name: String,
    pub email: String
}

#[derive(Deserialize, Debug)]
pub struct CommitPayload {
    pub id: String,
    pub message: String,
    pub url: String,
    pub author: AuthorPayload
}

#[derive(Deserialize, Debug)]
pub struct BuildPayload {
    id: i64,
    stage: String,
    name: String,
    status: String,
    when: String,
    manual: bool,
    user: UserPayload,
    runner: RunnerPayload,
    artifacts_file: Option<ArtifactsPayload>
}

#[derive(Deserialize, Debug)]
pub struct ArtifactsPayload {
    filename: Option<String>,
    size: i64
}

#[derive(Deserialize, Debug)]
pub struct RunnerPayload {
    pub id: i64,
    pub description: String,
    pub active: bool,
    pub is_shared: bool
}

#[derive(Deserialize, Debug)]
pub struct PipelineEventAttributesPayload {
    pub id: i64,
    #[serde(rename="ref")]
    pub git_ref: String,
    pub tag: bool,
    pub sha: String,
    pub before_sha: String,
    pub status: String,
    pub detailed_status: String,
    pub stages: Vec<String>,
    pub duration: i64
}

#[derive(Deserialize, Debug)]
pub struct ChangesPayload {
    pub assignees: AssigneesPayload,
    pub labels: LabelsPayload
}

#[derive(Deserialize, Debug)]
pub struct AssigneesPayload {
    pub previous: Vec<UserPayload>,
    pub current: Vec<UserPayload>
}

#[derive(Deserialize, Debug)]
pub struct LabelsPayload {
    pub previous: Vec<LabelPayload>,
    pub current: Vec<LabelPayload>
}

#[derive(Deserialize, Debug)]
pub struct LabelPayload {
    pub id: i64,
    pub title: String,
    pub color: String,
    pub project_id: i64, 
    pub description: String,
    #[serde(rename="type")]
    pub label_type: String
}

#[derive(Deserialize, Debug)]
pub struct RepositoryPayload {
    pub name: String,
    pub url: String,
    pub description: String,
    pub homepage: Option<String>
}

pub struct GitLabEventType {
    pub event_type: String
}

pub struct GitLabTokens {
    pub acceptable_tokens: Vec<String>
}

pub struct GitLabToken {
    pub token: String
}

impl<'a, 'r> FromRequest<'a, 'r> for GitLabToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        match request.headers().get("X-GitLab-Token").next() {
            Some(token) => {
                Success(GitLabToken { token: token.to_string() })
            },
            None => Failure((Status::BadRequest, ()))
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for GitLabEventType {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        match request.headers().get("X-GitLab-Event").next() {
            Some(event_type) => {
                Success(GitLabEventType { event_type: event_type.to_string() })
            },
            None => Failure((Status::BadRequest, ()))
        }
    }
}
