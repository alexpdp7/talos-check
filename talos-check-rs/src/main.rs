use talos_check_rs::meta::namespace;
use talos_check_rs::rbac::{
    cluster_role, cluster_role_binding, policy_rule, service_account, AsRoleRef, AsSubject, Verb,
};
use talos_check_rs::workload::{container_port, deployment, Protocol, SetServiceAccount};

pub(crate) fn main() {
    let namespace_name = "foo";
    let ns = namespace(namespace_name);
    let service_account = service_account(&ns, "monitor");
    let get_list_nodes = policy_rule(vec!["nodes"], vec![Verb::List, Verb::Get]);
    let cluster_role = cluster_role(&format!("{namespace_name}-get-nodes"), vec![get_list_nodes]);
    let cluster_role_binding = cluster_role_binding(
        &format!("{namespace_name}-monitor"),
        cluster_role.as_ref(),
        vec![service_account.as_subject()],
    );
    let deployment = deployment(
        &ns,
        "monitor",
        "quay.io/alexpdp7/talos-check:latest",
        vec![container_port(8000, Protocol::TCP)],
    )
    .set_service_account(&service_account);
    println!("{}", serde_yaml::to_string(&ns).unwrap());
    println!("{}", serde_yaml::to_string(&service_account).unwrap());
    println!("{}", serde_yaml::to_string(&cluster_role).unwrap());
    println!("{}", serde_yaml::to_string(&cluster_role_binding).unwrap());
    println!("{}", serde_yaml::to_string(&deployment).unwrap());
}
