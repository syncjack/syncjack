FROM rustlang/rust:nightly-slim

COPY --from=kcov/kcov:latest /lib/x86_64-linux-gnu/*.so* /lib/x86_64-linux-gnu/
COPY --from=kcov/kcov:latest /usr/lib/x86_64-linux-gnu/*.so* /usr/lib/x86_64-linux-gnu/
COPY --from=kcov/kcov:latest /usr/local/bin/kcov* /usr/local/bin/

RUN cargo install cargo-kcov cargo-watch

COPY entrypoint.sh /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/bin/bash", "/usr/local/bin/entrypoint.sh"]