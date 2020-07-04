use hcloud::apis::client::APIClient;
use hcloud::apis::configuration::Configuration;
use hcloud::apis::servers_api::{
    CreateServerParams, DeleteServerParams, ListActionsForServerParams,
};
use hcloud::models;
use rand::{distributions, thread_rng, Rng};
use std::{env, thread, time};

// server type and location to be used for creating the servers
const SERVER_TYPE: &str = "cx11";
const LOCATION: &str = "fsn1";
const IMAGE: &str = "ubuntu-18.04";

// number of servers to create during the example
const NUMBER_OF_SERVERS: u32 = 2;

// struct to track some information about our servers
struct ServerInfo {
    id: i32,
    name: String,
    ipv4: String,
}

fn main() -> Result<(), String> {
    // use API token from command line
    let api_token = env::args()
        .nth(1)
        .ok_or("Please provide API token as command line parameter.")?;

    // set up basic configuration using provided API token
    let mut configuration = Configuration::new();
    configuration.bearer_access_token = Some(api_token);

    // create API client handle from configuration
    let api_client = APIClient::new(configuration);

    // collect all available SSH keys to be added to the server
    let ssh_keys: Vec<String> = api_client
        .ssh_keys_api()
        .list_ssh_keys(Default::default())
        .map_err(|err| format!("API call to list_ssh_keys failed: {:?}", err))?
        .ssh_keys
        .into_iter()
        .map(|ssh_key| ssh_key.name)
        .collect();

    // store server info of created servers for querying
    // corresponding actions and finally deleting the servers
    let mut created_servers = Vec::new();

    println!("Creating {} example servers...", NUMBER_OF_SERVERS);
    for _ in 0..NUMBER_OF_SERVERS {
        let mut name = "example-server-".to_string();
        name.push_str(
            // append random postfix
            &thread_rng()
                .sample_iter(distributions::Alphanumeric)
                .take(8)
                .collect::<String>(),
        );
        println!(" Creating server \"{}\"...", name);
        let request = models::CreateServerRequest {
            name,
            server_type: SERVER_TYPE.to_string(),
            start_after_create: Some(true),
            image: IMAGE.to_string(),
            ssh_keys: Some(ssh_keys.clone()),
            volumes: None,
            networks: None,
            user_data: None,
            labels: None,
            automount: None,
            location: Some(LOCATION.to_string()),
            datacenter: None,
        };

        // execute request and store server ID
        let params = CreateServerParams {
            create_server_request: Some(request),
        };
        let response = api_client
            .servers_api()
            .create_server(params)
            .map_err(|err| format!("API call to create_server failed: {:?}", err))?;

        created_servers.push(ServerInfo {
            id: response.server.id,
            name: response.server.name,
            ipv4: response.server.public_net.ipv4.ip,
        });
    }
    println!();
    println!("Server info of created servers:");
    for server_info in &created_servers {
        println!(
            " id: {}, name: {}, ipv4: {}",
            server_info.id, server_info.name, server_info.ipv4
        );
    }
    println!();

    println!("Wait for servers to be ready by polling corresponding actions...");
    loop {
        let mut any_running = false;
        for server_info in &created_servers {
            let params = ListActionsForServerParams {
                id: server_info.id.to_string(),
                ..Default::default()
            };
            let actions = api_client
                .servers_api()
                .list_actions_for_server(params)
                .map_err(|err| format!("API call to list_actions_for_server failed: {:?}", err))?
                .actions;
            println!(" Actions for server {}:", server_info.name);
            for action in actions {
                println!(
                    "  id: {}, command: {}, status: {:?}, progress: {}",
                    action.id, action.command, action.status, action.progress
                );
                if action.status == models::action::Status::Running {
                    any_running = true;
                }
            }
        }
        if !any_running {
            break;
        }
        println!("Some actions are still running, let's wait some time and check again...");
        thread::sleep(time::Duration::from_secs(1));
    }
    println!("All actions have finished. The servers should be up and running now!");
    println!();

    // maybe do something useful with the servers here?

    println!("Deleting servers...");
    for server_info in &created_servers {
        println!(" Deleting server {}...", server_info.name);
        let params = DeleteServerParams {
            id: server_info.id.to_string(),
        };
        api_client
            .servers_api()
            .delete_server(params)
            .map_err(|err| format!("API call to delete_server failed: {:?}", err))?;
    }
    println!("The servers should be deleted now!");

    Ok(())
}
