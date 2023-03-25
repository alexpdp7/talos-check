FROM registry.access.redhat.com/ubi9/ubi
COPY . /app
RUN python3 -m ensurepip
RUN python3 -m pip install /app
CMD /usr/local/bin/talos-check-http
