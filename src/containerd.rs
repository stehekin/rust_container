#[cfg(test)]
mod test {
    use std::any::Any;

    use containerd_client::connect;
    use containerd_client::services::v1::containers_client::ContainersClient;
    use containerd_client::services::v1::namespaces_client::NamespacesClient;
    use containerd_client::services::v1::{ListContainersRequest, ListNamespacesRequest};
    use containerd_client::types::RuntimeInfo;
    use containerd_client::with_namespace;
    use prost::{Message, Name};
    use prost_helper::ToJson;
    use serde::Deserialize;
    use serde_json::Value;
    use tonic::{IntoRequest, Request, Response};

    use oci_spec::runtime::{Spec, SpecBuilder};

    #[tokio::test(flavor = "multi_thread")]
    async fn test_connect_containerd() {
        let channel = connect("/proc/16105/root/run/containerd/containerd.sock")
            .await
            .expect("cannt connect");

        // let mut ns_client = NamespacesClient::new(channel);
        // let ns_request = ListNamespacesRequest {
        //     filter: String::from(""),
        // };
        // let ns_response = ns_client.list(ns_request).await.expect("errpr");
        // print!("ns_response: {:?}", ns_response);

        let mut client = ContainersClient::new(channel.clone());
        let request = ListContainersRequest::default();
        let request = with_namespace!(request, "k8s.io");
        let response = client.list(request).await.unwrap();
        let response = response.get_ref();
        for c in &response.containers {
            let spec_any = &c.spec;

            let spec_any = match spec_any {
                Some(any) => any,
                None => {
                    println!("Spec is None for container: {}", c.id);
                    continue; // Skip to the next container
                }
            };

            // Attempt to decode the Any message into an oci_spec::Spec
            let json_value: Value = serde_json::from_slice(&spec_any.value).expect("msg");
            let spec: Spec = Spec::deserialize(&json_value).expect("msg");
            //     .map_err(|e| format!("Failed to deserialize spec: {}", e))?;

            // let spec = c.spec.as_ref().unwrap();
            print!("{:?}\n", spec.linux().as_ref().unwrap().namespaces());
        }
    }
}
