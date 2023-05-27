FROM docker.io/library/rust:latest as builder
WORKDIR /usr/src/myapp
COPY manifest-builder .
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo install --path .

FROM registry.access.redhat.com/ubi9/ubi
COPY --from=builder /usr/local/cargo/bin/talos-check-manifest-builder /usr/local/bin/talos-check-manifest-builder
RUN mkdir /app
COPY poetry.lock pyproject.toml README.md /app
COPY talos_check /app/talos_check
RUN python3 -m ensurepip
RUN python3 -m pip install /app[k8s,httpd]
CMD /usr/local/bin/talos-check-httpd --bind 0.0.0.0:8000
