use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use axum::Json;
use hmac_sha256::HMAC;
use http::StatusCode;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use subtle::ConstantTimeEq;
use crate::json_error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum WebhookPayload {
    Published(RegistryPublished)
}

impl<S> FromRequest<S> for WebhookPayload
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request(req: Request, state: &S) -> anyhow::Result<Self, Self::Rejection> {

        // TODO: Do this more properly with some type of environment variable handler eg https://nrempel.com/handling-environment-variables-with-axum/
        let webhook_token = std::env::var("GITHUB_WEBHOOK_TOKEN").expect("GITHUB_WEBHOOK_TOKEN environment variable is required");

        let signature_sha256 = req
            .headers()
            .get("X-Hub-Signature-256")
            .and_then(|v| v.to_str().ok())
            .ok_or(json_error("Signature is missing."))?
            .strip_prefix("sha256=")
            .ok_or(json_error("Signature prefix is missing."))?;
        let signature = hex::decode(signature_sha256).map_err(|_| json_error("Malformed signature."))?;
        let body = Bytes::from_request(req, state)
            .await
            .map_err(|_| json_error("Error reading request body."))?;
        let mac = HMAC::mac(&body, webhook_token.as_bytes());
        if mac.ct_ne(&signature).into() {
            return Err(json_error("Bad signature."));
        }

        // TODO: Do this more properly with Json<T>
        let payload = serde_json::from_slice::<WebhookPayload>(&body)
            .map_err(|e| json_error(format!("Unable to parse webhook request body: {}", e)))?;

        Ok(payload)

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryPublished {
    pub(crate) registry_package: RegistryPackage,
    pub(crate) repository: RegistryPublishedRepository,
    pub(crate) sender: Sender,
    pub(crate) installation: Option<Installation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Installation {
    pub(crate) id: Option<i64>,
    pub(crate) node_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryPackage {
    pub(crate) id: Option<i64>,
    pub(crate) name: Option<String>,
    pub(crate) namespace: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) ecosystem: Option<String>,
    pub(crate) package_type: Option<String>,
    pub(crate) html_url: Option<String>,
    pub(crate) created_at: Option<String>,
    pub(crate) updated_at: Option<String>,
    pub(crate) owner: Sender,
    pub(crate) package_version: PackageVersion,
    pub(crate) registry: Registry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sender {
    pub(crate) login: Option<String>,
    pub(crate) id: Option<i64>,
    pub(crate) node_id: Option<String>,
    pub(crate) avatar_url: Option<String>,
    pub(crate) gravatar_id: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) html_url: Option<String>,
    pub(crate) followers_url: Option<String>,
    pub(crate) following_url: Option<String>,
    pub(crate) gists_url: Option<String>,
    pub(crate) starred_url: Option<String>,
    pub(crate) subscriptions_url: Option<String>,
    pub(crate) organizations_url: Option<String>,
    pub(crate) repos_url: Option<String>,
    pub(crate) events_url: Option<String>,
    pub(crate) received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub(crate) sender_type: Option<String>,
    pub(crate) user_view_type: Option<String>,
    pub(crate) site_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersion {
    pub(crate) id: Option<i64>,
    pub(crate) version: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) summary: Option<String>,
    pub(crate) body: Body,
    pub(crate) manifest: Option<String>,
    pub(crate) html_url: Option<String>,
    pub(crate) target_commitish: Option<String>,
    pub(crate) target_oid: Option<String>,
    pub(crate) created_at: Option<String>,
    pub(crate) updated_at: Option<String>,
    pub(crate) metadata: Vec<Option<Value>>,
    pub(crate) container_metadata: ContainerMetadata,
    pub(crate) package_files: Vec<Option<Value>>,
    pub(crate) installation_command: Option<String>,
    pub(crate) package_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    pub(crate) repository: BodyRepository,
    pub(crate) info: Info,
    pub(crate) attributes: Attributes,
    #[serde(rename = "_formatted")]
    pub(crate) formatted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attributes {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    #[serde(rename = "type")]
    pub(crate) info_type: Option<String>,
    pub(crate) oid: Option<String>,
    pub(crate) mode: Option<i64>,
    pub(crate) name: Option<String>,
    pub(crate) path: Option<String>,
    pub(crate) size: Option<Value>,
    pub(crate) collection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyRepository {
    pub(crate) repository: RepositoryRepository,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryRepository {
    pub(crate) id: Option<i64>,
    pub(crate) name: Option<String>,
    pub(crate) owner_id: Option<i64>,
    pub(crate) parent_id: Option<Value>,
    pub(crate) sandbox: Option<Value>,
    pub(crate) updated_at: Option<String>,
    pub(crate) created_at: Option<String>,
    pub(crate) public: bool,
    pub(crate) description: Option<String>,
    pub(crate) homepage: Option<Value>,
    pub(crate) source_id: Option<i64>,
    pub(crate) public_push: Option<Value>,
    pub(crate) disk_usage: Option<i64>,
    pub(crate) locked: bool,
    pub(crate) pushed_at: Option<String>,
    pub(crate) watcher_count: Option<i64>,
    pub(crate) public_fork_count: Option<i64>,
    pub(crate) primary_language_name_id: Option<i64>,
    pub(crate) has_issues: bool,
    pub(crate) has_wiki: bool,
    pub(crate) has_downloads: bool,
    pub(crate) raw_data: RawData,
    pub(crate) organization_id: Option<Value>,
    pub(crate) disabled_at: Option<Value>,
    pub(crate) disabled_by: Option<Value>,
    pub(crate) disabling_reason: Option<Value>,
    pub(crate) health_status: Option<Value>,
    pub(crate) pushed_at_usec: Option<i64>,
    pub(crate) active: bool,
    pub(crate) reflog_sync_enabled: bool,
    pub(crate) made_public_at: Option<String>,
    pub(crate) user_hidden: Option<i64>,
    pub(crate) maintained: bool,
    pub(crate) template: bool,
    pub(crate) owner_login: Option<String>,
    pub(crate) world_writable_wiki: bool,
    pub(crate) refset_updated_at: Option<String>,
    pub(crate) disabling_detail: Option<Value>,
    pub(crate) archived_at: Option<Value>,
    pub(crate) deleted_at: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawData {
    pub(crate) data: Data,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub(crate) created_by_user_id: Option<i64>,
    pub(crate) primary_language_name: Option<String>,
    pub(crate) completed_onboarding_tasks: Vec<Option<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerMetadata {
    pub(crate) tag: Tag,
    pub(crate) labels: Labels,
    pub(crate) manifest: Manifest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Labels {
    pub(crate) description: Option<String>,
    pub(crate) source: Option<String>,
    pub(crate) revision: Option<String>,
    pub(crate) image_url: Option<String>,
    pub(crate) licenses: Option<String>,
    pub(crate) all_labels: AllLabels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllLabels {
    #[serde(rename = "github.internal.platforms")]
    pub(crate) github_internal_platforms: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub(crate) digest: Option<String>,
    pub(crate) media_type: Option<String>,
    pub(crate) uri: Option<String>,
    pub(crate) size: Option<i64>,
    pub(crate) config: Config,
    pub(crate) layers: Vec<Option<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub(crate) digest: Option<String>,
    pub(crate) media_type: Option<String>,
    pub(crate) size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub(crate) name: Option<String>,
    pub(crate) digest: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub(crate) about_url: Option<String>,
    pub(crate) name: Option<String>,
    #[serde(rename = "type")]
    pub(crate) registry_type: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) vendor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryPublishedRepository {
    pub(crate) id: Option<i64>,
    pub(crate) node_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) full_name: Option<String>,
    pub(crate) private: bool,
    pub(crate) owner: Sender,
    pub(crate) html_url: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) fork: bool,
    pub(crate) url: Option<String>,
    pub(crate) forks_url: Option<String>,
    pub(crate) keys_url: Option<String>,
    pub(crate) collaborators_url: Option<String>,
    pub(crate) teams_url: Option<String>,
    pub(crate) hooks_url: Option<String>,
    pub(crate) issue_events_url: Option<String>,
    pub(crate) events_url: Option<String>,
    pub(crate) assignees_url: Option<String>,
    pub(crate) branches_url: Option<String>,
    pub(crate) tags_url: Option<String>,
    pub(crate) blobs_url: Option<String>,
    pub(crate) git_tags_url: Option<String>,
    pub(crate) git_refs_url: Option<String>,
    pub(crate) trees_url: Option<String>,
    pub(crate) statuses_url: Option<String>,
    pub(crate) languages_url: Option<String>,
    pub(crate) stargazers_url: Option<String>,
    pub(crate) contributors_url: Option<String>,
    pub(crate) subscribers_url: Option<String>,
    pub(crate) subscription_url: Option<String>,
    pub(crate) commits_url: Option<String>,
    pub(crate) git_commits_url: Option<String>,
    pub(crate) comments_url: Option<String>,
    pub(crate) issue_comment_url: Option<String>,
    pub(crate) contents_url: Option<String>,
    pub(crate) compare_url: Option<String>,
    pub(crate) merges_url: Option<String>,
    pub(crate) archive_url: Option<String>,
    pub(crate) downloads_url: Option<String>,
    pub(crate) issues_url: Option<String>,
    pub(crate) pulls_url: Option<String>,
    pub(crate) milestones_url: Option<String>,
    pub(crate) notifications_url: Option<String>,
    pub(crate) labels_url: Option<String>,
    pub(crate) releases_url: Option<String>,
    pub(crate) deployments_url: Option<String>,
    pub(crate) created_at: Option<String>,
    pub(crate) updated_at: Option<String>,
    pub(crate) pushed_at: Option<String>,
    pub(crate) git_url: Option<String>,
    pub(crate) ssh_url: Option<String>,
    pub(crate) clone_url: Option<String>,
    pub(crate) svn_url: Option<String>,
    pub(crate) homepage: Option<Value>,
    pub(crate) size: Option<i64>,
    pub(crate) stargazers_count: Option<i64>,
    pub(crate) watchers_count: Option<i64>,
    pub(crate) language: Option<String>,
    pub(crate) has_issues: bool,
    pub(crate) has_projects: bool,
    pub(crate) has_downloads: bool,
    pub(crate) has_wiki: bool,
    pub(crate) has_pages: bool,
    pub(crate) has_discussions: bool,
    pub(crate) forks_count: Option<i64>,
    pub(crate) mirror_url: Option<Value>,
    pub(crate) archived: bool,
    pub(crate) disabled: bool,
    pub(crate) open_issues_count: Option<i64>,
    pub(crate) license: Option<Value>,
    pub(crate) allow_forking: bool,
    pub(crate) is_template: bool,
    pub(crate) web_commit_signoff_required: bool,
    pub(crate) topics: Vec<Option<Value>>,
    pub(crate) visibility: Option<String>,
    pub(crate) forks: Option<i64>,
    pub(crate) open_issues: Option<i64>,
    pub(crate) watchers: Option<i64>,
    pub(crate) default_branch: Option<String>,
}