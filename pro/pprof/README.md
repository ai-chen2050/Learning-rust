# pprof

## use the gpprof

please see examples, for cpu profiling.

- [crates/pprof](https://crates.io/crates/pprof)

```bash
~/go/bin/pprof -svg profile.pb  > profile.svg
go tool pprof -call_tree -lines -output profile.png -png <exec_path>  profile.pb
```

## use the linux Heaptrack

```bash
sudo apt-get install heaptrack
sudo apt-get install heaptrack-gui

heaptrack target/release/your_binary

# or
heaptrack -a heaptrack.your_binary.[timestamp].gz
heaptrack_gui heaptrack.your_binary.[timestamp].gz
```

