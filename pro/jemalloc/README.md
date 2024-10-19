## jemalloc resolve OOM

Tips: Needs linux OS to run.

- [crates/jemalloc_pprof](https://crates.io/crates/jemalloc_pprof/0.1.0)
- [case 2](https://github.com/ai-chen2050/chronos/blob/poc_1/demos/test_vlc_net/src/main.rs)

```bash
# check pthread numbers
ps -T -p <pid>

# linux
sudo apt-get update
sudo apt install libjemalloc-dev
sudo apt-get install graphviz


cargo run --package jemalloc --bin jemalloc
curl localhost:3000/debug/pprof/heap > heap.pb.gz
pprof -pdf heap.pb.gz > output.pdf # linux, needs /lib/x86_64-linux-gnu/libc.so.6
pprof -http=:8080 heap.pb.gz # needs graph and browser env on linux

# mac 
brew install gperftools # include jeprof
jeprof --show_bytes --pdf <path_to_binary> ./profile.out > ./profile.pdf
```

```bash
export MALLOC_CONF=prof:true
```