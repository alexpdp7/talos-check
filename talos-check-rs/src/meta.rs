use k8s_openapi::api::core::v1::Namespace;
use kube::core::ObjectMeta;

pub fn namespace(name: String) -> Namespace {
    Namespace {
        metadata: metadata(name),
        ..Default::default()
    }
}

pub fn metadata(name: String) -> ObjectMeta {
    ObjectMeta {
        name: Some(name),
        ..Default::default()
    }
}

pub fn namespaced_metadata(namespace: &Namespace, name: String) -> ObjectMeta {
    ObjectMeta {
        name: Some(name),
        namespace: namespace.metadata.name.clone(),
        ..Default::default()
    }
}
