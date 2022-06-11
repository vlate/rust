# rust note

```rust
fn main() {
    println!("hello");
}
```

## ch05 结构体

```rust
// 结构体

// 用debug trait输出结构体
#[derive(Debug)]

struct Rec {
    width: u32,
    height: u32,
}

fn main() {

    // 结构体参数
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

// 结构体
fn area(rec: &Rec) -> u32 {
    rec.width * rec.height
}

// 通过元组方式

fn main() {
    let rec1 = (20, 30);
    println!(
        "area is {}",
        area(rec1)
    );
}

fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

// 简单函数实现
fn main() {
    let width1 = 20;
    let height1 = 30;

    println!("area is {}", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
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
