## 如何在生产环境排查 Rust 内存占用过高问题

Linux to run

```bash
# mac
brew install jemalloc
jemalloc-config --version

brew install google-perftools

# linux
sudo apt update
sudo apt install libjemalloc-dev
```

- https://rustmagazine.github.io/rust_magazine_2021/chapter_5/rust-memory-troubleshootting.html

demo 已经是非常简化的测试用例了，主要做如下的说明：

set_prof_active 和 dump_profile 都是通过 jemalloc-ctl 来调用 jemalloc 提供的 mallctl 函数，通过给相应的 key 设置 value 即可，比如这里就是给 prof.active 设置布尔值，给 profile.dump 设置 dump 的文件路径。
编译完成之后，直接运行程序是不行的，需要设置好环境变量（开启内存 profile 功能）：

```bash
export MALLOC_CONF=prof:true
```

然后再运行程序，就会输出一份 memory profile 文件，demo 中文件名字已经写死 —— profile.out，这个是一份文本文件，不利于直接观察（没有直观的 symbol）。

通过 jeprof 等工具，可以直接将其转化成可视化的图形：


jeprof --show_bytes --pdf <path_to_binary> ./profile.out > ./profile.pdf
这样就可以将其可视化，从下图中，我们可以清晰地看到所有的内存来源：