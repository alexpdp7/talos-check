FROM registry.access.redhat.com/ubi9/ubi
COPY . /app
RUN python3 -m ensurepip
RUN python3 -m pip install /app[k8s,httpd]
CMD /usr/local/bin/talos-check-httpd --bind 0.0.0.0:8000
