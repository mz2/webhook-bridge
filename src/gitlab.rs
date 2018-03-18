#[derive(Deserialize, Debug)]
pub struct WebhookPayload {
    pub object_kind: String,
    pub project: ProjectPayload,
    pub object_attributes: ObjectAttributesPayload,
    pub changes: ChangesPayload
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
    pub homepage: String
}

#[derive(Deserialize, Debug)]
pub struct ObjectAttributesPayload {
    pub author_id: i64,
    pub iid: i64,
    pub state: String,
    pub title: String
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
    pub homepage: String
}
