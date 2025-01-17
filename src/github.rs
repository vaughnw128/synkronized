use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum WebhookPayload {
    Published(RegistryPublished)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryPublished {
    pub(crate) registry_package: RegistryPackage,
    pub(crate) repository: RegistryPublishedRepository,
    pub(crate) sender: Sender,
    pub(crate) installation: Installation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Installation {
    pub(crate) id: i64,
    pub(crate) node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryPackage {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) namespace: String,
    pub(crate) description: String,
    pub(crate) ecosystem: String,
    pub(crate) package_type: String,
    pub(crate) html_url: String,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
    pub(crate) owner: Sender,
    pub(crate) package_version: PackageVersion,
    pub(crate) registry: Registry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sender {
    pub(crate) login: String,
    pub(crate) id: i64,
    pub(crate) node_id: String,
    pub(crate) avatar_url: String,
    pub(crate) gravatar_id: String,
    pub(crate) url: String,
    pub(crate) html_url: String,
    pub(crate) followers_url: String,
    pub(crate) following_url: String,
    pub(crate) gists_url: String,
    pub(crate) starred_url: String,
    pub(crate) subscriptions_url: String,
    pub(crate) organizations_url: String,
    pub(crate) repos_url: String,
    pub(crate) events_url: String,
    pub(crate) received_events_url: String,
    #[serde(rename = "type")]
    pub(crate) sender_type: String,
    pub(crate) user_view_type: String,
    pub(crate) site_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersion {
    pub(crate) id: i64,
    pub(crate) version: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) summary: String,
    pub(crate) body: Body,
    pub(crate) manifest: String,
    pub(crate) html_url: String,
    pub(crate) target_commitish: String,
    pub(crate) target_oid: String,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
    pub(crate) metadata: Vec<Option<serde_json::Value>>,
    pub(crate) container_metadata: ContainerMetadata,
    pub(crate) package_files: Vec<Option<serde_json::Value>>,
    pub(crate) installation_command: String,
    pub(crate) package_url: String,
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
    pub(crate) info_type: String,
    pub(crate) oid: String,
    pub(crate) mode: i64,
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) size: Option<serde_json::Value>,
    pub(crate) collection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyRepository {
    pub(crate) repository: RepositoryRepository,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryRepository {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) owner_id: i64,
    pub(crate) parent_id: Option<serde_json::Value>,
    pub(crate) sandbox: Option<serde_json::Value>,
    pub(crate) updated_at: String,
    pub(crate) created_at: String,
    pub(crate) public: bool,
    pub(crate) description: String,
    pub(crate) homepage: Option<serde_json::Value>,
    pub(crate) source_id: i64,
    pub(crate) public_push: Option<serde_json::Value>,
    pub(crate) disk_usage: i64,
    pub(crate) locked: bool,
    pub(crate) pushed_at: String,
    pub(crate) watcher_count: i64,
    pub(crate) public_fork_count: i64,
    pub(crate) primary_language_name_id: i64,
    pub(crate) has_issues: bool,
    pub(crate) has_wiki: bool,
    pub(crate) has_downloads: bool,
    pub(crate) raw_data: RawData,
    pub(crate) organization_id: Option<serde_json::Value>,
    pub(crate) disabled_at: Option<serde_json::Value>,
    pub(crate) disabled_by: Option<serde_json::Value>,
    pub(crate) disabling_reason: Option<serde_json::Value>,
    pub(crate) health_status: Option<serde_json::Value>,
    pub(crate) pushed_at_usec: i64,
    pub(crate) active: bool,
    pub(crate) reflog_sync_enabled: bool,
    pub(crate) made_public_at: String,
    pub(crate) user_hidden: i64,
    pub(crate) maintained: bool,
    pub(crate) template: bool,
    pub(crate) owner_login: String,
    pub(crate) world_writable_wiki: bool,
    pub(crate) refset_updated_at: String,
    pub(crate) disabling_detail: Option<serde_json::Value>,
    pub(crate) archived_at: Option<serde_json::Value>,
    pub(crate) deleted_at: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawData {
    pub(crate) data: Data,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub(crate) created_by_user_id: i64,
    pub(crate) primary_language_name: String,
    pub(crate) completed_onboarding_tasks: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerMetadata {
    pub(crate) tag: Tag,
    pub(crate) labels: Labels,
    pub(crate) manifest: Manifest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Labels {
    pub(crate) description: String,
    pub(crate) source: String,
    pub(crate) revision: String,
    pub(crate) image_url: String,
    pub(crate) licenses: String,
    pub(crate) all_labels: AllLabels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllLabels {
    #[serde(rename = "github.internal.platforms")]
    pub(crate) github_internal_platforms: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub(crate) digest: String,
    pub(crate) media_type: String,
    pub(crate) uri: String,
    pub(crate) size: i64,
    pub(crate) config: Config,
    pub(crate) layers: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub(crate) digest: String,
    pub(crate) media_type: String,
    pub(crate) size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub(crate) name: String,
    pub(crate) digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub(crate) about_url: String,
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) registry_type: String,
    pub(crate) url: String,
    pub(crate) vendor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryPublishedRepository {
    pub(crate) id: i64,
    pub(crate) node_id: String,
    pub(crate) name: String,
    pub(crate) full_name: String,
    pub(crate) private: bool,
    pub(crate) owner: Sender,
    pub(crate) html_url: String,
    pub(crate) description: String,
    pub(crate) fork: bool,
    pub(crate) url: String,
    pub(crate) forks_url: String,
    pub(crate) keys_url: String,
    pub(crate) collaborators_url: String,
    pub(crate) teams_url: String,
    pub(crate) hooks_url: String,
    pub(crate) issue_events_url: String,
    pub(crate) events_url: String,
    pub(crate) assignees_url: String,
    pub(crate) branches_url: String,
    pub(crate) tags_url: String,
    pub(crate) blobs_url: String,
    pub(crate) git_tags_url: String,
    pub(crate) git_refs_url: String,
    pub(crate) trees_url: String,
    pub(crate) statuses_url: String,
    pub(crate) languages_url: String,
    pub(crate) stargazers_url: String,
    pub(crate) contributors_url: String,
    pub(crate) subscribers_url: String,
    pub(crate) subscription_url: String,
    pub(crate) commits_url: String,
    pub(crate) git_commits_url: String,
    pub(crate) comments_url: String,
    pub(crate) issue_comment_url: String,
    pub(crate) contents_url: String,
    pub(crate) compare_url: String,
    pub(crate) merges_url: String,
    pub(crate) archive_url: String,
    pub(crate) downloads_url: String,
    pub(crate) issues_url: String,
    pub(crate) pulls_url: String,
    pub(crate) milestones_url: String,
    pub(crate) notifications_url: String,
    pub(crate) labels_url: String,
    pub(crate) releases_url: String,
    pub(crate) deployments_url: String,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
    pub(crate) pushed_at: String,
    pub(crate) git_url: String,
    pub(crate) ssh_url: String,
    pub(crate) clone_url: String,
    pub(crate) svn_url: String,
    pub(crate) homepage: Option<serde_json::Value>,
    pub(crate) size: i64,
    pub(crate) stargazers_count: i64,
    pub(crate) watchers_count: i64,
    pub(crate) language: String,
    pub(crate) has_issues: bool,
    pub(crate) has_projects: bool,
    pub(crate) has_downloads: bool,
    pub(crate) has_wiki: bool,
    pub(crate) has_pages: bool,
    pub(crate) has_discussions: bool,
    pub(crate) forks_count: i64,
    pub(crate) mirror_url: Option<serde_json::Value>,
    pub(crate) archived: bool,
    pub(crate) disabled: bool,
    pub(crate) open_issues_count: i64,
    pub(crate) license: Option<serde_json::Value>,
    pub(crate) allow_forking: bool,
    pub(crate) is_template: bool,
    pub(crate) web_commit_signoff_required: bool,
    pub(crate) topics: Vec<Option<serde_json::Value>>,
    pub(crate) visibility: String,
    pub(crate) forks: i64,
    pub(crate) open_issues: i64,
    pub(crate) watchers: i64,
    pub(crate) default_branch: String,
}