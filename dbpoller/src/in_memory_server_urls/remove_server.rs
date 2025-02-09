use crate::database::get_data::Server;

pub fn remove_server(prev_state: &Vec<Server>, new_state: &Vec<Server>) -> Vec<Server> {
    let mut server_to_return = Vec::new();
    for server in prev_state.iter() {
        if !new_state.contains(server) {
            server_to_return.push(Server {
                id: server.id,
                server_url: server.server_url.to_owned(),
            });
        }
    }
    server_to_return
}
