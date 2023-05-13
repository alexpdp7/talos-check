use std::collections::BTreeMap;

use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{Container, Namespace, PodSpec, PodTemplateSpec, ServiceAccount},
    },
    apimachinery::pkg::apis::meta::v1::LabelSelector,
};
use kube::core::ObjectMeta;

use crate::meta::{namespaced_metadata, AddLabel};

pub fn deployment(namespace: &Namespace, name: &str, image: &str) -> Deployment {
    let labels = Some(BTreeMap::from([("app".into(), name.into())]));
    Deployment {
        metadata: namespaced_metadata(namespace, name).add_label("app", name),
        spec: Some(DeploymentSpec {
            selector: LabelSelector {
                match_labels: labels.clone(),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels,
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![Container {
                        name: name.into(),
                        image: Some(image.into()),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
            },
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub trait SetServiceAccount {
    fn set_service_account(&self, name: &ServiceAccount) -> Self;
}

impl SetServiceAccount for Deployment {
    fn set_service_account(&self, account: &ServiceAccount) -> Self {
        let mut result = self.clone();
        result
            .spec
            .as_mut()
            .unwrap()
            .template
            .spec
            .as_mut()
            .unwrap()
            .service_account = account.metadata.name.clone();
        result
            .spec
            .as_mut()
            .unwrap()
            .template
            .spec
            .as_mut()
            .unwrap()
            .service_account_name = account.metadata.name.clone();
        result
    }
}
