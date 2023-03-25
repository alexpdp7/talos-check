```
>>> import talos
>>> talos.get_cluster_talos_versions()
['v1.3.4']
>>> talos.get_available_talos_version()
'v1.3.6'
>>> talos.get_available_kubernetes_version()
'v1.26.3'
>>> talos.get_cluster_kubernetes_version()
'v1.26.1'
```

```
$ poetry run python -m talos.http
...
$ curl http://localhost:8000/
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
 "needs_talos_update": false,
 "status": "NEEDS-KUBE-UPDATE-TO-v1.26.3-FROM-v1.26.1"
}[alex@molly talos-check]$ curl http://localhost:8000/
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