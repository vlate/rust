use rand::Rng;
// 通过crate引入rand库；rand不在标准库中，需要在cargo.toml中引入相关依赖。
// 在cargo.toml引入后首次运行cargo build会从crates.io下载相关依赖
use std::cmp::Ordering;
// ordering是一个成员为less、greater、equal的枚举；
use std::io;
//为用户输入引入来自标准库std的io库

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // 调用rand::thread_rug函数提供随机数生成器；gen_range方法定义随机数范围，包含下限不包含上限（1..11）与(1..=100)相同
    loop {
        // 使用loop创建一个无限循环，
        println!("Please input your guess.");

        let mut guess = String::new();
        // new为string字符串类型的关联函数

        io::stdin()
            // 从io库中调用stdin函数；如果开头没有使用use std::io引入io库，则可以写成std::io::stdin来使用
            .read_line(&mut guess)
            // 调用read_line获取用户输入；&表述应用相关变量；read_line返回枚举值ok或err
            .expect("Failed to read line");
            // 当返回值为err时程序崩溃，并返回信息"Failed to read line"
        //
        let guess: u32 = match guess.trim().parse() 
        // .expect("Please type a number!")
        // parse返回resule类型,并传给expect
        {
            // rust默认为i32有符号数字，此处增加类型信息使rust推断secret_number为u32无符号数字类型
            //trim为string的方法用于去除字符串开头和结尾的空白字符
            //read_line返回值在windows上会增加一个回车和换行符
            //parse方法将字符串解析成数字，数字类型由let guess:u32指定
            Ok(num) => num,
            Err(_) => continue,
            // 将expect调换成match语句，将遇到错误崩溃变为处理错误
            // "_"为通配符，即匹配所有err值，不管何种信息都执行此分支
        };

        println!("You guessed: {}", guess);
        // {}为占位符

        match guess.cmp(&secret_number)  {
            // 比较guess与secret numver，cmp返回一个ordering枚举成员
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
                // 用break语句在猜对时退出
            }
        }
    }
}