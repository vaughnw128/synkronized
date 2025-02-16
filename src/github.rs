use std::collections::HashMap;
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
    /// An enterprise on GitHub. Webhook payloads contain the `enterprise` property when the
    /// webhook is configured
    /// on an enterprise account or an organization that's part of an enterprise account. For
    /// more information,
    /// see "[About enterprise
    /// accounts](https://docs.github.com/admin/overview/about-enterprise-accounts)."
    pub(crate) enterprise: Option<Enterprise>,
    /// The GitHub App installation. Webhook payloads contain the `installation` property when
    /// the event is configured
    /// for and sent to a GitHub App. For more information,
    /// see "[Using webhooks with GitHub
    /// Apps](https://docs.github.com/apps/creating-github-apps/registering-a-github-app/using-webhooks-with-github-apps)."
    pub(crate) installation: Option<SimpleInstallation>,
    /// A GitHub organization. Webhook payloads contain the `organization` property when the
    /// webhook is configured for an
    /// organization, or when the event occurs from activity in a repository owned by an
    /// organization.
    pub(crate) organization: Option<OrganizationSimple>,
    pub(crate) registry_package: RegistryPackage,
    /// The repository on GitHub where the event occurred. Webhook payloads contain the
    /// `repository` property
    /// when the event occurs from activity in a repository.
    pub(crate) repository: Option<Repository>,
    /// A GitHub user.
    pub(crate) sender: SenderClass,
}

/// An enterprise on GitHub. Webhook payloads contain the `enterprise` property when the
/// webhook is configured
/// on an enterprise account or an organization that's part of an enterprise account. For
/// more information,
/// see "[About enterprise
/// accounts](https://docs.github.com/admin/overview/about-enterprise-accounts)."
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enterprise {
    pub(crate) avatar_url: String,
    pub(crate) created_at: String,
    /// A short description of the enterprise.
    pub(crate) description: Option<String>,
    pub(crate) html_url: String,
    /// Unique identifier of the enterprise
    pub(crate) id: i64,
    /// The name of the enterprise.
    pub(crate) name: String,
    pub(crate) node_id: String,
    /// The slug url identifier for the enterprise.
    pub(crate) slug: String,
    pub(crate) updated_at: String,
    /// The enterprise's website URL.
    pub(crate) website_url: Option<String>,
}

/// The GitHub App installation. Webhook payloads contain the `installation` property when
/// the event is configured
/// for and sent to a GitHub App. For more information,
/// see "[Using webhooks with GitHub
/// Apps](https://docs.github.com/apps/creating-github-apps/registering-a-github-app/using-webhooks-with-github-apps)."
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleInstallation {
    /// The ID of the installation.
    pub(crate) id: i64,
    /// The global node ID of the installation.
    pub(crate) node_id: String,
}

