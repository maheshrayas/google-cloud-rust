use std::sync::Arc;

use google_cloud_rust_raw::container::v1::{
    cluster_service::ListClustersRequest, cluster_service_grpc::ClusterManagerClient,
};

use grpcio::{Channel, ChannelBuilder, ChannelCredentials, EnvBuilder};
use std::error::Error;

fn connect(endpoint: &str) -> Channel {
    // Set up the gRPC environment.
    let env = Arc::new(EnvBuilder::new().build());
    let creds =
        ChannelCredentials::google_default_credentials().expect("No Google credentials found");

    // Create a channel to connect to Gcloud.
    ChannelBuilder::new(env)
        // Set the max size to correspond to server-side limits.
        .max_send_message_len(1 << 28)
        .max_receive_message_len(1 << 28)
        .secure_connect(endpoint, creds)
}

fn main() -> Result<(), Box<dyn Error>> {
    let endpoint = "container.googleapis.com";
    let channel = connect(endpoint);
    let client = ClusterManagerClient::new(channel);
    let mut req = ListClustersRequest::new();
    req.set_project_id("x-vector-307507".to_string());
    req.set_zone("australia-southeast1-a".to_string());
    let x = client.list_clusters(&req);
    match x {
        Ok(e) => {
            println!("hi {:?}", e);
        }
        Err(e) => eprintln!("{}", e),
    }

    // req.set_parent(format!("projects/{}", "mozilla-rust-sdk-dev"));
    //
    // match client.list_assets(&req) {
    //     Ok(response) => {
    //         response
    //             .get_assets()
    //             .iter()
    //             .for_each(|asset| println!("  Asset: {:?}", asset));
    //     },
    //     Err(error) => println!("Failed to list assets: {}", error),
    // }

    Ok(())
}
