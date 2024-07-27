# Mr wolf 

> What's the time...? [Subbit](https://subbit.xyz) time

This repo is a toy app that integrates with a subbit payment channel. 

It's main aims are to 

- demonstrate what _featherweight_ means
- demonstrate what an application built over subbit means

## The name? 

[https://en.wikipedia.org/wiki/What%27s_the_time,_Mr_Wolf%3F](https://en.wikipedia.org/wiki/What%27s_the_time,_Mr_Wolf%3F)

## Setup

This project uses [flakes](https://nixos.wiki/wiki/Flakes), 
although you can surely roll-your-own with no trouble.
Its a rust repo. `cargo build`

## Run

There is a bash run script which runs a server and a number of clients.
```bash
./run.sh 10 # 10 clients
```
## Benchmarks 

It hits ~13,000 "txs" a second on underwhelming hardware (intel gen8 i7).
This number is already a bit meaningless since the clients and server are sitting on the same machine.

More realistic scenarios will be conducted in due course. 
