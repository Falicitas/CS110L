# CS110L

==BiliBili 的 Rust 值得一看，补充知识==

## Rust

```rust
//结构体构造实例：
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
//通过 Key:Value 的方式进行初始化。其中 Value 的标识符可以与 Key 相同，如 child:child,

//引用和借用：
//Rust 中借用指的是创建引用变量的过程
//非可变借用的 Syntax
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
//传参时指明是借用而非所有权的转移。函数参数的类型是引用
//可变引用 Syntax
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
//多读无写和一写无读在 Rust 中的体现：
//将引用在代码块中的生命周期（非严格说法，指声明到移除所有权的区间）看做一个两闭的区间
//一读无写：可变引用的之间的区间严格不交
//多读不写：不可变引用之间的区间可以相交
//不可变和可变引用的区间严格不交
//作为引用的源头，其对于所有可变引用必须是真包含，非可变引用非严格包含
let mut s = String::from("hello");
let r1 = &mut s;
println!("{} {}", r1, s);
//Error

let mut s = String::from("hello");
let r1 = &mut s;
println!("{}", r1);
println!("{}", s);
//Ok

//enum
//enum的某个实例仅含有enum中的一个值。enum中不同值可以为不同类型
enum Message {
    Quit, //无类型，不包含任何值
    Move {x : i32, y : i32}, //包含匿名结构体 
    Write(String), //包含一个字符串
    ChangeColor(u32, u32, u32) //包含三元组
}
//实例
let x = Message::Write(String::from("Hello"));
let y = Message::Move { x: 1, y: 2 };
//! 待更结构体
//Rust enum 特点之一，可以为 enum 实现方法
impl Message{
    fn call(&self){
	//...
    }
}

//Option<T>
//Option<T>的类型推断
let x = Some(1);//i32
let y = Some("String");//&str
let z : Option<i32> = None;//需要显式声明类型

//match模式匹配
//enum中的某个类型包含的值需要都列出来，match Message{.. ChangeColor(x,y,z) => ...}
//match需要枚举完，但没有特殊处理归为一类的其他情况可以_代替
match Message {
    Message::Quit => ...,
    Message::Move { x, y } => {}//小细节：不需要加逗号
     _ => (),
}

//if let （match）语法糖
//相对于 match，仅匹配一种值
//可以这么写（Option为例）
if let Some(2) = v{
    ...
}else{
    println!("Other");
}

//Prelude
//Prelude 成为 Rust 的预导入模块，不需要导入即可使用
//常见的 Prelude（持续更新）
Option<T>
Some(T)
None

use std::env;
Vec<String> = args: Vec<String> = env::args().collect(); //获取 bash 参数。args[1] 是第一个参数

// ?语法糖：当函数返回值是 Option<Some(), None> 或者 Result<T, E>，可以 exp? 将 exp 的 None 或 E 返回。

#[derive(X, Y, Z)] //继承虚类XYZ。虚类也称作 trait（目前的理解）

impl fmt::Display for AccessMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//方法实例化：self是AccessMode 的实例，f 是流。实现了对 AccessMode 的{}模式匹配输出
        match self {
            AccessMode::Read => write!(f, "{}", "read"),
            AccessMode::Write => write!(f, "{}", "write"),
            AccessMode::ReadWrite => write!(f, "{}", "read/write"),
        }
    }
}

let s = data.to_string(); //to_string() 它能用于任何实现了 Display trait 的类型，字符串字面值也实现了它。

"foo" //字面量的类型是 &'static str

let q.ok()//将q : Result<T> 转为 Option<T>
for entry in fs::read_dir(dir).ok()? {}
//（read_dir(dir)返回的是Result）不过在迭代过程中，Err将返回Option，而给entry的值依然是Result。原因暂时未知

//需要返回 Option 或者 Result，尽量选前者，比较好把握

|参数1, 参数2, ...| -> 返回值类型 {
    // 函数体
}//闭包，Rust的lambda表达式。返回值类型不是必须，会根据返回值智能判断

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
//Rust中使用spawn创建新线程
//线程内部是函数体，通常用无名函数替代
//子进程仅执行spawn内的代码
handle.join().unwrap();//使得父进程等子进程结束后再终止

//std::time::Duration提供了静态时间函数
Duration::from_secs(1)//1s
Duration::from_millis(1)//1ms
Duration::from_micros(1)//1us
Duration::from_nanos(1)//1ns
//同时Duration的对象可以返回以 秒 和 纳秒 的二元组形式
let five_seconds = Duration::new(5, 0);
let five_seconds_and_five_nanos = five_seconds + Duration::new(0, 5);

assert_eq!(five_seconds_and_five_nanos.as_secs(), 5);//秒与5比较
assert_eq!(five_seconds_and_five_nanos.subsec_nanos(), 5);//纳秒与5比较

move//关键字，将匿名函数 A 中调用的函数外的值的所有权转至函数 A 内。
move |...|{
    ...
}//用法

//Rust 中一个实现消息传递并发的主要工具是通道（channel），通道有两部分组成，一个发送者（transmitter）和一个接收者（receiver）。
//std::sync::mpsc 包含了消息传递的方法：
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

## 系统指令相关函数

```cpp
//exec 是六个以exec为前缀的函数构成的一个函数族
//以 execl 为例：为进程重载0-3G的用户空间，同时执行命令。可以搭配 fork()，在子进程中重载该进程（即将进程转为执行指令，并为其分配空间）
int execl("绝对路径", “标识符”,  “需要的参数”（需要多少传入多少） ,NULL);
//举例
execl("/usr/bin/firefox", "firefox", "www.baidu.com", NULL);
//打开路径的文件，执行指令 firefox，指令参数为百度地址，那么进程将用 firefox 打开百度网页。
//最后 NULL 结尾

//ptrace 用于跟踪进程的信息。详细在下文
//https://zhuanlan.zhihu.com/p/438534744
//这篇第一个实例某行调用ptrace的参数 ®s，猜想是错的，应为 regs
```

