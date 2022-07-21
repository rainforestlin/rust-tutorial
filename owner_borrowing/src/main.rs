fn main() {
    // 我们已经见过字符串字面值 let s ="hello"，s 是被硬编码进程序里的字符串值（类型为 &str ）。字符串字面值是很方便的，但是它并不适用于所有场景。原因有二：
    // 字符串字面值是不可变的，因为被硬编码到程序代码中
    // 并非所有字符串的值都能在编写代码时得知
    // 例如，字符串是需要程序运行时，通过用户动态输入然后存储在内存中的，这种情况，字符串字面值就完全无用武之地。 为此，Rust 为我们提供动态字符串类型: String, 该类型被分配到堆上，因此可以动态伸缩，也就能存储在编译时大小未知的文本。
    let mut s = String::from("hello");
    s.push_str(", world");
    let s2 = s;
    // ^ value borrowed here after move
    // 把 s 的内容拷贝一份赋值给 s2，实际上，并不是这样。之前也提到了，对于基本类型（存储在栈上），Rust 会自动拷贝，但是 String 不是基本类型，而且是存储在堆上的，因此不能自动拷贝。
    // 实际上， String 类型是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量共同组成，其中堆指针是最重要的，它指向了真实存储字符串内容的堆内存，至于长度和容量
    // println!("{}", s);
    // 当变量离开作用域后，Rust 会自动调用 drop 函数并清理变量的堆内存。不过由于两个 String 变量指向了同一位置。这就有了一个问题：当 s1 和 s2 离开作用域，它们都会尝试释放相同的内存。
    // 这是一个叫做 二次释放（double free） 的错误，也是之前提到过的内存安全性 BUG 之一。两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。
    // 因此，Rust 这样解决问题：当 s1 赋予 s2 后，Rust 认为 s1 不再有效，因此也无需在 s1 离开作用域后 drop 任何东西，这就是把所有权从 s1 转移给了 s2，s1 在被赋予 s2 后就马上失效了。

    // 这段代码和之前的 String 有一个本质上的区别：在 String 的例子中 s 持有了通过String::from("hello") 创建的值的所有权，而这个例子中，x 只是引用了存储在二进制中的字符串 "hello, world"，并没有持有所有权。
    // 因此 let y = x 中，仅仅是对该引用进行了拷贝，此时 y 和 x 都引用了同一个字符串。
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}", x, y);

    // 克隆(深拷贝)
    // 首先，Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何自动的复制都不是深拷贝，可以被认为对运行时性能影响较小。
    // 如果我们确实需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的方法。
    // 如果代码性能无关紧要，例如初始化程序时，或者在某段时间只会执行一次时，你可以使用 clone 来简化编程。但是对于执行较为频繁的代码(热点路径)，使用 clone 会极大的降低程序性能
    let s3 = s2.clone();
    println!("{}||||||{}", s2, s3);

    // 拷贝(浅拷贝)
    // 浅拷贝只发生在栈上，因此性能很高，在日常编程中，浅拷贝无处不在。
    // 原因是像整型这样的基本类型在编译时是已知大小的，会被存储在栈上，所以拷贝其实际的值是快速的。这意味着没有理由在创建变量 y 后使 x 无效（x、y 都仍然有效）。
    // 换句话说，这里没有深浅拷贝的区别，因此这里调用 clone 并不会与通常的浅拷贝有什么不同
    let x = 5;
    let y = x; // 全等于let y = x.clone();
    println!("x = {}, y = {}", x, y);
    // Rust 有一个叫做 Copy 的特征，可以用在类似整型这样在栈中存储的类型。如果一个类型拥有 Copy 特征，一个旧的变量在被赋值给其他变量后仍然可用。
    // 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。如下是一些 Copy 的类型：
    // 所有整数类型，比如 u32。
    // 布尔类型，bool，它的值是 true 和 false。
    // 所有浮点数类型，比如 f64。
    // 字符类型，char。
    // 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
    // 不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意: 可变引用 &mut T 是不可以 Copy的

    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // println!("{}",s);                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x

    let s1 = gives_ownership(); // gives_ownership 将返回值
                                // 移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到
                                       // takes_and_gives_back 中,
                                       // println!("{}", s2); value borrowed here after move// 也将返回值移给 s3
    println!("{}", s3);

    //如果仅仅支持通过转移所有权的方式获取一个值，那会让程序变得复杂。 Rust 通过 借用(Borrowing) 这个概念来达成上述的目的，获取变量的引用，称之为借用(borrowing)。
    // 正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来，当使用完毕后，也必须要物归原主。

    // 不可变引用
    // 我们用 s1 的引用作为参数传递给 calculate_length 函数，而不是把 s1 的所有权转移给该函数
    // 先通过函数参数传入所有权，然后再通过函数返回来传出所有权，代码更加简洁
    // calculate_length 的参数 s 类型从 String 变为 &String
    // 这里，& 符号即是引用，它们允许你使用值，但是不获取所有权
    // 通过 &s1 语法，我们创建了一个指向 s1 的引用，但是并不拥有它。因为并不拥有这个值，当引用离开作用域后，其指向的值也不会被丢弃。
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 可变引用
    // 声明 s 是可变类型，其次创建一个可变的引用 &mut s 和接受可变引用参数 some_string: &mut String 的函数。
    // 可变引用同时只能存在一个
    // 不过可变引用并不是随心所欲、想用就用的，它有一个很大的限制： 同一作用域，特定数据只能有一个可变引用
    let mut s = String::from("hello");
    {
        // 会报错
        // 这种限制的好处就是使 Rust 在编译期就避免数据竞争，数据竞争可由以下行为造成：
        // 两个或更多的指针同时访问同一数据
        // 至少有一个指针被用来写入数据
        // 没有同步数据访问的机制
        // let r1 = &mut s;
        // let r2 = &mut s;
        // 可变引用与不可变引用不能同时存在
        // let r1 = &s; // 没问题
        // let r2 = &s; // 没问题
        // let r3 = &mut s; // 大问题
    }

    change(&mut s);
    println!("{}", s);
    {
        //引用的作用域 s 从创建开始，一直持续到它最后一次使用的地方，这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号 }
        // 在老版本的编译器中（Rust 1.31 前），将会报错，因为 r1 和 r2 的作用域在花括号 } 处结束，那么 r3 的借用就会触发 无法同时借用可变和不可变的规则。
        // 但是在新的编译器中，该代码将顺利通过，因为 引用作用域的结束位置从花括号变成最后一次使用的位置，因此 r1 借用和 r2 借用在 println! 后，就结束了，此时 r3 可以顺利借用到可变引用。

        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
        // 新编译器中，r1,r2作用域在这里结束

        let r3 = &mut s;
        println!("{}", r3);
    } //老编译器中，r1、r2、r3作用域在这里结束
      // 新编译器中，r3作用域在这里结束

    // 借用规则总结
    // 总的来说，借用规则如下：
    // 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
    // 引用必须总是有效的
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}
