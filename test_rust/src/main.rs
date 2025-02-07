use prometheus_http_query::Client;

pub mod docker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();
    // TODO:: hit the database to get the current total servers (done already in the previous main
    // repo)
    // do this or something similar to handle this issue
    // if the current total is 3 then set a flag to not trigger add_server
    // else add the server and set the flag and update the database and run the docker container
    // remove the flag

    let q = "sum(rate(http_requests_total[5m])) * 60";
    let response_res = client.query(q).get().await;
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
        println!("The final rate is {:?}", rate);

        if rate > 13.0 {
            println!("Add a server");
            docker::add_server::add_server().await;
        } else {
            println!("Remove a server if server count > 2");
            docker::remove_server::remove_server().await;
        }
    }
    Ok(())
}
