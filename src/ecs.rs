use aws_sdk_ecs::error::{ProvideErrorMetadata, SdkError};
use aws_sdk_ecs::Client;
use nix::unistd::execvp;

use std::ffi::CString;
use std::fmt;

trait FromArn {
    fn new(arn: &str, name: &str) -> Self;
    fn from_arn(arn: &str) -> Self
    where
        Self: Sized,
    {
        Self::new(arn, arn.split('/').last().unwrap_or_default())
    }
}

#[derive(Debug)]
pub struct Cluster {
    pub name: String,
    pub arn: String,
}

impl FromArn for Cluster {
    fn new(arn: &str, name: &str) -> Self {
        Cluster {
            arn: arn.to_string(),
            name: name.to_string(),
        }
    }
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

impl FromArn for Service {
    fn new(arn: &str, name: &str) -> Self {
        Service {
            arn: arn.to_string(),
            name: name.to_string(),
        }
    }
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

impl FromArn for Task {
    fn new(arn: &str, name: &str) -> Self {
        Task {
            arn: arn.to_string(),
            name: name.to_string(),
        }
    }
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
                .map(|arn| Cluster::from_arn(arn))
                .collect()
        })
        .map_err(|e| format!("Failed to list clusters: {}", error_message(e)))
}

fn error_message<E, R>(e: SdkError<E, R>) -> String
where
    E: fmt::Display + ProvideErrorMetadata,
    R: fmt::Debug,
{
    match e {
        SdkError::ServiceError(e) => e.err().message().unwrap_or("").to_string(),
        _ => format!("{}", e),
    }
}

pub async fn list_services(client: &Client, cluster: &Cluster) -> Result<Vec<Service>, String> {
    let mut services = Vec::new();
    let paginator = client
        .list_services()
        .cluster(cluster.name.as_str())
        .into_paginator()
        .send();

    let service_arns = paginator
        .try_collect()
        .await
        .map_err(|e| format!("Failed to list services: {}", error_message(e)))?;

    for page in service_arns {
        services.extend(page.service_arns().iter().map(|arn| Service::from_arn(arn)));
    }

    Ok(services)
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
                .map(|arn| Task::from_arn(arn))
                .collect()
        })
        .map_err(|e| format!("Failed to list tasks: {}", error_message(e)))
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
        .map_err(|e| format!("Failed to list containers: {}", error_message(e)))
}

use std::io::Error;

pub fn execute_command(
    cluster: &Cluster,
    task: &Task,
    container: &Container,
    command: &str,
) -> Result<(), Error> {
    let program = CString::new("aws").unwrap();
    let args = vec![
        program.clone(), // Include the program name as the first argument
        CString::new("ecs").unwrap(),
        CString::new("execute-command").unwrap(),
        CString::new("--cluster").unwrap(),
        CString::new(cluster.name.as_str()).unwrap(),
        CString::new("--task").unwrap(),
        CString::new(task.arn.as_str()).unwrap(),
        CString::new("--container").unwrap(),
        CString::new(container.name.as_str()).unwrap(),
        CString::new("--command").unwrap(),
        CString::new(command).unwrap(),
        CString::new("--interactive").unwrap(),
    ];

    // Convert Vec<CString> to &[CString] and pass to execvp
    let c_args: Vec<&CString> = args.iter().collect();

    match execvp(&program, &c_args) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
    }
}
