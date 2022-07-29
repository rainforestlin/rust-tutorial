use std::collections::btree_map::Iter;

fn main() {
    // 在 &&Rust 中，模式匹配最常用的就是 &match 和 if let
    {
        // match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
        // match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
        // X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可
        // match 跟其golang中的 switch 非常像，_ 类似于 switch 中的 default
        // 何为模式匹配：将模式与 target 进行匹配，即为模式匹配，而模式匹配不仅仅局限于 match
        let direct = Direction::East;
        match direct {
            Direction::East => {
                println!("This is East")
            }
            Direction::North | Direction::South => {
                println!("这是南北");
            }
            // 类似于default
            _ => {
                println!("West!!")
            }
        }
    }
    // 当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match
    {
        // 只想要对 Some(3) 模式进行匹配, 不想处理任何其他 Some<u8> 值或 None 值。
        // 但是为了满足 match 表达式（穷尽性）的要求，写代码时必须在处理完这唯一的成员后加上 _ => ()，
        // 这样会增加不少无用的代码。
        let v = Some(3u8);
        match v {
            Some(3) => println!("three"),
            _ => (),
        }
        if let Some(3) = v {
            println!("three");
        }
    }
    // matches!宏
    // Rust 标准库中提供了一个非常实用的宏：matches!，
    // 它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 true or false
    {
        let v: Vec<MyEnum> = vec![MyEnum::Foo, MyEnum::Bar];
        // 现在如果想对 v 进行过滤，只保留类型是 MyEnum::Foo 的元素
        // binary operation `==` cannot be applied to type `&&MyEnum`
        // v.iter().filter(|x| x == MyEnum::Bar);
        let res = v.iter().filter(|x| matches!(x, MyEnum::Bar));
        for _ in res {
            println!("Bar")
        }
        // res的所有权已经转移到了上面的for中，后续已经moved
    }
    // 变量覆盖
    {
        // 可以看出在 if let 中，= 右边 Some(i32) 类型的 age 被左边 i32 类型的新 age 覆盖了，
        // 该覆盖一直持续到 if let 语句块的结束。因此第三个 println! 输出的 age 依然是 Some(i32) 类型。
        let age = Some(27);
        println!("if let 前age={:?}", age);
        if let Some(age) = age {
            println!("if let 中 age={:?}", age)
        }
        println!("if let 后 age={:?}", age);
        // match 中的变量覆盖其实不是那么的容易看出
        match age {
            Some(age) => println!("匹配出来的age是{}", age),
            _ => (),
        }
        println!("在匹配后，age是{:?}", age);
    }
    // 模式是 Rust 中的特殊语法，它用来匹配类型中的结构和数据，它往往和 match 表达式联用，以实现强大的模式匹配能力。模式一般由以下内容组合而成：

    // 字面值
    // 解构的数组、枚举、结构体或者元组
    // 变量
    // 通配符
    // 占位符
    {
        // match 的每个分支就是一个模式，因为 match 匹配是穷尽式的，因此我们往往需要一个特殊的模式 _，来匹配剩余的所有情况
        // if let 往往用于匹配一个模式，而忽略剩下的所有模式的场景
        // 一个与 if let 类似的结构是 while let 条件循环，它允许只要模式匹配就一直进行 while 循环
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }

        let v = vec!['a', 'b', 'c'];
        // 这里使用 enumerate 方法产生一个迭代器，该迭代器每次迭代会返回一个 (索引，值) 形式的元组，然后用 (index,value) 来匹配
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
        // 下面也是一种模式匹配：
        // let PATTERN = EXPRESSION;
        // x 也是一种模式绑定，代表将匹配的值绑定到变量 x 上。因此，在 Rust 中,变量名也是一种模式
        let x = 5;
    }
}

enum Direction {
    East,
    West,
    North,
    South,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum MyEnum {
    Foo,
    Bar,
}

// value_in_cents 函数根据匹配到的硬币，返回对应的美分数值。
// match 后紧跟着的是一个表达式，跟 if 很像，但是 if 后的表达式
// 必须是一个布尔值，而 match 后的表达式返回值可以是任意类型，只要能跟后面的分支中的模式匹配起来即可
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
