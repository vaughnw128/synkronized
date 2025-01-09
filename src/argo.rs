use derivative::Derivative;
use kube::{Api, Client, CustomResource};
use kube::api::{Patch, PatchParams};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{argo, SynkronizedProject, Template, CHART_REPO};

const ARGO_NAMESPACE: &str = "argocd";
const ARGO_PROJECT: &str = "default";
const LOCAL_CLUSTER: &str = "https://kubernetes.default.svc";

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct Helm {
    values: String
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(rename="repoURL")]
    repo_url: String,
    target_revision: String,
    chart: String,
    helm: Helm
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
pub struct Destination {
    server: String,
    namespace: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct SyncPolicy {
    automated: Automated,
    sync_options: Vec<String>
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Derivative)]
#[serde(rename_all = "camelCase")]
#[derivative(Default)]
pub struct Automated {
    #[derivative(Default(value="false"))]
    prune: bool,
    #[derivative(Default(value="false"))]
    self_heal: bool,
    #[derivative(Default(value="false"))]
    allow_empty: bool
}

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[kube(group = "argoproj.io", version = "v1alpha1", kind = "Application", namespaced)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    project: String,
    source: Source,
    destination: Destination,
    sync_policy: SyncPolicy
}

impl Application {
    pub fn create(project: SynkronizedProject, template: Template) -> Application {
        Application::new(&project.name, Spec {
            project: ARGO_PROJECT.to_string(),
            source: Source {
                repo_url: CHART_REPO.to_string(),
                chart: template.name,
                target_revision: template.version,
                helm: Helm {
                    values: serde_yaml::to_string(&project.config).unwrap()
                }
            },
            destination: Destination {
                server: LOCAL_CLUSTER.to_string(),
                namespace: project.name.clone(),
            },
            sync_policy: SyncPolicy {
                sync_options: vec!["CreateNamespace=true".to_string()],
                ..Default::default()
            },
            ..Default::default()
        })
    }

    pub async fn apply(self, client: Client) -> anyhow::Result<()> {
        let ss_apply = PatchParams::apply("kubectl-light").force();
        let data: serde_json::Value = serde_json::to_value(&self)?;
        let api: Api<Application> = Api::namespaced(client.clone(), ARGO_NAMESPACE);
        api.patch(&self.metadata.name.unwrap(), &ss_apply, &Patch::Apply(data)).await?;

        Ok(())
    }

}