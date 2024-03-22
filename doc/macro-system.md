# Rust 四大类型宏

## 概念 

Rust 的宏体系：主要分为**声明式函数宏 Function Macro** 与**过程宏（Procedural Macro）**，其中过程宏 （proc_macro）又有**类函数的过程宏**（Function-like-procedural-macros）、**过程属性宏**（Attribute Macro）和**过程派生宏**（Derive Macro）。而属性宏（Attribute Macro）和派生宏（Derive Macro）是两个比较常见常用的类型。他们各自具有不同的特性和用途，可以根据需要选择合适的宏类型来实现特定的代码生成或元编程需求。

过程宏允许在编译时运行对 Rust 句法进行操作的代码，它可以在消费掉一些 Rust 句法输入的同时产生新的 Rust 句法输出。可以将过程宏想象成是从一个 AST 到另一个 AST 的函数映射。

过程宏有两种报告错误的方法。首先是 panic；第二个是发布 [`compile_error`](https://doc.rust-lang.org/std/macro.compile_error.html) 性质的宏调用。

## 函数宏或声明宏（Function Macro）

函数宏是一种宏，它接受输入并生成输出。它们可以像函数一样接受参数，并使用类似于宏的语法进行处理和转换。函数宏使用 `macro_rules!` 关键字定义，并使用 `!` 符号调用。

例如，在下面的示例中，我们定义了一个简单的函数宏 `hello_macro!`，它接受一个参数并生成一个打印语句：

````rust
   macro_rules! hello_macro {
       ($name:expr) => {
           println!("Hello, {}!", $name);
       };
   }

   fn main() {
       hello_macro!("World"); // 输出：Hello, World!
   }
````

## 过程宏（Procedural Macro）

过程宏是一种更强大和灵活的宏，它允许在编译时根据 Rust 代码的结构进行更复杂的代码生成和转换。过程宏是通过创建一个实现特定 trait 的自定义宏来定义的。这些过程宏可以用于生成代码、属性处理、代码转换和其他元编程任务。

另外，Rust 标准库中的 `proc-macro` 模块提供了用于编写自定义过程宏的相关类型和函数，例如 `TokenStream` 和 `TokenTree` 等。

### 类函数的过程宏（Function-like-procedural-macros）

类函数过程宏是使用宏调用运算符（`!`）调用的过程宏。

这种宏是由一个带有 `proc_macro`[属性](https://rustwiki.org/zh-CN/reference/attributes.html) 和 `(TokenStream) -> TokenStream` 签名的 [公有](https://rustwiki.org/zh-CN/reference/visibility-and-privacy.html)可见性[函数](https://rustwiki.org/zh-CN/reference/items/functions.html)定义。输入 [`TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html) 是由宏调用的定界符界定的内容，输出 [`TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html) 将替换整个宏调用。

例如，下面的宏定义忽略它的输入，并将函数 `answer` 输出到它的作用域。

```rust
#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

```

然后我们用它在一个二进制 crate 里打印 “42” 到标准输出。

```rust
extern crate proc_macro_examples;
use proc_macro_examples::make_answer;

make_answer!();

fn main() {
    println!("{}", answer());
}
```

### 派生宏（Derive Macro）

派生宏是一种特殊类型的宏，它用于根据用户自定义的类型自动生成代码。派生宏通常用于为结构体、枚举或 trait 实现自动生成常见的实现代码。

自定义派生宏由带有 `proc_macro_derive`属性和 `(TokenStream) -> TokenStream`签名的公有可见性函数定义。

派生宏使用 `#[derive(...)]` 语法，并放置在类型定义上方。它们可以自动为类型实现特定的 trait 或生成相关的代码。

例如，使用 `#[derive(Debug)]` 可以自动生成针对调试输出的实现，`#[derive(Clone, Copy)]` 可以自动生成克隆和复制的实现。

用户也可以通过编写自己的派生宏来自定义生成的代码，例如，实现自定义的序列化、反序列化逻辑或其他定制行为。

需要注意的是，属性宏和派生宏都是 Rust 中的过程宏（Procedural Macro）的一种。它们通过自定义宏来扩展或生成代码，提供了更大的灵活性和元编程能力，可以根据需要修改或生成 Rust 代码的结构和行为。

#### 派生宏附加其他属性

派生宏可以将额外的属性添加到它们所在的程序项的作用域中。这些属性被称为派生宏辅助属性。这些属性是[惰性的](https://rustwiki.org/zh-CN/reference/attributes.html#active-and-inert-attributes)，它们存在的唯一目的是将这些属性在使用现场获得的属性值反向输入到定义它们的派生宏中。也就是说所有该宏的宏应用都可以看到它们。

**关于活跃属性和惰性属性**：属性要么是活跃的，要么是惰性的。在属性处理过程中，活跃属性将自己从它们所在的对象上移除，而惰性属性依然保持原位置不变。

[`cfg`](https://rustwiki.org/zh-CN/reference/conditional-compilation.html#the-cfg-attribute) 和 [`cfg_attr`](https://rustwiki.org/zh-CN/reference/conditional-compilation.html#the-cfg_attr-attribute) 属性是活跃的。[`test`](https://rustwiki.org/zh-CN/reference/attributes/testing.html#the-test-attribute)属性在为测试所做的编译形式中是惰性的，在其他编译形式中是活跃的。[宏属性](https://rustwiki.org/zh-CN/reference/procedural-macros.html#attribute-macros)是活跃的。所有其他属性都是惰性的。

定义辅助属性的方法是在 `proc_macro_derive` 宏中放置一个 `attributes` 键，此键带有一个使用逗号分隔的标识符列表，这些标识符是辅助属性的名称。

例如，下面的派生宏定义了一个辅助属性 `helper`，但最终没有用它做任何事情。

```rust
#![crate_type="proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(HelperAttr, attributes(helper))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
```

然后在一个结构体上使用这个派生宏：
```rust
#[derive(HelperAttr)]
struct Struct {
    #[helper] field: ()
}
```

**派生宏示例一：自定义的派生宏**

下面是派生宏的一个示例。它没有对输入执行任何有用的操作，只是追加了一个函数 `answer`。

```rust
#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
```

然后使用这个派生宏：
```rust
extern crate proc_macro_examples;
use proc_macro_examples::AnswerFn;

#[derive(AnswerFn)]
struct Struct;

fn main() {
    assert_eq!(42, answer());
}
```

**派生宏示例二：派生系统内建的宏**

```rust
// 定义一个派生宏 HelloDebug，为结构体自动生成 Debug trait 的实现
#[derive(Debug)]
struct HelloDebug {
    name: String,
}

fn main() {
    let hello = HelloDebug {
        name: "World".to_string(),
    };

    println!("{:?}", hello); // 输出：HelloDebug { name: "World" }
}
```

在上述示例中，我们定义了一个名为 `HelloDebug` 的结构体，并为其应用了派生宏 `#[derive(Debug)]`。这将自动生成 `Debug` trait 的实现，使我们能够使用 `println!` 宏打印出结构体的调试信息。在 `main` 函数中，我们创建了一个 `HelloDebug` 实例，并通过 `println!` 打印出结构体的调试信息。

**派生宏示例三：派生第三方定义的宏**

```rust
// 定义一个派生宏 Builder，为结构体自动生成 builder 模式的代码
#[derive(Builder)]
struct Person {
    name: String,
    age: u32,
    address: String,
}

fn main() {
    let person = Person::new()
        .name("John")
        .age(30)
        .address("123 Street")
        .build();

    println!("{:?}", person);
}
```

在上述示例中，我们使用派生宏 `#[derive(Builder)]` 为结构体 `Person` 自动生成 builder 模式的代码。这使我们能够使用链式调用的方式创建 `Person` 实例，并在 `build` 方法中构建最终的对象。在 `main` 函数中，我们使用 builder 模式创建了一个 `Person` 实例，并打印出其信息。

这是一个更复杂的示例，展示了派生宏可以用于生成更多的代码，例如构建器模式、序列化和反序列化的代码等。

### 属性宏（Attribute Macro）

属性宏是一种基于属性的宏，用于修改、扩展或注解 Rust 代码。它们通常用于为函数、结构体、枚举、模块等添加元数据或自定义行为。

属性宏使用 `#[...]` 语法，可以应用于各种语法结构，例如函数、结构体等。它们可以接收属性中的参数，并根据需要对代码进行转换、生成额外的代码或执行其他逻辑。当出现  `#![...]` 时候，表示该属性应用于当前模版。

属性宏由带有 `proc_macro_attribute`属性和 `(TokenStream, TokenStream) -> TokenStream`签名的公有可见性函数定义。签名中的第一个 [`TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html) 是属性名称后面的定界 token树。如果该属性作为裸属性(bare attribute)给出，则第一个 `TokenStream` 值为空。第二个 `TokenStream` 是[程序项](https://rustwiki.org/zh-CN/reference/items.html)的其余部分，包括该程序项的其他[属性](https://rustwiki.org/zh-CN/reference/attributes.html)。

## 示例

当谈到属性宏和派生宏时，以下是在 Rust 中的代码示例：

**属性宏示例一：**

例如，下面这个属性宏接受输入流并按原样返回，实际上对属性并无操作。

```rust
#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
```

下面示例显示了属性宏看到的字符串化的 [`TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html)。输出将显示在编译时的编译器输出窗口中。（具体格式是以 "out:"为前缀的）输出内容也都在后面每个示例函数后面的注释中给出了。

```rust
// my-macro/src/lib.rs
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

```

下面示例显示了属性宏看到的字符串化的 [`TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html)。输出将显示在编译时的编译器输出窗口中。（具体格式是以 "out:"为前缀的）输出内容也都在后面每个示例函数后面的注释中给出了。
```rust
// src/lib.rs
extern crate my_macro;

use my_macro::show_streams;

// 示例: 基础函数
#[show_streams]
fn invoke1() {}
// out: attr: ""
// out: item: "fn invoke1() { }"

// 示例: 带输入参数的属性
#[show_streams(bar)]
fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"

// 示例: 输入参数中有多个 token 的
#[show_streams(multiple => tokens)]
fn invoke3() {}
// out: attr: "multiple => tokens"
// out: item: "fn invoke3() {}"

// 示例:
#[show_streams { delimiters }]
fn invoke4() {}
// out: attr: "delimiters"
// out: item: "fn invoke4() {}"

```

**属性宏示例二：**

```rust
// 定义一个属性宏 hello_attribute，将传入的字符串包装在 println! 宏中
#[proc_macro_attribute]
pub fn hello_attribute(attr: TokenStream, input1: TokenStream) -> TokenStream {
    let output = format!("println!(\"Hello, {}\");", attr.to_string());
    println!("{}", output);
    input1
}

// 使用 hello_attribute 宏为函数添加注解
#[hello_attribute("World")]
fn greet() {
    // 生成的代码将打印 "Hello, World"
}
```

在上述示例中，我们定义了一个名为 `hello_attribute` 的属性宏，它将传入的字符串包装在 `println!` 宏中。然后，我们使用 `hello_attribute` 宏对 `greet` 函数进行注解，在运行时将打印 "Hello, World"。

当涉及到更复杂的示例时，属性宏和派生宏可以实现更多的功能和代码转换。以下是更复杂一点的示例：

**属性宏示例三：**

```rust
// 定义一个属性宏 repeat，将函数体重复执行指定次数
#[proc_macro_attribute]
pub fn repeat(attr: TokenStream, input: TokenStream) -> TokenStream {
    let repeat_count = attr.to_string().parse::<u32>().unwrap();
    let input_fn = input.to_string();

    let mut output = TokenStream::new();
    if let Some(index) = input_fn.find('(') {
        for i in 0..repeat_count {
            let ret = format!("{}{}{}", &input_fn[..index], i+1, &input_fn[index..]);
            println!("fn: \"{}\"", ret);
            output.extend(ret.parse::<TokenStream>().unwrap());
        }
    }

    output
}

// 使用 repeat 宏为函数添加注解，使函数体重复执行 3 次
#[repeat(3)]
fn greet() {
    println!("Hello, world!");
}
```

在上述示例中，我们定义了一个名为 `repeat` 的属性宏。该宏接受一个参数 `attr`，表示重复执行的次数，并将函数体 `input` 重复执行指定次数。在 `greet` 函数上方使用 `#[repeat(3)]` 注解，将函数体定义了 3 个类似的函数。