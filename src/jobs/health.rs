use chrono::NaiveDateTime;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Debug)]
pub struct GetModuleResponse {
    id: i32,
    name: String,
    full_name: String,
    description: String,
    api_url: String,
    web_url: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Serialize)]
struct NewModuleRequest {
    name: String,
    full_name: String,
    description: String,
    api_url: String,
    web_url: String,
}

#[derive(Serialize)]
struct UpdateModuleRequest {
    id: i32,
    name: String,
    full_name: String,
    description: String,
    api_url: String,
    web_url: String,
}

const MODULE_NAME: &str = "projects";
const MODULE_FULL_NAME: &str = "Projects";
const MODULE_DESCRIPTION: &str = "Projects Module";

pub async fn create_module(core_api_url: String, client: reqwest::Client) -> std::io::Result<()> {
    let new_module = NewModuleRequest {
        name: MODULE_NAME.to_string(),
        full_name: MODULE_FULL_NAME.to_string(),
        description: MODULE_DESCRIPTION.to_string(),
        api_url: match std::env::var("MODULE_API_URL") {
            Ok(value) => value,
            Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::Other, error)),
        },
        web_url: match std::env::var("MODULE_WEB_URL") {
            Ok(value) => value,
            Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::Other, error)),
        },
    };

    let create_module_url = format!("{}/modules", core_api_url);

    let response = match client
        .post(&create_module_url)
        .json(&new_module)
        .send()
        .await
    {
        Ok(value) => value,
        Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::Other, error)),
    };

    match response.status() {
        reqwest::StatusCode::CREATED => {}
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error creating module",
            ));
        }
    }

    Ok(())
}

pub async fn update_module(
    core_api_url: String,
    client: reqwest::Client,
    module: GetModuleResponse,
) -> std::io::Result<()> {
    let update_module = UpdateModuleRequest {
        id: module.id,
        name: MODULE_NAME.to_string(),
        full_name: MODULE_FULL_NAME.to_string(),
        description: MODULE_DESCRIPTION.to_string(),
        api_url: match std::env::var("MODULE_API_URL") {
            Ok(value) => value,
            Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::Other, error)),
        },
        web_url: match std::env::var("MODULE_WEB_URL") {
            Ok(value) => value,
            Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::Other, error)),
        },
    };

    let update_module_url = format!("{}/modules/{}", core_api_url, update_module.id);

    let response = match client
        .put(&update_module_url)
        .json(&update_module)
        .send()
        .await
    {
        Ok(value) => value,
        Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::Other, error)),
    };

    match response.status() {
        reqwest::StatusCode::OK => {}
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error updating module",
            ));
        }
    }

    Ok(())
}

pub async fn job() -> std::io::Result<()> {
    let core_api_url = match std::env::var("CORE_API_URL") {
        Ok(value) => value,
        Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::Other, error)),
    };

    let client = reqwest::Client::new();

    // Set headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let get_module_url = format!("{}/modules/name/{}", core_api_url, MODULE_NAME);

    let response = match client
        .get(&get_module_url)
        .headers(headers.clone()) // Ajout des headers
        .send()
        .await {
        Ok(value) => value,
        Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::Other, error)),
    };

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body = response.text().await.unwrap();
            let module: GetModuleResponse = serde_json::from_str(&response_body)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            match update_module(core_api_url, client, module).await {
                Ok(_) => {}
                Err(error) => return Err(error),
            }
        }
        _ => match create_module(core_api_url, client).await {
            Ok(_) => {}
            Err(error) => return Err(error),
        },
    }

    Ok(())
}
