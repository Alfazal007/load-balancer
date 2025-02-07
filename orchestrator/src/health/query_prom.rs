use prometheus_http_query::Client;

pub async fn query_prom() {
    let client = Client::default();
    let q = "topk by (code) (5, prometheus_http_requests_total)";
    let response = client.query(q).get().await;
    println!("{:?}", response);
}
