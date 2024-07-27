#!/usr/bin/env bash

cargo build --release --bin cldb-server
cargo build --release --bin cldb-client

./target/release/cldb-server &> /dev/null &
server_id=$ID
(
  for ii in $(seq $1) 
  do
    ./target/release/cldb-client &
  done

  wait
)

pkill -P $$
