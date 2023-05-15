This projects helps keep a [Talos](https://www.talos.dev/) Kubernetes cluster updated.

The code includes a library that given the LIST and GET verbs permissions on Kubernetes nodes, can check a cluster Talos and Kubernetes versions.
It can also fetch the latest versions of Talos and Kubernetes from GitHub.

```
>>> import talos_check
>>> talos_check.get_cluster_talos_versions()
['v1.3.4']
>>> talos_check.get_available_talos_version()
'v1.3.6'
>>> talos_check.get_available_kubernetes_version()
'v1.26.3'
>>> talos_check.get_cluster_kubernetes_version()
'v1.26.1'
```

The code contains an httpd server that exposes this information as JSON.

```
$ talos-check-httpd
...
$ curl http://localhost:8000/
{
 "cluster_kubernetes_version": "v1.26.1",
 "cluster_talos_versions": [
  "v1.3.4"
 ],
 "status": "OK"
}
$ curl http://localhost:8000/available
{
 "cluster_kubernetes_version": "v1.26.1",
 "cluster_talos_versions": [
  "v1.3.4"
 ],
 "available_kubernetes_version": "v1.26.3",
 "available_talos_version": "v1.3.6",
 "needs_kubernetes_update": true,
 "outdated_talos_versions": [
  "v1.3.4"
 ],
 "needs_talos_update": true,
 "status": "NEEDS-KUBE-UPDATE-TO-v1.26.3-FROM-v1.26.1,NEEDS-TALOS-UPDATE-TO-v1.3.6-FROM-v1.3.4"
}
```

The code also includes an executable script that builds Kubernetes manifests to deploy the httpd server.
These manifests use the quay.io/alexpdp7/talos-check image.
The image also includes the manifest builder.

```
$ kubectl apply -f <(podman run --rm quay.io/alexpdp7/talos-check:latest talos-check-manifest-builder talos-check monitor)
```

The parameters correspond to the namespace to use for the manifests, and the host name to use.

The `manifest.yaml` file in the root of this repo contains the output of the previous command.

```
$ curl http://ingress.address/available --header "Host: monitor"
{
 "available_kubernetes_version": "v1.26.3",
 "cluster_kubernetes_version": "v1.26.1",
 "available_talos_version": "v1.3.6",
 "cluster_talos_versions": [
  "v1.3.4"
 ],
 "needs_kubernetes_update": true,
 "outdated_talos_versions": [
  "v1.3.4"
 ],
 "needs_talos_update": true,
 "status": "NEEDS-KUBE-UPDATE-TO-v1.26.3-FROM-v1.26.1,NEEDS-TALOS-UPDATE-TO-v1.3.6-FROM-v1.3.4"
```

You can use the `check_http` Nagios check to monitor a cluster for updates.

```
$ /usr/lib64/nagios/plugins/check_http -H monitor -I ingress.address -s OK
```
