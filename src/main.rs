mod aws_client;
mod cli;
mod ecs;

use aws_client::create_client;
use cli::select_item;
use clap::Parser;
use ecs::{execute_command, list_clusters, list_containers, list_services, list_tasks};
use tokio;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Shell to use for connecting to the container
    #[arg(short, long, default_value = "/bin/bash")]
    shell: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client = create_client().await;

    let clusters = match list_clusters(&client).await {
        Ok(clusters) => clusters,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let selected_cluster = select_item("Select a cluster", &clusters);

    let services = match list_services(&client, selected_cluster).await {
        Ok(services) => services,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let selected_service = select_item("Select a service", &services);

    let tasks = match list_tasks(&client, selected_cluster, selected_service).await {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // Skip selecting task if there is only one task available
    let selected_task = if tasks.len() == 1 {
        &tasks[0]
    } else {
        select_item("Select a task", &tasks)
    };

    let containers = match list_containers(&client, selected_cluster, selected_task).await {
        Ok(containers) => containers,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let selected_container = select_item("Select a container", &containers);

    // Execute the command with the specified shell
    println!("Connecting to container with shell: {}", args.shell);
    match execute_command(
        selected_cluster,
        selected_task,
        selected_container,
        &args.shell,
    ) {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to execute: {}", e),
    }
}
