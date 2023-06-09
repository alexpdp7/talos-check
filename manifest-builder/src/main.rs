use clap::Parser;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::{Namespace, Service, ServiceAccount};
use k8s_openapi::api::networking::v1::Ingress;
use k8s_openapi::api::rbac::v1::{ClusterRole, ClusterRoleBinding};
use krust_manifesto::meta::*;
use krust_manifesto::rbac::*;
use krust_manifesto::util::{combine_yamls, to_yaml};
use krust_manifesto::workload::*;

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

    fn yamls(&self) -> Vec<String> {
        vec![
            to_yaml(&self.namespace),
            to_yaml(&self.service_account),
            to_yaml(&self.cluster_role),
            to_yaml(&self.cluster_role_binding),
            to_yaml(&self.deployment),
            to_yaml(&self.service),
            to_yaml(&self.ingress),
        ]
    }

    fn as_yaml(&self) -> String {
        combine_yamls(self.yamls())
    }
}

#[derive(Parser)]
struct Args {
    namespace: String,
    host_name: String,
}

pub(crate) fn main() {
    let args = Args::parse();
    let check = TalosCheck::create(&args.namespace, &args.host_name);
    print!("{}", check.as_yaml());
}

#[test]
fn test() {
    let generated = TalosCheck::create("talos-check", "monitor").as_yaml();
    let expected = std::fs::read_to_string("../manifest.yaml").unwrap();
    assert_eq!(generated, expected);
}
