# pprof

## use the gpprof

please see examples, for cpu profiling.

## use the Heaptrack

```bash
sudo apt-get install heaptrack
sudo apt-get install heaptrack-gui

heaptrack target/release/your_binary

# or
heaptrack -a heaptrack.your_binary.[timestamp].gz
heaptrack_gui heaptrack.your_binary.[timestamp].gz
```

