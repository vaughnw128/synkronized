// Define modules
mod argo;
mod github;
mod utils;
mod helm;

// Define imports
use std::fmt::Display;
use std::sync::Arc;
use dotenv::dotenv;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use axum::{routing::{post}, http::StatusCode, Json, Router, http::header::HeaderMap};
use axum::extract::State;
use base64::prelude::*;
use kube::{Client, Config};
use kube::config::{KubeConfigOptions, Kubeconfig};
use octocrab::Octocrab;
use serde_json::{json, Value};



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
pub struct ContainerImage {
    name: String,
    image: String,
}

#[derive(Clone)]
struct AppState {
    github_client: Octocrab,
    kube_client: Client,
}

async fn registry_published (package_published: github::RegistryPublished, github_client: &Octocrab, kube_client: &Client)  -> Result<StatusCode> {
    // Pull the Synkronized.yaml file from the repository base
    let encoded_yaml = match package_published.repository {
        Some(repo) => github_client.repos(repo.owner.login, repo.name)
            .get_content()
            .path("synkronized.yaml")
            .send()
            .await?
            .items[0]
            .clone()
            .content
            .unwrap()
            .replace("\n", ""),
        None => return Err(anyhow!("No repository was specified by the webhook."))
    };

    // Decode the synkronized yaml from base64 as this is what Github returns
    let mut synkronized_yaml: SynkronizedProject = serde_yaml::from_str(&String::from_utf8(BASE64_STANDARD.decode(encoded_yaml)?)?)?;

    if package_published.registry_package.package_version.package_url.split(":").collect::<Vec<&str>>()[1] == "" {
        return Err(anyhow!("Unknown server error."));
    }

    let container_image = serde_yaml::to_value(ContainerImage {
        name: package_published.registry_package.name,
        image: package_published.registry_package.package_version.package_url,
    })?;

    // Merge the pulled spec yaml, and the name + image
    utils::merge_yaml(&mut synkronized_yaml.config, container_image);

    let chart_template = helm::Template::from_chart_repo(&synkronized_yaml.synkronized.template).await?;

    let application = argo::Application::create(synkronized_yaml, chart_template);
    application.apply(kube_client).await?;

    Ok(StatusCode::OK)
}

fn json_error(message: impl Display) -> (StatusCode, Json<Value>) {
    (StatusCode::BAD_REQUEST, Json(json!(
        {"message": message.to_string()}
    )))
}

async fn github_hooks(headers: HeaderMap, State(state): State<Arc<AppState>>, payload: github::WebhookPayload) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    // Handle ping type events before routing specific payload types
    if headers.get("X-GitHub-Event")
        .ok_or(json_error("Expected X-Github-Event"))?
        .to_str().unwrap() == "ping" {
        return Ok(StatusCode::OK)
    };

    // let payload = match payload {
    //     Ok(payload) => payload,
    //     Err(JsonRejection::JsonDataError(_)) => return Err(json_error("The supplied webhook payload type is not accepted.")),
    //     Err(_) => return Err(json_error("An unknown error has occurred in JSON parsing."))
    // };

    // Process different payload types based on enum parsed
    match payload {
        github::WebhookPayload::Published(payload) => registry_published(payload, &state.github_client, &state.kube_client).await.map_err(|_| json_error("Something went wrong!"))?
    };

    Ok(StatusCode::OK)
}


#[tokio::main]
async fn main() -> Result<()>{
    dotenv().ok();

    tracing_subscriber::fmt::init();

    // Initialize Octocrab client
    let token = std::env::var("GITHUB_API_TOKEN").expect("GITHUB_API_TOKEN env variable is required");
    let github_client = Octocrab::builder().personal_token(token).build()?;

    // Initialize kube client
    let kubeconfig_secret = std::env::var("KUBE_CONFIG").expect("KUBE_CONFIG env variable is required");
    let custom_kubeconfig = Kubeconfig::from_yaml(&String::from_utf8(BASE64_STANDARD.decode(&kubeconfig_secret)?)?)?;

    // let custom_kubeconfig = Kubeconfig::from_yaml(&kubeconfig_secret)?;
    let mut kube_config = Config::from_custom_kubeconfig(custom_kubeconfig, &KubeConfigOptions::default()).await?;
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
