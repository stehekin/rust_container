#[cfg(test)]
mod test {
    use hyper_util::rt::TokioIo;
    use k8s_cri::v1::runtime_service_client::RuntimeServiceClient;
    use k8s_cri::v1::ListContainersRequest;
    use tokio::net::UnixStream;
    use tonic::transport::{Channel, Endpoint, Uri};
    use tonic::{Request, Response};
    use tower::service_fn;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_connect_cri() {
        let channel = Endpoint::try_from("http://[::]")
            .unwrap()
            .connect_with_connector(service_fn(|_: Uri| async {
                Ok::<_, std::io::Error>(TokioIo::new(
                    UnixStream::connect("/proc/5312/root/run/containerd/containerd.sock").await?,
                ))
            }))
            .await
            .expect("Could not create client.");

        let mut client = RuntimeServiceClient::new(channel);

        let request = tonic::Request::new(ListContainersRequest { filter: None });
        let response = client
            .list_containers(request)
            .await
            .expect("Request failed.");
        for c in &response.get_ref().containers {
            println!("{:?}\n", c);
        }
    }
}
