import kubernetes
import requests

kubernetes.config.load_config()


def get_cluster_kubernetes_version():
    return kubernetes.client.VersionApi().get_code().git_version


def get_available_kubernetes_version():
    return requests.get("https://api.github.com/repos/kubernetes/kubernetes/releases/latest").json()["tag_name"]


def get_cluster_talos_versions():
    ns = kubernetes.client.CoreV1Api().list_node()
    return [parse_talos_os_image(n.status.node_info.os_image)["version"] for n in ns.items]


def parse_talos_os_image(os_image):
    version = os_image.split(" ")[1][1:-1]
    return {
        "version": version
    }


def get_available_talos_version():
    return requests.get("https://api.github.com/repos/siderolabs/talos/releases/latest").json()["tag_name"]
