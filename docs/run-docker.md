
### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.


This command will firstly compile your code, and then start a local development network. You can
also replace the default command
(`cargo build --release && ./target/release/dao-entrance-node --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run node without re-compiling
docker run asyoume/dao-entrance-node:dev.2023-02-18-17_39 dao-entrance-node --dev --ws-external

# Purge the local dev chain
docker run asyoume/dao-entrance-node:dev.2023-02-18-17_39 dao-entrance-node purge-chain --dev

```