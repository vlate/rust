# rust note

```rust
fn main() {
    println!("hello");
}
```

## ch05 结构体

```rust

// 实现一个计算矩形面积的功能
// 简单函数实现
fn main() {
    let width1 = 20;
    let height1 = 30;

    println!("area is {}", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// 通过元组方式

fn main() {
    let rec1 = (20, 30);
    println!(
        "area is {}",
        area(rec1)
    );
}
    // 元组为参数的函数
fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

// 结构体的方式

    // 用debug trait输出结构体
#[derive(Debug)]

struct Rec {
    width: u32,
    height: u32,
}

fn main() {

    // 实例化结构体
    let rec1 = Rec {
        width: 20,
        height: 30,
    };
    println!(
        "area is {}",
        area(&rec1)
    );
    println!("rec1 is {:#?}", &rec1);
    // 加入:?或:#?指示符，调用debug输出
}

    // 结构体为参数的函数
fn area(rec: &Rec) -> u32 {
    rec.width * rec.height
}




// 用dbg!宏输出，本测试有些问题
#[derive(Debug)]
struct rec {
    width: u32,
    height: u32,
}
fn main(){
    let rec1=rec{
        width:20,
        height:30,
    };
    dbg!(rec1);
}

```

```rust
// 斐波那契数列

fn main() {
    let a = 10;
    for i in 0..a {
        println!("fib is{}, {}", i + 1, fib(i))
    }
}

fn fib(a: i32) -> i32 {
    if a <= 1 {
        a
    } else {
        // 由递归实现
        fib(a - 1) + fib(a - 2)
    }
}


```
