use aws_sdk_ecs::{operation::execute_command::ExecuteCommandOutput, Client};

#[derive(Debug)]
pub struct Cluster {
    pub name: String,
    pub arn: String,
}

pub async fn list_clusters(client: &Client) -> Result<Vec<Cluster>, String> {
    client
        .list_clusters()
        .send()
        .await
        .map(|resp| {
            resp.cluster_arns()
                .iter()
                .map(|arn| Cluster {
                    name: arn.split('/').last().unwrap_or("").to_string(),
                    arn: arn.to_string(),
                })
                .collect()
        })
        .map_err(|e| format!("Failed to list clusters: {:#?}", e))
}

pub async fn list_services(client: &Client, cluster: &str) -> Result<Vec<String>, String> {
    client
        .list_services()
        .cluster(cluster)
        .send()
        .await
        .map(|resp| resp.service_arns().to_vec())
        .map_err(|e| format!("Failed to list services: {}", e))
}

pub async fn list_tasks(
    client: &Client,
    cluster: &str,
    service: &str,
) -> Result<Vec<String>, String> {
    client
        .list_tasks()
        .cluster(cluster)
        .service_name(service)
        .send()
        .await
        .map(|resp| resp.task_arns().to_vec())
        .map_err(|e| format!("Failed to list tasks: {}", e))
}

pub async fn execute_command(
    client: &Client,
    cluster: &str,
    task: &str,
    container: &str,
    command: &str,
) -> Result<ExecuteCommandOutput, String> {
    client
        .execute_command()
        .cluster(cluster)
        .task(task)
        .container(container)
        .command(command)
        .interactive(true)
        .send()
        .await
        .map_err(|e| format!("Failed to execute command: {}", e))
}
