import kubernetes
import requests
import typing
import yaml


def get_cluster_kubernetes_version():
    kubernetes.config.load_config()
    return kubernetes.client.VersionApi().get_code().git_version


def get_cluster_talos_versions():
    kubernetes.config.load_config()
    ns = kubernetes.client.CoreV1Api().list_node()
    return set([_parse_talos_os_image(n.status.node_info.os_image)["version"] for n in ns.items])


def get_cluster_talos_major_versions():
    return set([_full_to_minor(v) for v in get_cluster_talos_versions()])


def get_available_talos_version():
    return requests.get("https://api.github.com/repos/siderolabs/talos/releases/latest").json()["tag_name"]


def _parse_talos_os_image(os_image):
    version = os_image.split(" ")[1][1:-1]
    return {
        "version": version
    }


def _get_kubernetes_schedules():
    return yaml.load(requests.get("https://raw.githubusercontent.com/kubernetes/website/main/data/releases/schedule.yaml").text, Loader=yaml.SafeLoader)


def get_cluster_earlier_talos_major():
    majors = get_cluster_talos_major_versions()
    majors = list(map(_version_as_numeric_tuple, majors))
    sorted_majors = sorted(majors)
    return sorted_majors[0]


def get_latest_available_supported_kubernetes_version():
    """
    Returns a dict like:

    {'1.27': '1.27.2', '1.26': '1.26.5', '1.25': '1.25.10'}

    One key per Kubernetes major supported by the node running the earlier Talos major.
    The value is the latest available Kubernetes version for that major.

    On the example above, the cluster is running Talos 1.4.
    Talos 1.4 supports Kubernetes 1.25 to 1.27.
    The latest Kubernetes 1.27 is 1.27.2.
    """
    supported_kubernetes_versions = _TALOS_MAJOR_TO_KUBE_MAJOR[get_cluster_earlier_talos_major()]
    schedules = _get_kubernetes_schedules()

    supported_latest = {}

    for schedule in schedules["schedules"]:
        release = str(schedule["release"])
        if release in supported_kubernetes_versions:
            release_latest_patch = schedule["previousPatches"][0]["release"]
            supported_latest[release] = release_latest_patch
    return supported_latest


def _full_to_minor(version):
    """
    >>> _full_to_minor("v1.4.4")
    '1.4'
    """
    first, second, third = version[1:].split(".")
    return f"{first}.{second}"


def _version_as_numeric_tuple(version):
    """
    >>> _version_as_numeric_tuple("1.4")
    (1, 4)
    """
    return tuple([int(part) for part in version.split(".")])


_TALOS_MAJOR_TO_KUBE_MAJOR = {
    (1, 4): ["1.25", "1.26", "1.27"],
    (1, 5): ["1.26", "1.27", "1.28"],
    (1, 6): ["1.29", "1.28", "1.27", "1.26", "1.25", "1.24"],
    (1, 7): ["1.30", "1.29", "1.28", "1.27", "1.26", "1.25"],
    (1, 8): ["1.31", "1.30", "1.29", "1.28", "1.27", "1.26"],
    (1, 9): ["1.32", "1.31", "1.30", "1.29", "1.28", "1.27"],
}
