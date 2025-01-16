use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum WebhookPayload {
    Published(RegistryPublished)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryPublished {
    #[serde(rename = "registry_package")]
    pub registry_package: RegistryPackage,
    pub repository: Repository3,
    pub sender: Sender,
    pub installation: Installation,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryPackage {
    pub id: i64,
    pub name: String,
    pub namespace: String,
    pub description: String,
    pub ecosystem: String,
    #[serde(rename = "package_type")]
    pub package_type: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub owner: Owner,
    #[serde(rename = "package_version")]
    pub package_version: PackageVersion,
    pub registry: Registry,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "user_view_type")]
    pub user_view_type: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageVersion {
    pub id: i64,
    pub version: String,
    pub name: String,
    pub description: String,
    pub summary: String,
    pub body: Body,
    pub manifest: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "target_commitish")]
    pub target_commitish: String,
    #[serde(rename = "target_oid")]
    pub target_oid: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub metadata: Vec<Value>,
    #[serde(rename = "container_metadata")]
    pub container_metadata: ContainerMetadata,
    #[serde(rename = "package_files")]
    pub package_files: Vec<Value>,
    #[serde(rename = "installation_command")]
    pub installation_command: String,
    #[serde(rename = "package_url")]
    pub package_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub repository: Repository,
    pub info: Info,
    pub attributes: Attributes,
    #[serde(rename = "_formatted")]
    pub formatted: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub repository: Repository2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository2 {
    pub id: i64,
    pub name: String,
    #[serde(rename = "owner_id")]
    pub owner_id: i64,
    #[serde(rename = "parent_id")]
    pub parent_id: Value,
    pub sandbox: Value,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub public: bool,
    pub description: String,
    pub homepage: Value,
    #[serde(rename = "source_id")]
    pub source_id: i64,
    #[serde(rename = "public_push")]
    pub public_push: Value,
    #[serde(rename = "disk_usage")]
    pub disk_usage: i64,
    pub locked: bool,
    #[serde(rename = "pushed_at")]
    pub pushed_at: String,
    #[serde(rename = "watcher_count")]
    pub watcher_count: i64,
    #[serde(rename = "public_fork_count")]
    pub public_fork_count: i64,
    #[serde(rename = "primary_language_name_id")]
    pub primary_language_name_id: i64,
    #[serde(rename = "has_issues")]
    pub has_issues: bool,
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    #[serde(rename = "has_downloads")]
    pub has_downloads: bool,
    #[serde(rename = "raw_data")]
    pub raw_data: RawData,
    #[serde(rename = "organization_id")]
    pub organization_id: Value,
    #[serde(rename = "disabled_at")]
    pub disabled_at: Value,
    #[serde(rename = "disabled_by")]
    pub disabled_by: Value,
    #[serde(rename = "disabling_reason")]
    pub disabling_reason: Value,
    #[serde(rename = "health_status")]
    pub health_status: Value,
    #[serde(rename = "pushed_at_usec")]
    pub pushed_at_usec: i64,
    pub active: bool,
    #[serde(rename = "reflog_sync_enabled")]
    pub reflog_sync_enabled: bool,
    #[serde(rename = "made_public_at")]
    pub made_public_at: String,
    #[serde(rename = "user_hidden")]
    pub user_hidden: i64,
    pub maintained: bool,
    pub template: bool,
    #[serde(rename = "owner_login")]
    pub owner_login: String,
    #[serde(rename = "world_writable_wiki")]
    pub world_writable_wiki: bool,
    #[serde(rename = "refset_updated_at")]
    pub refset_updated_at: String,
    #[serde(rename = "disabling_detail")]
    pub disabling_detail: Value,
    #[serde(rename = "archived_at")]
    pub archived_at: Value,
    #[serde(rename = "deleted_at")]
    pub deleted_at: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawData {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "created_by_user_id")]
    pub created_by_user_id: i64,
    #[serde(rename = "primary_language_name")]
    pub primary_language_name: String,
    #[serde(rename = "completed_onboarding_tasks")]
    pub completed_onboarding_tasks: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(rename = "type")]
    pub type_field: String,
    pub oid: String,
    pub mode: i64,
    pub name: String,
    pub path: String,
    pub size: Value,
    pub collection: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerMetadata {
    pub tag: Tag,
    pub labels: Labels,
    pub manifest: Manifest,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub digest: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Labels {
    pub description: String,
    pub source: String,
    pub revision: String,
    #[serde(rename = "image_url")]
    pub image_url: String,
    pub licenses: String,
    #[serde(rename = "all_labels")]
    pub all_labels: AllLabels,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllLabels {
    #[serde(rename = "github.internal.platforms")]
    pub github_internal_platforms: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub digest: String,
    #[serde(rename = "media_type")]
    pub media_type: String,
    pub uri: String,
    pub size: i64,
    pub config: Config,
    pub layers: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub digest: String,
    #[serde(rename = "media_type")]
    pub media_type: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registry {
    #[serde(rename = "about_url")]
    pub about_url: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub vendor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository3 {
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub private: bool,
    pub owner: Owner2,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "deployments_url")]
    pub deployments_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "pushed_at")]
    pub pushed_at: String,
    #[serde(rename = "git_url")]
    pub git_url: String,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "clone_url")]
    pub clone_url: String,
    #[serde(rename = "svn_url")]
    pub svn_url: String,
    pub homepage: Value,
    pub size: i64,
    #[serde(rename = "stargazers_count")]
    pub stargazers_count: i64,
    #[serde(rename = "watchers_count")]
    pub watchers_count: i64,
    pub language: String,
    #[serde(rename = "has_issues")]
    pub has_issues: bool,
    #[serde(rename = "has_projects")]
    pub has_projects: bool,
    #[serde(rename = "has_downloads")]
    pub has_downloads: bool,
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    #[serde(rename = "has_pages")]
    pub has_pages: bool,
    #[serde(rename = "has_discussions")]
    pub has_discussions: bool,
    #[serde(rename = "forks_count")]
    pub forks_count: i64,
    #[serde(rename = "mirror_url")]
    pub mirror_url: Value,
    pub archived: bool,
    pub disabled: bool,
    #[serde(rename = "open_issues_count")]
    pub open_issues_count: i64,
    pub license: Value,
    #[serde(rename = "allow_forking")]
    pub allow_forking: bool,
    #[serde(rename = "is_template")]
    pub is_template: bool,
    #[serde(rename = "web_commit_signoff_required")]
    pub web_commit_signoff_required: bool,
    pub topics: Vec<Value>,
    pub visibility: String,
    pub forks: i64,
    #[serde(rename = "open_issues")]
    pub open_issues: i64,
    pub watchers: i64,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner2 {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "user_view_type")]
    pub user_view_type: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "user_view_type")]
    pub user_view_type: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Installation {
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
}
