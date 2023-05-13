use k8s_openapi::api::{
    apps::v1::{Deployment, DeploymentSpec},
    core::v1::Namespace,
};

use crate::meta::{namespaced_metadata, AddLabel};

pub fn deployment(namespace: &Namespace, name: &str) -> Deployment {
    Deployment {
        metadata: namespaced_metadata(namespace, name).add_label("app", name),
        spec: Some(DeploymentSpec {
            ..Default::default()
        }),
        ..Default::default()
    }
}
