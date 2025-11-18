FROM debian:trixie-slim

RUN apt-get update && apt-get install -y curl tar libxml2 libssl3 openssl ca-certificates libgcc-s1

COPY hurl-7.0.0-x86_64-unknown-linux-gnu.tar.gz /tmp/
COPY hurly.hurl /hurly.hurl

CMD ["/bin/bash", "-c", "tar -xzf /tmp/hurl-7.0.0-x86_64-unknown-linux-gnu.tar.gz -C /usr/local/ --strip-components=1 && hurl --version && hurl /hurly.hurl"]