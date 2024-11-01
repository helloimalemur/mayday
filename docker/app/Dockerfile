FROM rust:1.79

WORKDIR /mayday
COPY . .
RUN ls
RUN cargo install --path crates/server
RUN rm -rf /mayday/docker
CMD ["mayday_server"]
