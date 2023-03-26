import json

import gunicorn.app.base

import talos_check


def build_cluster_versions(o):
    o["cluster_kubernetes_version"] = talos_check.get_cluster_kubernetes_version()
    o["cluster_talos_versions"] = talos_check.get_cluster_talos_versions()
    return o


def build_available_versions(o):
    o["available_kubernetes_version"] = talos_check.get_available_kubernetes_version()
    o["available_talos_version"] = talos_check.get_available_talos_version()

    o["needs_kubernetes_update"] = o["available_kubernetes_version"] != o["cluster_kubernetes_version"]
    o["outdated_talos_versions"] = [ctv for ctv in o["cluster_talos_versions"] if ctv != o["available_talos_version"]]
    o["needs_talos_update"] = len(o["outdated_talos_versions"]) > 0
    return o


def app(environ, start_response):
    o = dict()
    o = build_cluster_versions(o)

    if environ["PATH_INFO"] == "/available":
        o = build_available_versions(o)

    statuses = []
    if o.get("needs_kubernetes_update"):
        statuses.append(f"NEEDS-KUBE-UPDATE-TO-{o['available_kubernetes_version']}-FROM-{o['cluster_kubernetes_version']}")
    if o.get("needs_talos_update"):
        outdated_talos_versions = ",".join(set(o["outdated_talos_versions"]))
        statuses.append(f"NEEDS-TALOS-UPDATE-TO-{o['available_talos_version']}-FROM-{outdated_talos_versions}")
    if not statuses:
        statuses = ["OK"]
    o["status"] = ",".join(statuses)
    response = json.dumps(o, indent=True).encode("utf8") + b"\n"
    status = "200 OK"
    response_headers = [
        ("Content-type", "application/json"),
        ("Content-Length", str(len(response))),
    ]
    start_response(status, response_headers)
    return iter([response])


class Application(gunicorn.app.base.Application):
    def init(self, *_, **__):
        pass

    def load(self):
        return app


def main():
    Application().run()


if __name__ == "__main__":
    main()
