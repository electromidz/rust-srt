#!/bin/bash
cargo b --release
sudo setcap net_cap_admin=eip ./target/release/rust-srt
./target/release/rust-srt &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
wait $pid

