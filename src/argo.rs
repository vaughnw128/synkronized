use derivative::Derivative;
use kube::{Api, Client, CustomResource};
use kube::api::{Patch, PatchParams};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{SynkronizedProject, helm};

const ARGO_NAMESPACE: &str = "argocd";

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

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default, Derivative)]
pub struct Destination {
    // #[derivative(Default(value="in-cluster"))]
    // name: String,
    #[derivative(Default(value="https://kubernetes.default.svc"))]
    server: String,
    #[derivative(Default(value="argocd"))]
    namespace: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct SyncPolicy {
    automated: Automated,
    sync_options: Vec<String>
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct Automated {
    #[derivative(Default(value="false"))]
    prune: bool,
    #[derivative(Default(value="false"))]
    self_heal: bool,
    #[derivative(Default(value="false"))]
    allow_empty: bool
}

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema, Default, Derivative)]
#[kube(group = "argoproj.io", version = "v1alpha1", kind = "Application", namespaced)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    #[derivative(Default(value="default"))]
    project: String,
    source: Source,
    destination: Destination,
    sync_policy: SyncPolicy
}

impl Application {
    pub fn create(project: SynkronizedProject, template: helm::Template) -> Application {
        Application::new(&project.synkronized.name, Spec {
            source: Source {
                repo_url: helm::CHART_REPO.to_string(),
                chart: template.name,
                target_revision: template.version,
                helm: Helm {
                    values: serde_yaml::to_string(&project.config).unwrap()
                }
            },
            sync_policy: SyncPolicy {
                sync_options: vec!["CreateNamespace=true".to_string()],
                ..Default::default()
            },
            ..Default::default()
        })
    }

    pub async fn apply(self, client: &Client) -> anyhow::Result<()> {
        let ss_apply = PatchParams::apply("kubectl-light").force();
        let data: serde_json::Value = serde_json::to_value(&self)?;
        let api: Api<Application> = Api::namespaced(client.clone(), ARGO_NAMESPACE);
        api.patch(&self.metadata.name.unwrap(), &ss_apply, &Patch::Apply(data)).await?;

        Ok(())
    }

}