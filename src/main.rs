mod argo;

use k8s_openapi::api::core::v1::Pod;
use dotenv::dotenv;
use kube::config::KubeConfigOptions;
use anyhow::{bail, Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use derivative::Derivative;
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::{apimachinery::pkg::apis::meta::v1::Time, chrono::Utc};
use apiexts::CustomResourceDefinition;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiexts;
use kube::{
    api::{Api, DynamicObject, ListParams, Patch, PatchParams, ResourceExt},
    core::GroupVersionKind,
    discovery::{ApiCapabilities, ApiResource, Discovery, Scope},
    runtime::{
        wait::{await_condition, conditions, conditions::is_deleted},
        watcher, WatchStreamExt,
    },
    CustomResource,
    CustomResourceExt,
    Client,
    Config
};

const ARGO_NAMESPACE: &str = "argocd";
const ARGO_PROJECT: &str = "default";
const LOCAL_CLUSTER: &str = "https://kubernetes.default.svc";
const CHART_REPO: &str = "https://charts.vaughn.sh";

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(rename="repoURL")]
    repo_url: String,
    path: String,
    target_revision: String
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
pub struct Destination {
    server: String,
    namespace: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
pub struct SyncPolicy {
    automated: Automated
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
pub struct ArgoSpec {
    project: String,
    source: Source,
    destination: Destination,
    sync_policy: SyncPolicy
}

pub struct Project {
    name: String,
    repo: String
}

async fn get_chart(chart_name: String) -> Result<()> {
    let body = reqwest::get(CHART_REPO)
    .await?
    .text()
    .await?;

    println!("{:?}", body);

    Ok(())
}


async fn apply(client: Client, yaml: serde_yaml::Value) -> Result<()> {
    let ssapply = PatchParams::apply("kubectl-light").force();
    let obj: DynamicObject = serde_yaml::from_value(yaml)?;
    let gvk = if let Some(tm) = &obj.types {
        GroupVersionKind::try_from(tm)?
    } else {
        bail!("cannot apply object without valid TypeMeta {:?}", obj);
    };
    let name = obj.name_any();
    let api: Api<Application> = Api::namespaced(client.clone(), ARGO_NAMESPACE);
    println!("Applying {}: \n{}", gvk.kind, serde_yaml::to_string(&obj)?);
    let data: serde_json::Value = serde_json::to_value(&obj)?;
    let _r = api.patch(&name, &ssapply, &Patch::Apply(data)).await?;
    println!("applied {} {}", gvk.kind, name);

    Ok(())
}

fn create_application(project: Project) -> Application {

    let argo_spec = ArgoSpec {
        project: ARGO_PROJECT.to_string(),
        source: Source {
                repo_url: project.repo,
                path: project.name.clone(),
                target_revision: "HEAD".to_string(),
            },
        destination: Destination {
            server: LOCAL_CLUSTER.to_string(),
            namespace: project.name.clone(),
        },
        ..Default::default()
    };

    Application::new(&project.name, argo_spec)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();

    let mut kube_config = Config::from_kubeconfig(&KubeConfigOptions::default()).await?;
    kube_config.accept_invalid_certs = true;
    let client = Client::try_from(kube_config).expect("Could not configure the client.");

    let mcstatus = Project {
        name: "mc-status-rs".to_string(),
        repo: "git@github.com:vaughnw128/k8s-infra.git".to_string(),
    };

    let application = create_application(mcstatus);
    apply(client, serde_yaml::to_value(&application)?).await?;

    Ok(())
}
