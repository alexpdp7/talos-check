use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{Namespace, Service, ServiceAccount};
use k8s_openapi::api::networking::v1::Ingress;
use k8s_openapi::api::rbac::v1::{ClusterRole, ClusterRoleBinding};
use talos_check_rs::meta::namespace;
use talos_check_rs::rbac::{
    cluster_role, cluster_role_binding, policy_rule, service_account, AsRoleRef, AsSubject, Verb,
};
use talos_check_rs::workload::{
    container_port, deployment, GetIngress, Protocol, SetServiceAccount,
};

struct TalosCheck {
    namespace: Namespace,
    service_account: ServiceAccount,
    cluster_role: ClusterRole,
    cluster_role_binding: ClusterRoleBinding,
    deployment: Deployment,
    service: Service,
    ingress: Ingress,
}

impl TalosCheck {
    fn create(namespace_name: &str, host_name: &str) -> TalosCheck {
        let namespace = namespace(namespace_name);
        let service_account = service_account(&namespace, "monitor");
        let get_list_nodes = policy_rule(vec!["nodes"], vec![Verb::List, Verb::Get], vec![""]);
        let cluster_role =
            cluster_role(&format!("{namespace_name}-get-nodes"), vec![get_list_nodes]);
        let cluster_role_binding = cluster_role_binding(
            &format!("{namespace_name}-monitor"),
            cluster_role.as_ref(),
            vec![service_account.as_subject()],
        );
        let (deployment, services) = deployment(
            &namespace,
            "monitor",
            "quay.io/alexpdp7/talos-check:latest",
            vec![container_port(8000, Protocol::TCP)],
        );
        let service = &services[0];
        let deployment = deployment.set_service_account(&service_account);
        let ingress = service.ingress(host_name);
        TalosCheck {
            namespace,
            service_account,
            cluster_role,
            cluster_role_binding,
            deployment,
            service: service.clone(),
            ingress,
        }
    }

    fn as_yaml(&self) -> String {
        format!(
            "---\n{}---\n{}---\n{}---\n{}---\n{}---\n{}---\n{}",
            serde_yaml::to_string(&self.namespace).unwrap(),
            serde_yaml::to_string(&self.service_account).unwrap(),
            serde_yaml::to_string(&self.cluster_role).unwrap(),
            serde_yaml::to_string(&self.cluster_role_binding).unwrap(),
            serde_yaml::to_string(&self.deployment).unwrap(),
            serde_yaml::to_string(&self.service).unwrap(),
            serde_yaml::to_string(&self.ingress).unwrap(),
        )
    }
}

pub(crate) fn main() {
    let check = TalosCheck::create("talos-check", "monitor");
    print!("{}", check.as_yaml());
}
