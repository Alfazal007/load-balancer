pub async fn check_health(url: &str) -> bool {
    let response = reqwest::get(format!("http://{}/health", url)).await;
    if response.is_err() {
        return false;
    }
    if response.unwrap().status() == 200 {
        return true;
    }
    false
}
