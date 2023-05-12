use k8s_openapi::api::{
    core::v1::{Namespace, ServiceAccount},
    rbac::v1::{ClusterRole, ClusterRoleBinding, PolicyRule, RoleRef, Subject},
};

use crate::meta::{metadata, namespaced_metadata};

pub fn service_account(namespace: &Namespace, name: String) -> ServiceAccount {
    ServiceAccount {
        metadata: namespaced_metadata(namespace, name),
        ..Default::default()
    }
}

pub fn cluster_role(name: String, rules: Vec<PolicyRule>) -> ClusterRole {
    ClusterRole {
        metadata: metadata(name),
        rules: Some(rules),
        ..Default::default()
    }
}

pub fn policy_rule(resources: Vec<String>, verbs: Vec<Verb>) -> PolicyRule {
    PolicyRule {
        resources: Some(resources),
        verbs: verbs.iter().map(|v| v.to_string()).collect(),
        ..Default::default()
    }
}

pub enum Verb {
    List,
    Get,
}

impl std::string::ToString for Verb {
    fn to_string(self: &Self) -> String {
        match self {
            Verb::List => "list",
            Verb::Get => "get",
        }
        .to_string()
    }
}

pub trait AsRoleRef {
    fn as_ref(self: &Self) -> RoleRef;
}

impl AsRoleRef for ClusterRole {
    fn as_ref(self: &Self) -> RoleRef {
        RoleRef {
            api_group: "rbac.authorization.k8s.io".to_string(),
            kind: "ClusterRole".to_string(),
            name: self.metadata.name.clone().unwrap(),
            ..Default::default()
        }
    }
}

pub trait AsSubject {
    fn as_subject(self: &Self) -> Subject;
}

impl AsSubject for ServiceAccount {
    fn as_subject(self: &Self) -> Subject {
        Subject {
            kind: "ServiceAccount".to_string(),
            name: self.metadata.name.clone().unwrap(),
            namespace: Some(self.metadata.namespace.clone().unwrap()),
            ..Default::default()
        }
    }
}

pub fn cluster_role_binding(
    name: String,
    role_ref: RoleRef,
    subjects: Vec<Subject>,
) -> ClusterRoleBinding {
    ClusterRoleBinding {
        metadata: metadata(name),
        role_ref,
        subjects: Some(subjects),
        ..Default::default()
    }
}
