use k8s_openapi::api::{
    apps::v1::{Deployment, DeploymentSpec},
    core::v1::Namespace,
};

use crate::meta::namespaced_metadata;

pub fn deployment(namespace: &Namespace, name: String) -> Deployment {
    Deployment {
        metadata: namespaced_metadata(&namespace, name),
        spec: Some(DeploymentSpec {
            ..Default::default()
        }),
        ..Default::default()
    }
}
