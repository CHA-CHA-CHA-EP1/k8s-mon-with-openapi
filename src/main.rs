use k8s_openapi::api::core::v1::Pod;
use kube::{Client, Api, api::ListParams};

use k8s_mon::configs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    //** Load Config env  **/
    let config = configs::config::Config::default()?;

    let client = Client::try_default().await?;

    let pods: Api<Pod> = Api::namespaced(client, &config.namespace);
    for p in pods.list(&ListParams::default()).await? {
        let container_status = p.status.unwrap().container_statuses.unwrap();
        for container in container_status {
            let container_name = container.name;
            let container_state = container.state.unwrap();

            if let Some(_) = container_state.running {
                println!("{} is running", container_name);
            } else if let Some(waiting) = container_state.waiting {
                println!("{} is waiting: {}", container_name, waiting.reason.unwrap());
            } else if let Some(terminated) = container_state.terminated {
                println!("{} is terminated: {}", container_name, terminated.reason.unwrap());
            }
        }
    }
    Ok(())
}
