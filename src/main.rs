mod aws_client;
mod cli;
mod ecs;

use aws_client::create_client;
use cli::select_item;
use ecs::{execute_command, list_clusters, list_services, list_tasks};
use tokio;

#[tokio::main]
async fn main() {
    let client = create_client().await;

    let clusters = match list_clusters(&client).await {
        Ok(clusters) => clusters,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let selected_cluster = select_item("Select a cluster", &clusters);
    println!("You selected cluster: {}", selected_cluster.name);

    let services = match list_services(&client, selected_cluster).await {
        Ok(services) => services,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let selected_service = select_item("Select a service", &services);
    println!("You selected service: {}", selected_service.name);

    let tasks = match list_tasks(&client, selected_cluster, selected_service).await {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let selected_task = select_item("Select a task", &tasks);
    println!("You selected task: {}", selected_task.arn);

    let container_name = "your-container-name"; // Replace this with logic to fetch container name

    match execute_command(
        &client,
        selected_cluster.name.as_str(),
        selected_task.arn.as_str(),
        container_name,
        "your-command-here",
    )
    .await
    {
        Ok(_) => println!("Command executed successfully"),
        Err(e) => eprintln!("{}", e),
    }
}
