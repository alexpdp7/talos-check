This projects helps keep a [Talos](https://www.talos.dev/) Kubernetes cluster updated.

The code includes a library that given the LIST and GET verbs permissions on Kubernetes nodes, can check a cluster Talos and Kubernetes versions.
It can also fetch the latest versions of Talos and Kubernetes from GitHub.

```
>>> import talos_check

>>> talos_check.get_cluster_talos_versions()
{'v1.4.4'}

>>> talos_check.get_cluster_talos_major_versions()
{'1.4'}

>>> talos_check.get_cluster_earlier_talos_major()
(1, 4)

>>> talos_check.get_available_talos_version()
'v1.4.4'

>>> talos_check.get_cluster_kubernetes_version()
'v1.27.1'

>>> talos_check.get_available_supported_kubernetes_version()
{'1.27': '1.27.2', '1.26': '1.26.5', '1.25': '1.25.10'}
```

The code contains an http server that exposes this information as JSON.

```
$ talos-check-httpd
...
$ curl http://localhost:8000/
{
 "talos_versions": [
  "v1.4.4"
 ],
 "available_talos_version": "v1.4.4",
 "kubernetes_version": "v1.27.1",
 "latest_available_supported_kubernetes_versions": {
  "1.27": "1.27.2",
  "1.26": "1.26.5",
  "1.25": "1.25.10"
 }
}
```

The code contains a Nagios check.

```
$ ./check_talos_version http://localhost:8000 host_header
WARNING Running outdated Kubernetes version v1.27.1, update to v1.27.2. Running Talos v1.4.4; available v1.4.4. Running Kubernetes v1.27.1, available v1.27.2.
$ echo $?
1
```

The `manifest.yaml` file in the root of this repo contains Kubernetes manifests to deploy the http server.
The manifests create a `talos-check` namespace and use the `monitor` host.

```
$ curl http://ingress.address/available --header "Host: monitor"
{
 "talos_versions": [
  "v1.4.4"
 ],
 "available_talos_version": "v1.4.4",
 "kubernetes_version": "v1.27.1",
 "latest_available_supported_kubernetes_versions": {
  "1.27": "1.27.2",
  "1.26": "1.26.5",
  "1.25": "1.25.10"
 }
}
```
