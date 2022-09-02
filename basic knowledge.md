# CS110L

## Rust

```rust
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
    thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
//Rust中使用spawn创建新线程
//线程内部是函数体，通常用无名函数替代
//子进程仅执行spawn内的代码

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

