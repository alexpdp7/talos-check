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

```
$ talos-check-httpd
...
[alex@molly talos-check]$ curl http://localhost:8000/
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

```
$ kubectl create ns monitor
$ kubectl config set-context --namespace monitor --current
$ kubectl create sa monitor
$ kubectl create clusterrole get-nodes --verb=get,list --resource=node
$ kubectl create clusterrolebinding monitor --clusterrole=get-nodes --serviceaccount=monitor:monitor
$ kubectl create deployment monitor --image=quay.io/alexpdp7/talos-check
$ kubectl set serviceaccount deployment monitor monitor
$ kubectl expose deployment monitor --port 8000
$ kubectl create ingress monitor --rule "monitor/=monitor:8000"
...
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
...
$ /usr/lib64/nagios/plugins/check_http -H monitor -I ingress.address -s OK
```