/// A GitHub organization. Webhook payloads contain the `organization` property when the
/// webhook is configured for an
/// organization, or when the event occurs from activity in a repository owned by an
/// organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationSimple {
    pub(crate) avatar_url: String,
    pub(crate) description: String,
    pub(crate) events_url: String,
    pub(crate) hooks_url: String,
    pub(crate) id: i64,
    pub(crate) issues_url: String,
    pub(crate) login: String,
    pub(crate) members_url: String,
    pub(crate) node_id: String,
    pub(crate) public_members_url: String,
    pub(crate) repos_url: String,
    pub(crate) url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryPackage {
    pub(crate) created_at: String,
    pub(crate) description: String,
    pub(crate) ecosystem: String,
    pub(crate) html_url: String,
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) namespace: String,
    pub(crate) owner: RegistryPackageOwner,
    pub(crate) package_type: String,
    pub(crate) package_version: PackageVersion,
    pub(crate) registry: Registry,
    pub(crate) updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryPackageOwner {
    pub(crate) avatar_url: String,
    pub(crate) events_url: String,
    pub(crate) followers_url: String,
    pub(crate) following_url: String,
    pub(crate) gists_url: String,
    pub(crate) gravatar_id: String,
    pub(crate) html_url: String,
    pub(crate) id: i64,
    pub(crate) login: String,
    pub(crate) node_id: String,
    pub(crate) organizations_url: String,
    pub(crate) received_events_url: String,
    pub(crate) repos_url: String,
    pub(crate) site_admin: bool,
    pub(crate) starred_url: String,
    pub(crate) subscriptions_url: String,
    #[serde(rename = "type")]
    pub(crate) owner_type: String,
    pub(crate) url: String,
    pub(crate) user_view_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersion {
    pub(crate) author: Option<PackageVersionAuthor>,
    pub(crate) body: Option<Body>,
    pub(crate) body_html: Option<String>,
    pub(crate) container_metadata: Option<ContainerMetadata>,
    pub(crate) created_at: Option<String>,
    pub(crate) description: String,
    pub(crate) docker_metadata: Option<Vec<DockerMetadatum>>,
    pub(crate) draft: Option<bool>,
    pub(crate) html_url: String,
    pub(crate) id: i64,
    pub(crate) installation_command: String,
    pub(crate) manifest: Option<String>,
    pub(crate) metadata: Vec<HashMap<String, Option<Value>>>,
    pub(crate) name: String,
    pub(crate) npm_metadata: Option<NpmMetadata>,
    pub(crate) nuget_metadata: Option<Vec<NugetMetadatum>>,
    pub(crate) package_files: Vec<PackageFile>,
    pub(crate) package_url: String,
    pub(crate) prerelease: Option<bool>,
    pub(crate) release: Option<Release>,
    pub(crate) rubygems_metadata: Option<Vec<RubyGemsMetadata>>,
    pub(crate) summary: String,
    pub(crate) tag_name: Option<String>,
    pub(crate) target_commitish: Option<String>,
    pub(crate) target_oid: Option<String>,
    pub(crate) updated_at: Option<String>,
    pub(crate) version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersionAuthor {
    pub(crate) avatar_url: String,
    pub(crate) events_url: String,
    pub(crate) followers_url: String,
    pub(crate) following_url: String,
    pub(crate) gists_url: String,
    pub(crate) gravatar_id: String,
    pub(crate) html_url: String,
    pub(crate) id: i64,
    pub(crate) login: String,
    pub(crate) node_id: String,
    pub(crate) organizations_url: String,
    pub(crate) received_events_url: String,
    pub(crate) repos_url: String,
    pub(crate) site_admin: bool,
    pub(crate) starred_url: String,
    pub(crate) subscriptions_url: String,
    #[serde(rename = "type")]
    pub(crate) author_type: String,
    pub(crate) url: String,
    pub(crate) user_view_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Body {
    AnythingMap(HashMap<String, Option<Value>>),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerMetadata {
    pub(crate) labels: Option<HashMap<String, Option<Value>>>,
    pub(crate) manifest: Option<HashMap<String, Option<Value>>>,
    pub(crate) tag: Option<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub(crate) digest: Option<String>,
    pub(crate) name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerMetadatum {
    pub(crate) tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpmMetadata {
    pub(crate) author: Option<Body>,
    pub(crate) bin: Option<HashMap<String, Option<Value>>>,
    pub(crate) bugs: Option<Body>,
    pub(crate) commit_oid: Option<String>,
    pub(crate) contributors: Option<Vec<String>>,
    pub(crate) cpu: Option<Vec<String>>,
    pub(crate) deleted_by_id: Option<i64>,
    pub(crate) dependencies: Option<HashMap<String, Option<Value>>>,
    pub(crate) description: Option<String>,
    pub(crate) dev_dependencies: Option<HashMap<String, Option<Value>>>,
    pub(crate) directories: Option<Body>,
    pub(crate) dist: Option<Body>,
    pub(crate) engines: Option<HashMap<String, Option<Value>>>,
    pub(crate) files: Option<Vec<String>>,
    pub(crate) git_head: Option<String>,
    pub(crate) has_shrinkwrap: Option<bool>,
    pub(crate) homepage: Option<String>,
    pub(crate) id: Option<String>,
    pub(crate) installation_command: Option<String>,
    pub(crate) keywords: Option<Vec<String>>,
    pub(crate) license: Option<String>,
    pub(crate) main: Option<String>,
    pub(crate) maintainers: Option<Vec<String>>,
    pub(crate) man: Option<HashMap<String, Option<Value>>>,
    pub(crate) name: Option<String>,
    pub(crate) node_version: Option<String>,
    pub(crate) npm_user: Option<String>,
    pub(crate) npm_version: Option<String>,
    pub(crate) optional_dependencies: Option<HashMap<String, Option<Value>>>,
    pub(crate) os: Option<Vec<String>>,
    pub(crate) peer_dependencies: Option<HashMap<String, Option<Value>>>,
    pub(crate) published_via_actions: Option<bool>,
    pub(crate) readme: Option<String>,
    pub(crate) release_id: Option<i64>,
    pub(crate) repository: Option<Body>,
    pub(crate) scripts: Option<HashMap<String, Option<Value>>>,
    pub(crate) version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NugetMetadatum {
    pub(crate) id: Option<Id>,
    pub(crate) name: Option<String>,
    pub(crate) value: Option<ValueUnion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id {
    AnythingMap(HashMap<String, Option<Value>>),
    Integer(i64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueUnion {
    Bool(bool),
    Integer(i64),
    String(String),
    ValueClass(ValueClass),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueClass {
    pub(crate) branch: Option<String>,
    pub(crate) commit: Option<String>,
    #[serde(rename = "type")]
    pub(crate) value_type: Option<String>,
    pub(crate) url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageFile {
    pub(crate) content_type: String,
    pub(crate) created_at: String,
    pub(crate) download_url: String,
    pub(crate) id: i64,
    pub(crate) md5: String,
    pub(crate) name: String,
    pub(crate) sha1: String,
    pub(crate) sha256: String,
    pub(crate) size: i64,
    pub(crate) state: String,
    pub(crate) updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub(crate) author: Option<ReleaseAuthor>,
    pub(crate) created_at: Option<String>,
    pub(crate) draft: Option<bool>,
    pub(crate) html_url: Option<String>,
    pub(crate) id: Option<i64>,
    pub(crate) name: Option<String>,
    pub(crate) prerelease: Option<bool>,
    pub(crate) published_at: Option<String>,
    pub(crate) tag_name: Option<String>,
    pub(crate) target_commitish: Option<String>,
    pub(crate) url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseAuthor {
    pub(crate) avatar_url: Option<String>,
    pub(crate) events_url: Option<String>,
    pub(crate) followers_url: Option<String>,
    pub(crate) following_url: Option<String>,
    pub(crate) gists_url: Option<String>,
    pub(crate) gravatar_id: Option<String>,
    pub(crate) html_url: Option<String>,
    pub(crate) id: Option<i64>,
    pub(crate) login: Option<String>,
    pub(crate) node_id: Option<String>,
    pub(crate) organizations_url: Option<String>,
    pub(crate) received_events_url: Option<String>,
    pub(crate) repos_url: Option<String>,
    pub(crate) site_admin: Option<bool>,
    pub(crate) starred_url: Option<String>,
    pub(crate) subscriptions_url: Option<String>,
    #[serde(rename = "type")]
    pub(crate) author_type: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) user_view_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubyGemsMetadata {
    pub(crate) commit_oid: Option<String>,
    pub(crate) dependencies: Option<Vec<HashMap<String, String>>>,
    pub(crate) description: Option<String>,
    pub(crate) homepage: Option<String>,
    pub(crate) metadata: Option<HashMap<String, String>>,
    pub(crate) name: Option<String>,
    pub(crate) platform: Option<String>,
    pub(crate) readme: Option<String>,
    pub(crate) repo: Option<String>,
    pub(crate) version_info: Option<VersionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub(crate) version: Option<String>,
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

/// The repository on GitHub where the event occurred. Webhook payloads contain the
/// `repository` property
/// when the event occurs from activity in a repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    /// Whether to allow Auto-merge to be used on pull requests.
    pub(crate) allow_auto_merge: Option<bool>,
    /// Whether to allow forking this repo
    pub(crate) allow_forking: Option<bool>,
    /// Whether to allow merge commits for pull requests.
    pub(crate) allow_merge_commit: Option<bool>,
    /// Whether to allow rebase merges for pull requests.
    pub(crate) allow_rebase_merge: Option<bool>,
    /// Whether to allow squash merges for pull requests.
    pub(crate) allow_squash_merge: Option<bool>,
    /// Whether or not a pull request head branch that is behind its base branch can always be
    /// updated even if it is not required to be up to date before merging.
    pub(crate) allow_update_branch: Option<bool>,
    /// Whether anonymous git access is enabled for this repository
    pub(crate) anonymous_access_enabled: Option<bool>,
    pub(crate) archive_url: String,
    /// Whether the repository is archived.
    pub(crate) archived: bool,
    pub(crate) assignees_url: String,
    pub(crate) blobs_url: String,
    pub(crate) branches_url: String,
    pub(crate) clone_url: String,
    pub(crate) collaborators_url: String,
    pub(crate) comments_url: String,
    pub(crate) commits_url: String,
    pub(crate) compare_url: String,
    pub(crate) contents_url: String,
    pub(crate) contributors_url: String,
    pub(crate) created_at: String,
    /// The custom properties that were defined for the repository. The keys are the custom
    /// property names, and the values are the corresponding custom property values.
    pub(crate) custom_properties: Option<HashMap<String, Option<Value>>>,
    /// The default branch of the repository.
    pub(crate) default_branch: String,
    /// Whether to delete head branches when pull requests are merged
    pub(crate) delete_branch_on_merge: Option<bool>,
    pub(crate) deployments_url: String,
    pub(crate) description: Option<String>,
    /// Returns whether or not this repository disabled.
    pub(crate) disabled: bool,
    pub(crate) downloads_url: String,
    pub(crate) events_url: String,
    pub(crate) fork: bool,
    pub(crate) forks: i64,
    pub(crate) forks_count: i64,
    pub(crate) forks_url: String,
    pub(crate) full_name: String,
    pub(crate) git_commits_url: String,
    pub(crate) git_refs_url: String,
    pub(crate) git_tags_url: String,
    pub(crate) git_url: String,
    /// Whether discussions are enabled.
    pub(crate) has_discussions: Option<bool>,
    /// Whether downloads are enabled.
    pub(crate) has_downloads: bool,
    /// Whether issues are enabled.
    pub(crate) has_issues: bool,
    pub(crate) has_pages: bool,
    /// Whether projects are enabled.
    pub(crate) has_projects: bool,
    /// Whether the wiki is enabled.
    pub(crate) has_wiki: bool,
    pub(crate) homepage: Option<String>,
    pub(crate) hooks_url: String,
    pub(crate) html_url: String,
    /// Unique identifier of the repository
    pub(crate) id: i64,
    /// Whether this repository acts as a template that can be used to generate new repositories.
    pub(crate) is_template: Option<bool>,
    pub(crate) issue_comment_url: String,
    pub(crate) issue_events_url: String,
    pub(crate) issues_url: String,
    pub(crate) keys_url: String,
    pub(crate) labels_url: String,
    pub(crate) language: String,
    pub(crate) languages_url: String,
    /// License Simple
    pub(crate) license: Option<LicenseSimple>,
    pub(crate) master_branch: Option<String>,
    /// The default value for a merge commit message.
    ///
    /// - `PR_TITLE` - default to the pull request's title.
    /// - `PR_BODY` - default to the pull request's body.
    /// - `BLANK` - default to a blank commit message.
    pub(crate) merge_commit_message: Option<MergeCommitMessage>,
    /// The default value for a merge commit title.
    ///
    /// - `PR_TITLE` - default to the pull request's title.
    /// - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull
    /// request #123 from branch-name).
    pub(crate) merge_commit_title: Option<MergeCommitTitle>,
    pub(crate) merges_url: String,
    pub(crate) milestones_url: String,
    pub(crate) mirror_url: Option<String>,
    /// The name of the repository.
    pub(crate) name: String,
    pub(crate) network_count: Option<i64>,
    pub(crate) node_id: String,
    pub(crate) notifications_url: String,
    pub(crate) open_issues: i64,
    pub(crate) open_issues_count: i64,
    /// A GitHub user.
    pub(crate) organization: Option<OrganizationClass>,
    /// A GitHub user.
    pub(crate) owner: OwnerClass,
    pub(crate) permissions: Option<RepositoryPermissions>,
    /// Whether the repository is private or public.
    pub(crate) private: bool,
    pub(crate) pulls_url: String,
    pub(crate) pushed_at: String,
    pub(crate) releases_url: String,
    /// The size of the repository, in kilobytes. Size is calculated hourly. When a repository is
    /// initially created, the size is 0.
    pub(crate) size: i64,
    /// The default value for a squash merge commit message:
    ///
    /// - `PR_BODY` - default to the pull request's body.
    /// - `COMMIT_MESSAGES` - default to the branch's commit messages.
    /// - `BLANK` - default to a blank commit message.
    pub(crate) squash_merge_commit_message: Option<SquashMergeCommitMessage>,
    /// The default value for a squash merge commit title:
    ///
    /// - `PR_TITLE` - default to the pull request's title.
    /// - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull
    /// request's title (when more than one commit).
    pub(crate) squash_merge_commit_title: Option<SquashMergeCommitTitle>,
    pub(crate) ssh_url: String,
    pub(crate) stargazers_count: i64,
    pub(crate) stargazers_url: String,
    pub(crate) starred_at: Option<String>,
    pub(crate) statuses_url: String,
    pub(crate) subscribers_count: Option<i64>,
    pub(crate) subscribers_url: String,
    pub(crate) subscription_url: String,
    pub(crate) svn_url: String,
    pub(crate) tags_url: String,
    pub(crate) teams_url: Option<String>,
    pub(crate) temp_clone_token: Option<String>,
    pub(crate) template_repository: Option<TemplateRepository>,
    pub(crate) topics: Option<Vec<String>>,
    pub(crate) trees_url: String,
    pub(crate) updated_at: String,
    pub(crate) url: String,
    /// Whether a squash merge commit can use the pull request title as default. **This property
    /// is closing down. Please use `squash_merge_commit_title` instead.
    pub(crate) use_squash_pr_title_as_default: Option<bool>,
    /// The repository visibility: public, private, or internal.
    pub(crate) visibility: Option<String>,
    pub(crate) watchers: i64,
    pub(crate) watchers_count: i64,
    /// Whether to require contributors to sign off on web-based commits
    pub(crate) web_commit_signoff_required: Option<bool>,
}

/// License Simple
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseSimple {
    pub(crate) html_url: Option<String>,
    pub(crate) key: String,
    pub(crate) name: String,
    pub(crate) node_id: String,
    pub(crate) spdx_id: String,
    pub(crate) url: String,
}

/// The default value for a merge commit message.
///
/// - `PR_TITLE` - default to the pull request's title.
/// - `PR_BODY` - default to the pull request's body.
/// - `BLANK` - default to a blank commit message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MergeCommitMessage {
    Blank,
    #[serde(rename = "PR_BODY")]
    PrBody,
    #[serde(rename = "PR_TITLE")]
    PrTitle,
}

/// The default value for a merge commit title.
///
/// - `PR_TITLE` - default to the pull request's title.
/// - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull
/// request #123 from branch-name).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MergeCommitTitle {
    #[serde(rename = "MERGE_MESSAGE")]
    MergeMessage,
    #[serde(rename = "PR_TITLE")]
    PrTitle,
}

/// A GitHub user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationClass {
    pub(crate) avatar_url: String,
    pub(crate) email: Option<String>,
    pub(crate) events_url: String,
    pub(crate) followers_url: String,
    pub(crate) following_url: String,
    pub(crate) gists_url: String,
    pub(crate) gravatar_id: String,
    pub(crate) html_url: String,
    pub(crate) id: i64,
    pub(crate) login: String,
    pub(crate) name: Option<String>,
    pub(crate) node_id: String,
    pub(crate) organizations_url: String,
    pub(crate) received_events_url: String,
    pub(crate) repos_url: String,
    pub(crate) site_admin: bool,
    pub(crate) starred_at: Option<String>,
    pub(crate) starred_url: String,
    pub(crate) subscriptions_url: String,
    #[serde(rename = "type")]
    pub(crate) simple_user_type: String,
    pub(crate) url: String,
    pub(crate) user_view_type: Option<String>,
}

/// A GitHub user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerClass {
    pub(crate) avatar_url: String,
    pub(crate) email: Option<String>,
    pub(crate) events_url: String,
    pub(crate) followers_url: String,
    pub(crate) following_url: String,
    pub(crate) gists_url: String,
    pub(crate) gravatar_id: String,
    pub(crate) html_url: String,
    pub(crate) id: i64,
    pub(crate) login: String,
    pub(crate) name: Option<String>,
    pub(crate) node_id: String,
    pub(crate) organizations_url: String,
    pub(crate) received_events_url: String,
    pub(crate) repos_url: String,
    pub(crate) site_admin: bool,
    pub(crate) starred_at: Option<String>,
    pub(crate) starred_url: String,
    pub(crate) subscriptions_url: String,
    #[serde(rename = "type")]
    pub(crate) simple_user_type: String,
    pub(crate) url: String,
    pub(crate) user_view_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryPermissions {
    pub(crate) admin: bool,
    pub(crate) maintain: Option<bool>,
    pub(crate) pull: bool,
    pub(crate) push: bool,
    pub(crate) triage: Option<bool>,
}

/// The default value for a squash merge commit message:
///
/// - `PR_BODY` - default to the pull request's body.
/// - `COMMIT_MESSAGES` - default to the branch's commit messages.
/// - `BLANK` - default to a blank commit message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SquashMergeCommitMessage {
    Blank,
    #[serde(rename = "COMMIT_MESSAGES")]
    CommitMessages,
    #[serde(rename = "PR_BODY")]
    PrBody,
}

/// The default value for a squash merge commit title:
///
/// - `PR_TITLE` - default to the pull request's title.
/// - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull
/// request's title (when more than one commit).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SquashMergeCommitTitle {
    #[serde(rename = "COMMIT_OR_PR_TITLE")]
    CommitOrPrTitle,
    #[serde(rename = "PR_TITLE")]
    PrTitle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateRepository {
    pub(crate) allow_auto_merge: Option<bool>,
    pub(crate) allow_merge_commit: Option<bool>,
    pub(crate) allow_rebase_merge: Option<bool>,
    pub(crate) allow_squash_merge: Option<bool>,
    pub(crate) allow_update_branch: Option<bool>,
    pub(crate) archive_url: Option<String>,
    pub(crate) archived: Option<bool>,
    pub(crate) assignees_url: Option<String>,
    pub(crate) blobs_url: Option<String>,
    pub(crate) branches_url: Option<String>,
    pub(crate) clone_url: Option<String>,
    pub(crate) collaborators_url: Option<String>,
    pub(crate) comments_url: Option<String>,
    pub(crate) commits_url: Option<String>,
    pub(crate) compare_url: Option<String>,
    pub(crate) contents_url: Option<String>,
    pub(crate) contributors_url: Option<String>,
    pub(crate) created_at: Option<String>,
    pub(crate) default_branch: Option<String>,
    pub(crate) delete_branch_on_merge: Option<bool>,
    pub(crate) deployments_url: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) disabled: Option<bool>,
    pub(crate) downloads_url: Option<String>,
    pub(crate) events_url: Option<String>,
    pub(crate) fork: Option<bool>,
    pub(crate) forks_count: Option<i64>,
    pub(crate) forks_url: Option<String>,
    pub(crate) full_name: Option<String>,
    pub(crate) git_commits_url: Option<String>,
    pub(crate) git_refs_url: Option<String>,
    pub(crate) git_tags_url: Option<String>,
    pub(crate) git_url: Option<String>,
    pub(crate) has_downloads: Option<bool>,
    pub(crate) has_issues: Option<bool>,
    pub(crate) has_pages: Option<bool>,
    pub(crate) has_projects: Option<bool>,
    pub(crate) has_wiki: Option<bool>,
    pub(crate) homepage: Option<String>,
    pub(crate) hooks_url: Option<String>,
    pub(crate) html_url: Option<String>,
    pub(crate) id: Option<i64>,
    pub(crate) is_template: Option<bool>,
    pub(crate) issue_comment_url: Option<String>,
    pub(crate) issue_events_url: Option<String>,
    pub(crate) issues_url: Option<String>,
    pub(crate) keys_url: Option<String>,
    pub(crate) labels_url: Option<String>,
    pub(crate) language: Option<String>,
    pub(crate) languages_url: Option<String>,
    /// The default value for a merge commit message.
    ///
    /// - `PR_TITLE` - default to the pull request's title.
    /// - `PR_BODY` - default to the pull request's body.
    /// - `BLANK` - default to a blank commit message.
    pub(crate) merge_commit_message: Option<MergeCommitMessage>,
    /// The default value for a merge commit title.
    ///
    /// - `PR_TITLE` - default to the pull request's title.
    /// - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull
    /// request #123 from branch-name).
    pub(crate) merge_commit_title: Option<MergeCommitTitle>,
    pub(crate) merges_url: Option<String>,
    pub(crate) milestones_url: Option<String>,
    pub(crate) mirror_url: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) network_count: Option<i64>,
    pub(crate) node_id: Option<String>,
    pub(crate) notifications_url: Option<String>,
    pub(crate) open_issues_count: Option<i64>,
    pub(crate) owner: Option<TemplateRepositoryOwner>,
    pub(crate) permissions: Option<TemplateRepositoryPermissions>,
    pub(crate) private: Option<bool>,
    pub(crate) pulls_url: Option<String>,
    pub(crate) pushed_at: Option<String>,
    pub(crate) releases_url: Option<String>,
    pub(crate) size: Option<i64>,
    /// The default value for a squash merge commit message:
    ///
    /// - `PR_BODY` - default to the pull request's body.
    /// - `COMMIT_MESSAGES` - default to the branch's commit messages.
    /// - `BLANK` - default to a blank commit message.
    pub(crate) squash_merge_commit_message: Option<SquashMergeCommitMessage>,
    /// The default value for a squash merge commit title:
    ///
    /// - `PR_TITLE` - default to the pull request's title.
    /// - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull
    /// request's title (when more than one commit).
    pub(crate) squash_merge_commit_title: Option<SquashMergeCommitTitle>,
    pub(crate) ssh_url: Option<String>,
    pub(crate) stargazers_count: Option<i64>,
    pub(crate) stargazers_url: Option<String>,
    pub(crate) statuses_url: Option<String>,
    pub(crate) subscribers_count: Option<i64>,
    pub(crate) subscribers_url: Option<String>,
    pub(crate) subscription_url: Option<String>,
    pub(crate) svn_url: Option<String>,
    pub(crate) tags_url: Option<String>,
    pub(crate) teams_url: Option<String>,
    pub(crate) temp_clone_token: Option<String>,
    pub(crate) topics: Option<Vec<String>>,
    pub(crate) trees_url: Option<String>,
    pub(crate) updated_at: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) use_squash_pr_title_as_default: Option<bool>,
    pub(crate) visibility: Option<String>,
    pub(crate) watchers_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateRepositoryOwner {
    pub(crate) avatar_url: Option<String>,
    pub(crate) events_url: Option<String>,
    pub(crate) followers_url: Option<String>,
    pub(crate) following_url: Option<String>,
    pub(crate) gists_url: Option<String>,
    pub(crate) gravatar_id: Option<String>,
    pub(crate) html_url: Option<String>,
    pub(crate) id: Option<i64>,
    pub(crate) login: Option<String>,
    pub(crate) node_id: Option<String>,
    pub(crate) organizations_url: Option<String>,
    pub(crate) received_events_url: Option<String>,
    pub(crate) repos_url: Option<String>,
    pub(crate) site_admin: Option<bool>,
    pub(crate) starred_url: Option<String>,
    pub(crate) subscriptions_url: Option<String>,
    #[serde(rename = "type")]
    pub(crate) owner_type: Option<String>,
    pub(crate) url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateRepositoryPermissions {
    pub(crate) admin: Option<bool>,
    pub(crate) maintain: Option<bool>,
    pub(crate) pull: Option<bool>,
    pub(crate) push: Option<bool>,
    pub(crate) triage: Option<bool>,
}

/// A GitHub user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenderClass {
    pub(crate) avatar_url: String,
    pub(crate) email: Option<String>,
    pub(crate) events_url: String,
    pub(crate) followers_url: String,
    pub(crate) following_url: String,
    pub(crate) gists_url: String,
    pub(crate) gravatar_id: String,
    pub(crate) html_url: String,
    pub(crate) id: i64,
    pub(crate) login: String,
    pub(crate) name: Option<String>,
    pub(crate) node_id: String,
    pub(crate) organizations_url: String,
    pub(crate) received_events_url: String,
    pub(crate) repos_url: String,
    pub(crate) site_admin: bool,
    pub(crate) starred_at: Option<String>,
    pub(crate) starred_url: String,
    pub(crate) subscriptions_url: String,
    #[serde(rename = "type")]
    pub(crate) simple_user_type: String,
    pub(crate) url: String,
    pub(crate) user_view_type: Option<String>,
}