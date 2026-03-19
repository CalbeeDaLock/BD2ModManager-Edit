use semver::Version;
use serde::Deserialize;

// [TODO] remove this and use the tauri auto updater plugin

const RELEASES_URL: &str = "https://shy-waterfall-2797.bruhnn.workers.dev/";

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

pub fn get_app_version(app: &tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

pub async fn get_app_latest_version(app: &tauri::AppHandle) -> Result<(Version, String), String> {
    let current_version = get_app_version(&app);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {e}"))?;

    let release: GitHubRelease = client
        .get(RELEASES_URL)
        // .header("User-Agent", "BD2ModManager")
        .header("Accept", "application/json")
        .header("X-Manager-Version", current_version)
        .header("X-Manager-Platform", std::env::consts::OS)
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?
        .json()
        .await
        .map_err(|e| format!("Invalid JSON: {e}"))?;

    let latest_version = release.tag_name.trim_start_matches('v');

    let html_url = release.html_url.clone();
    println!("Latest version: {latest_version}, URL: {html_url}");

    let version =
        Version::parse(latest_version).map_err(|e| format!("Invalid remote version: {e}"))?;

    Ok((version, html_url))
}
