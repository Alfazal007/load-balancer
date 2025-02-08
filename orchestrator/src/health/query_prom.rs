use prometheus_http_query::Client;

pub async fn query_prom_need_more() -> bool {
    let prometheus_url = "http://prometheus:9090";
    let client = Client::try_from(prometheus_url).expect("Failed to create Prometheus client");

    let q = "sum(rate(http_requests_total[5m])) * 60";
    let response_res = client.query(q).get().await;
    println!("{:?}", response_res);
    if response_res.is_ok() {
        let response = response_res.unwrap();
        let response_inner_data = response.data();
        let mut rate = 0.0;
        if let Some(vector) = response_inner_data.as_vector() {
            for sample in vector {
                rate = sample.sample().value();
                println!(
                    "Rate of requests(per minute) over period of 5 minutes is Value: {}",
                    rate
                );
            }
        } else {
            println!("No vector data found in the response.");
        }

        if rate > 13.0 {
            println!("Add server");
            return true;
        } else {
            println!("No need to add server");
        }
    }
    false
}
