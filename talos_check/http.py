import http
import http.server
import json
import socketserver

import talos_check


class HTTPRequestHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        obj = {
            "available_kubernetes_version": talos_check.get_available_kubernetes_version(),
            "cluster_kubernetes_version": talos_check.get_cluster_kubernetes_version(),
            "available_talos_version": talos_check.get_available_talos_version(),
            "cluster_talos_versions": talos_check.get_cluster_talos_versions(),
        }
        obj["needs_kubernetes_update"] = obj["available_kubernetes_version"] != obj["cluster_kubernetes_version"]
        obj["outdated_talos_versions"] = [ctv for ctv in obj["cluster_talos_versions"] if ctv != obj["available_talos_version"]]
        obj["needs_talos_update"] = len(obj["outdated_talos_versions"]) > 0

        statuses = []
        if obj["needs_kubernetes_update"]:
            statuses.append(f"NEEDS-KUBE-UPDATE-TO-{obj['available_kubernetes_version']}-FROM-{obj['cluster_kubernetes_version']}")
        if obj["needs_talos_update"]:
            outdated_talos_versions = ",".join(set(obj["outdated_talos_versions"]))
            statuses.append(f"NEEDS-TALOS-UPDATE-TO-{obj['available_talos_version']}-FROM-{outdated_talos_versions}")
        if not statuses:
            statuses = "UPDATED"
        obj["status"] = ",".join(statuses)
        response = json.dumps(obj, indent=True).encode("utf8")
        self.send_response(http.HTTPStatus.OK)
        self.send_header("Content-type", "application/json")
        self.send_header("Content-length", len(response))
        self.end_headers()
        self.wfile.write(response)


def main():
    with socketserver.TCPServer(("", 8000), HTTPRequestHandler) as httpd:
        print("serving at 8000")
        httpd.serve_forever()


if __name__ == "__main__":
    main()
