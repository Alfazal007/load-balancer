use crate::database::get_data::Server;

pub fn add_server(prev_state: &Vec<Server>, new_state: &Vec<Server>) -> Vec<Server> {
    let mut all_vector = Vec::new();
    for server in new_state.iter() {
        if !prev_state.contains(server) {
            all_vector.push(Server {
                id: server.id,
                server_url: server.server_url.to_owned(),
            });
        }
    }
    all_vector
}
