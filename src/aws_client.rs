use aws_config::{meta::region::RegionProviderChain, Region};
use aws_sdk_ecs::Client;

pub async fn create_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-west-2"));
    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}
