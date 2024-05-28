mod aws_client;

use aws_client::create_client;
use dialoguer::Select;
use tokio;

#[tokio::main]
async fn main() {
    let client = create_client().await;

    let clusters = match client.list_clusters().send().await {
        Ok(resp) => resp.cluster_arns().to_vec(),
        Err(e) => {
            eprintln!("Failed to list clusters: {:#?}", e);
            return;
        }
    };

    let cluster_names: Vec<&str> = clusters
        .iter()
        .map(|s| s.split('/').last().unwrap())
        .collect();

    let selection = Select::new()
        .with_prompt("Select a cluster")
        .items(&cluster_names)
        .default(0)
        .interact()
        .unwrap();

    let selected_cluster = &clusters[selection];
    println!("You selected cluster: {}", selected_cluster);

    let services = match client
        .list_services()
        .cluster(selected_cluster)
        .send()
        .await
    {
        Ok(resp) => resp.service_arns().to_vec(),
        Err(e) => {
            eprintln!("Failed to list services: {}", e);
            return;
        }
    };

    let service_names: Vec<&str> = services
        .iter()
        .map(|s| s.split('/').last().unwrap())
        .collect();

    let service_selection = Select::new()
        .with_prompt("Select a service")
        .items(&service_names)
        .default(0)
        .interact()
        .unwrap();

    let selected_service = &services[service_selection];
    println!("You selected service: {}", selected_service);

    let tasks = match client
        .list_tasks()
        .cluster(selected_cluster)
        .service_name(selected_service)
        .send()
        .await
    {
        Ok(resp) => resp.task_arns().to_vec(),
        Err(e) => {
            eprintln!("Failed to list tasks: {}", e);
            return;
        }
    };

    let task_arns: Vec<&str> = tasks.iter().map(|s| s.as_str()).collect();

    let task_selection = Select::new()
        .with_prompt("Select a task")
        .items(&task_arns)
        .default(0)
        .interact()
        .unwrap();

    let selected_task = &tasks[task_selection];
    println!("You selected task: {}", selected_task);

    let container_name = "your-container-name"; // Replace this with logic to fetch container name

    let exec_command = client
        .execute_command()
        .cluster(selected_cluster)
        .task(selected_task)
        .container(container_name)
        .command("your-command-here")
        .interactive(true);

    match exec_command.send().await {
        Ok(_) => println!("Command executed successfully"),
        Err(e) => eprintln!("Failed to execute command: {}", e),
    }
}
