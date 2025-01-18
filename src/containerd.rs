use containerd_client::connect;
use containerd_client::with_namespace;
use containerd_client::services::v1::namespaces_client::NamespacesClient;
use containerd_client::services::v1::ListNamespacesRequest;
use containerd_client::services::v1::containers_client::ContainersClient;
use containerd_client::services::v1::ListContainersRequest;

#[cfg(test)]
mod test {
    use containerd_client::connect;
    use containerd_client::with_namespace;
    use containerd_client::services::v1::containers_client::ContainersClient;
    use containerd_client::services::v1::{ListContainersRequest, ListNamespacesRequest};
    use containerd_client::services::v1::namespaces_client::NamespacesClient;
    use tonic::Request;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_connect() {
        let channel = connect("/run/containerd/containerd.sock").await.expect("cannt connect");

        // let mut ns_client = NamespacesClient::new(channel);
        // let ns_request = ListNamespacesRequest{
        //     filter: String::from(""),
        // };
        // let ns_response = ns_client.list(ns_request).await.expect("errpr");
        // print!("ns_response: {:?}", ns_response);

        let mut client = ContainersClient::new(channel.clone());
        let request = ListContainersRequest::default();
        let request = with_namespace!(request, "moby");
        let response = client.list(request).await.unwrap();
        print!("response: {:?} \n", response);
    }
}