use std::collections::HashMap;

use eyre::{bail, eyre, Context, Result};
use serde_json::Value;

pub async fn fetch_prob_info(info_url: &str) -> Result<HashMap<String, Value>> {
    let response = reqwest::get(info_url)
        .await
        .wrap_err_with(|| format!("failed to GET `{info_url}`"))?;

    let status = response.status();
    let body: Value = response
        .json()
        .await
        .wrap_err_with(|| format!("failed to parse JSON response from `{info_url}`"))?;

    if !status.is_success() {
        bail!("request to `{info_url}` failed with status {status}: {body}");
    }

    extract_message_map(body)
}

pub fn read_info_url_from_arguments() -> Result<String> {
    let mut args = std::env::args().skip(1);
    let info_url = args.next().ok_or_else(|| {
        eyre!("missing info URL.\nusage: cargo run -p solver -- <http://host:port/<token>/info>")
    })?;

    if args.next().is_some() {
        return Err(eyre!(
            "expected exactly one argument.\nusage: cargo run -p solver -- <http://host:port/<token>/info>"
        ));
    }

    Ok(info_url)
}

pub fn rpc_url_from_info_url(info_url: &str) -> Result<String> {
    let trimmed = info_url.trim_end_matches('/');
    let prefix = trimmed
        .strip_suffix("/info")
        .ok_or_else(|| eyre!("info URL must end with `/info`: `{info_url}`"))?;
    Ok(format!("{prefix}/rpc"))
}

pub fn get_info_value(map: &HashMap<String, Value>, key: &str) -> Result<String> {
    map.get(key)
        .and_then(|value| value.as_str())
        .map(|x| x.to_owned())
        .ok_or_else(|| eyre!("Failed to find key or convert JSON value into string"))
}

fn extract_message_map(body: Value) -> Result<HashMap<String, Value>> {
    let root = body
        .as_object()
        .ok_or_else(|| eyre!("response body must be a JSON object"))?;
    let message = root
        .get("message")
        .ok_or_else(|| eyre!("response object must include `message`"))?;
    let message_obj = message
        .as_object()
        .ok_or_else(|| eyre!("`message` must be a JSON object"))?;
    Ok(message_obj.clone().into_iter().collect())
}