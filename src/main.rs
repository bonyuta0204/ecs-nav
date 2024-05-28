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

    let cluster_selection = select_item(
        "Select a cluster",
        &clusters
            .iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<&str>>(),
    );
    let selected_cluster = &clusters[cluster_selection];
    println!("You selected cluster: {}", selected_cluster.name);

    let services = match list_services(&client, selected_cluster).await {
        Ok(services) => services,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let service_selection = select_item(
        "Select a service",
        &services
            .iter()
            .map(|service| service.name.as_str())
            .collect::<Vec<&str>>(),
    );
    let selected_service = &services[service_selection];
    println!("You selected service: {}", selected_service.name);

    let tasks = match list_tasks(&client, selected_cluster, selected_service).await {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let task_selection = select_item(
        "Select a task",
        &tasks
            .iter()
            .map(|task| task.arn.as_str())
            .collect::<Vec<&str>>(),
    );
    let selected_task = &tasks[task_selection];
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
