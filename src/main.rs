pub mod argo;
pub mod github;

use std::collections::HashMap;
use std::fs::rename;
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

const CHART_REPO: &str = "https://charts.vaughn.sh";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SynkronizedProject {
    name: String,
    template: String,
    config: serde_yaml::Value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Charts {
    apiVersion: String,
    entries: HashMap<String, Vec<ChartVersion>>,
    generated: String
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

// #[tokio::main]
// async fn main() -> Result<()>{
//     dotenv().ok();
//
//     let mut kube_config = Config::from_kubeconfig(&KubeConfigOptions::default()).await?;
//     kube_config.accept_invalid_certs = true;
//     let client = Client::try_from(kube_config).expect("Could not configure the client.");
//
//     let synkronized_yaml = read_synkronized_file("synkronized.yaml").await?;
//     let chart_template = get_chart_template(&synkronized_yaml.template).await?;
//
//     // println!("{:?}", chart);
//     // println!("{:?}", synkronized_yaml);
//
//     let application = argo::Application::create(synkronized_yaml, chart_template);
//     application.apply(client).await
// }

fn registry_published (package_published: github::RegistryPublished) {
    package_published.registry_package.package_version.package_url;
}

async fn github_hooks(headers: HeaderMap, payload: Result<Json<github::WebhookPayload>, JsonRejection>) -> Result<StatusCode, (StatusCode, String)> {
    // println!("{:?}", serde_json::to_string(&package.0).unwrap());
    // let event_type = headers.get("X-GitHub-Event").unwrap();

    if headers.get("X-GitHub-Event").unwrap().to_str().unwrap() == "ping" {
        return Ok(StatusCode::OK)
    };

    let payload = match payload {
        Ok(payload) => payload,
        Err(JsonRejection::JsonDataError(_)) => return Err((StatusCode::NOT_ACCEPTABLE, "The supplied webhook payload type is not accepted.".to_string())),
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "An unknown error has occurred in JSON parsing.".to_string()))
    };

    match payload {
        Json(github::WebhookPayload::Published(payload)) => registry_published(payload)
    };

    Ok(StatusCode::OK)

    // match payload.unwrap() {
    //     "registry_package" => Ok(StatusCode::OK),
    //     "ping" => Ok(StatusCode::OK),
    //     _ => Err((StatusCode::NOT_ACCEPTABLE, format!("Payload type `{}` is not accepted.", payload_type)))
    // }

    // if event_type != "registry_package" {
    //     return StatusCode::OK
    // }
    // println!("{:?}", event_type);
    // // let publish_package_event: github::PackageUpdate = serde_json::from_value(package).unwrap();
    // println!("{:?}", package.registry_package);
    //
    // Ok(StatusCode::OK)
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/github-hooks", post(github_hooks));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
