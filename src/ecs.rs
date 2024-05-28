use aws_sdk_ecs::{operation::execute_command::ExecuteCommandOutput, Client};

use std::fmt;

#[derive(Debug)]
pub struct Cluster {
    pub name: String,
    pub arn: String,
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub arn: String,
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub arn: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub struct Container {
    pub name: String,
    pub arn: String,
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
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

pub async fn list_services(client: &Client, cluster: &Cluster) -> Result<Vec<Service>, String> {
    client
        .list_services()
        .cluster(cluster.name.as_str())
        .send()
        .await
        .map(|resp| {
            resp.service_arns()
                .iter()
                .map(|arn| Service {
                    name: arn.split('/').last().unwrap_or("").to_string(),
                    arn: arn.to_string(),
                })
                .collect()
        })
        .map_err(|e| format!("Failed to list services: {}", e))
}

pub async fn list_tasks(
    client: &Client,
    cluster: &Cluster,
    service: &Service,
) -> Result<Vec<Task>, String> {
    client
        .list_tasks()
        .cluster(cluster.name.as_str())
        .service_name(service.name.as_str())
        .send()
        .await
        .map(|resp| {
            resp.task_arns()
                .iter()
                .map(|arn| Task {
                    name: arn.split('/').last().unwrap_or("").to_string(),
                    arn: arn.to_string(),
                })
                .collect()
        })
        .map_err(|e| format!("Failed to list tasks: {}", e))
}

pub async fn list_containers(
    client: &Client,
    cluster: &Cluster,
    task: &Task,
) -> Result<Vec<Container>, String> {
    client
        .describe_tasks()
        .cluster(cluster.name.as_str())
        .tasks(task.arn.as_str())
        .send()
        .await
        .map(|resp| {
            resp.tasks()
                .iter()
                .flat_map(|task| task.containers().iter())
                .map(|container| Container {
                    name: container.name().unwrap_or("").to_string(),
                    arn: container.container_arn().unwrap_or("").to_string(),
                })
                .collect()
        })
        .map_err(|e| format!("Failed to list containers: {}", e))
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
