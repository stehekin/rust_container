use anyhow::Result;
use k8s_cri::v1::runtime_service_client::RuntimeServiceClient;

struct Cri {
}

impl Cri {
    async fn new() -> Result<()>{
        let mut _client = RuntimeServiceClient::connect("unix:///run/user/0/podman/podman.sock").await?;
        Ok(())
    }
}