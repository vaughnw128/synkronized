pub mod argo;
pub mod github;

use std::collections::HashMap;
use std::fs::rename;
use std::sync::{Arc, Mutex};
use k8s_openapi::api::core::v1::Pod;
use dotenv::dotenv;
use anyhow::{bail, Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use derivative::Derivative;
use futures::{StreamExt, TryStreamExt};
use chrono::DateTime;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    http::header::HeaderMap
};
use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use base64::prelude::*;
use kube::{Client, Config};
use kube::config::KubeConfigOptions;
use octocrab::Octocrab;

const CHART_REPO: &str = "https://charts.vaughn.sh";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Synkronized {
    name: String,
    template: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SynkronizedProject {
    synkronized: Synkronized,
    config: serde_yaml::Value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Charts {
    apiVersion: String,
    entries: HashMap<String, Vec<ChartVersion>>,
    generated: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerImage {
    name: String,
    image: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChartVersion {
    apiVersion: String,
    appVersion: String,
    created: String,
    description: String,
    digest: String,
    name: String,
    #[serde(rename="type")]
    chart_type: String,
    urls: Vec<String>,
    version: String
}

struct AppState {
    github_client: Octocrab,
    kube_client: Client
}

async fn get_chart_template(chart_name: &str) -> Result<Template> {
    let chart_repo_yaml = format!("{CHART_REPO}/index.yaml");

    let body = reqwest::get(chart_repo_yaml)
    .await?
    .text()
    .await?;

    let charts: Charts = serde_yaml::from_str(&body).expect("Unable to parse charts.");
    match charts.entries.get(chart_name) {
        Some(charts) => Ok(Template{
            name: charts[0].name.clone(),
            version: charts[0].version.clone()
        }),
        None => panic!("No viable charts were found for {}", chart_name)
    }
}

async fn read_synkronized_file(fp: &str) -> Result<SynkronizedProject> {
    let f = std::fs::File::open(fp)?;
    let synkronized_yaml: SynkronizedProject = serde_yaml::from_reader(f)?;
    Ok(synkronized_yaml)
}

fn merge_yaml(a: &mut serde_yaml::Value, b: serde_yaml::Value) {
    match (a, b) {
        (a @ &mut serde_yaml::Value::Mapping(_), serde_yaml::Value::Mapping(b)) => {
            let a = a.as_mapping_mut().unwrap();
            for (k, v) in b {
                if !a.contains_key(&k) {a.insert(k.to_owned(), v.to_owned());}
                else { merge_yaml(&mut a[&k], v); }
            }
        }
        (a, b) => *a = b,
    }
}


async fn registry_published (package_published: github::RegistryPublished, github_client: &Octocrab, kube_client: &Client)  -> Result<StatusCode, (StatusCode, String)> {
    let encoded_yaml = github_client.repos(package_published.repository.owner.login, package_published.repository.name)
        .get_content()
        .path("synkronized.yaml")
        .send()
        .await
        .unwrap()
        .items[0]
        .clone()
        .content
        .unwrap()
        .replace("\n", "");

    let mut synkronized_yaml: SynkronizedProject = serde_yaml::from_str(&String::from_utf8(BASE64_STANDARD.decode(encoded_yaml).unwrap()).unwrap()).unwrap();
    let container_image = serde_yaml::to_value(ContainerImage {
        name: package_published.registry_package.name,
        image: package_published.registry_package.package_version.package_url,
    }).unwrap();

    merge_yaml(&mut synkronized_yaml.config, container_image);

    let chart_template = get_chart_template(&synkronized_yaml.synkronized.template).await.unwrap();

    let application = argo::Application::create(synkronized_yaml, chart_template);
    application.apply(kube_client).await.unwrap();

    Ok(StatusCode::OK)
}

async fn github_hooks(headers: HeaderMap, State(state): State<Arc<AppState>>, payload: Result<Json<github::WebhookPayload>, JsonRejection>) -> Result<StatusCode, (StatusCode, String)> {

    // Handle ping type events before routing specific payload types
    if headers.get("X-GitHub-Event").unwrap().to_str().unwrap() == "ping" {
        return Ok(StatusCode::OK)
    };

    let payload = match payload {
        Ok(payload) => payload,
        Err(JsonRejection::JsonDataError(_)) => return Err((StatusCode::NOT_ACCEPTABLE, "The supplied webhook payload type is not accepted.".to_string())),
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "An unknown error has occurred in JSON parsing.".to_string()))
    };

    // Process different payload types based on enum parsed
    match payload {
        Json(github::WebhookPayload::Published(payload)) => registry_published(payload, &state.github_client, &state.kube_client).await
    }
}


#[tokio::main]
async fn main() -> Result<()>{
    dotenv().ok();

    tracing_subscriber::fmt::init();

    // Initialize Octocrab client
    let token = std::env::var("GITHUB_API_TOKEN").expect("GITHUB_API_TOKEN env variable is required");
    let github_client = Octocrab::builder().personal_token(token).build()?;

    // Initialize kube client
    let mut kube_config = Config::from_kubeconfig(&KubeConfigOptions::default()).await?;
    kube_config.accept_invalid_certs = true;
    let kube_client = Client::try_from(kube_config).expect("Could not configure the client.");

    let app_state = Arc::new(AppState { github_client, kube_client });

    let app = Router::new()
        .route("/github-hooks", post(github_hooks))
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
