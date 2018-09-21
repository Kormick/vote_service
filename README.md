# VoteService

Demo implementation of vote service using exonum framework.

Supported features:
- Adding/getting information of candidate
- Adding/getting information of voter
- Adding of vote (will be encrypted)
- Getting of compilated encrypted and decrypted vote results

## Install and run
Clone and build the project:

```sh
git clone https://github.com/Kormick/vote_service
cd vote_service
cargo build
```

Generate blockchain configuration:
Go to the build folder (i.e. vote_service/target/debug)
Generate configuration template:
```sh
mkdir example

./vote-service generate-template example/common.toml --validators-count 4 --author-public-key b05571f9af36c9e3a6c43a9da163c1cb8bc444338cdfc9ac8569690b552e7e25
```

Generate templates of nodes configuration:
```sh
./vote-service generate-config example/common.toml  example/pub_1.toml example/sec_1.toml --peer-address 127.0.0.1:6331

./vote-service generate-config example/common.toml  example/pub_2.toml example/sec_2.toml --peer-address 127.0.0.1:6332

./vote-service generate-config example/common.toml  example/pub_3.toml example/sec_3.toml --peer-address 127.0.0.1:6333

./vote-service generate-config example/common.toml  example/pub_4.toml example/sec_4.toml --peer-address 127.0.0.1:6334
```

Finalize generation of nodes configurations:
```sh
./vote-service finalize --public-api-address 0.0.0.0:8200 --private-api-address 0.0.0.0:8091 example/sec_1.toml example/node_1_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml

./vote-service finalize --public-api-address 0.0.0.0:8201 --private-api-address 0.0.0.0:8092 example/sec_2.toml example/node_2_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml

./vote-service finalize --public-api-address 0.0.0.0:8202 --private-api-address 0.0.0.0:8093 example/sec_3.toml example/node_3_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml

./vote-service finalize --public-api-address 0.0.0.0:8203 --private-api-address 0.0.0.0:8094 example/sec_4.toml example/node_4_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml
```

Run nodes:
```sh
./vote-service run --node-config example/node_1_cfg.toml --db-path example/db1 --public-api-address 0.0.0.0:8200

./vote-service run --node-config example/node_2_cfg.toml --db-path example/db2 --public-api-address 0.0.0.0:8201

./vote-service run --node-config example/node_3_cfg.toml --db-path example/db3 --public-api-address 0.0.0.0:8202

./vote-service run --node-config example/node_4_cfg.toml --db-path example/db4 --public-api-address 0.0.0.0:8203
```