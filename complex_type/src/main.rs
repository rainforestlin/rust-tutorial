fn main() {
    //    字符串
    {
        // 字符串字面量是切片
        let my_name = "Julian";
        // expected String, found &str
        // greet(my_name)
        let name = String::from("Julian");
        greet(name);
    }
    // 切片
    {
        let s = String::from("hello World");
        let hello = &s[..5];
        let world = &s[6..11];
        println!("{}", hello);
        println!("{}", world)
    }
    // String和&str的转换
    // 实际上这种灵活用法是因为 deref 隐式强制转换
    // Rust 不允许去索引字符串:原因有二，
    // 一是字符串的底层数据存储格式是[u8],一个字节组，对于不同的字符来讲，占用的字节不同，比如中文在UTF8中占用3个字节，单取一个没有意义
    // 二是因为索引操作，我们总是期望它的性能表现是 O(1)，然而对于 String 类型来说，无法保证这一点，因为 Rust 可能需要从 0 开始去遍历字符串来定位合法的字符
    {
        let s = String::from("hello World");
        let s = "hello World".to_string();
        say_hello(&s);
        say_hello(&s[..]);
        say_hello(s.as_str());
    }
    // 字符串切片
    // 字符串切片是非常危险的操作，因为切片的索引是通过字节来进行，
    // 但是字符串又是 UTF-8 编码，因此无法保证索引的字节刚好落在字符的边界上
    {
        let hello = "中国人";
        // panicked at 'byte index 2 is not a char boundary; it is inside '中' (bytes 0..3) of `中国人`',
        // let s = &hello[0..2];
    }
    // 操作字符串
    {
        let mut s = String::from("hello");
        s.push_str(" world");
        s.insert_str(5, "!!!");
    }
    // 元组是由多种类型组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组中元素的顺序也是固定的。
    {
        // 定义一个元组
        let this_is_a_tuple: (i32, f64, u8) = (10000, 23.45, 127);
        // 使用模式匹配解构元组
        let (x, y, z) = this_is_a_tuple;
        println!("x={},y={},z={}", x, y, z);
        // 使用.来访问元组
        // 模式匹配可以让我们一次性把元组中的值全部或者部分获取出来，如果只想要访问某个特定元素，那模式匹配就略显繁琐，对此，Rust 提供了 . 的访问方式
        let d = this_is_a_tuple.0;
        println!("d={}", d);
        // 元组在函数返回值场景很常用，例如下面的代码，可以使用元组返回多个值，类似golang的返回模式
        let str0 = String::from("helloworld");
        // 注意，此时的“helloworld”的使用权从str0变到了str1
        let (str1, size) = return_multiple_value(str0);
        println!("string str1={str1},size={size}");
    }
    // 结构体
    // 通过关键字 struct 定义
    // 一个清晰明确的结构体 名称
    // 几个有名字的结构体 字段
    {
        // 初始化实例时，每个字段都需要进行初始化
        // 初始化时的字段顺序不需要和结构体定义时的顺序一致
        let user1 = User {
            email: String::from("julianlee107@163.com"),
            active: true,
            sign_in_count: 10,
            username: String::from("julianlee"),
        };
        // 通过 . 操作符即可访问结构体实例内部的字段值，也可以修改它们
        println!("{}", user1.active);
        // 必须要将结构体实例声明为可变的，才能修改其中的字段，Rust 不支持将某个结构体某个字段标记为可变。
        // 因为 user2 仅仅在 email 上与 user1 不同，因此我们只需要对 email 进行赋值，剩下的通过结构体更新语法 ..user1 即可完成。
        // .. 语法表明凡是我们没有显示声明的字段，全部从 user1 中自动获取。需要注意的是 ..user1 必须在结构体的尾部使用
        // 结构体更新语法跟赋值语句 = 非常相像，因此在上面代码中，user1 的部分字段所有权被转移到 user2 中：username 字段发生了所有权转移，
        // 作为结果，user1 无法再被使用。
        let mut user2 = User {
            email: String::from("_"),
            ..user1
        };
        user2.active = false;
        // 实现了 Copy 特征的类型无需所有权转移，可以直接在赋值时进行 数据拷贝，其中 bool 和 u64 类型就实现了 Copy 特征，因此 active 和 sign_in_count 字段在赋值给 user2 时，仅仅发生了拷贝，而不是所有权转移。
        // 值得注意的是：username 所有权被转移给了 user2，导致了 user1 无法再被使用，但是并不代表 user1 内部的其它字段不能被继续使用
        println!("{}", user2.email);
        // borrow of moved value: `user1.username`
        // move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait
        // println!("{}", user1.username);
        // struct内部解构由ptr,size,capcity三个组成
        // user分别拥有底层4个 [u8] 数组的所有权(String 类型的底层也是 [u8] 数组)，通过 ptr 指针指向底层数组的内存地址，这里可以把 ptr 指针理解为 Rust 中的引用类型
        // 侧面印证了：把结构体中具有所有权的字段转移出去后，将无法再访问该字段，但是可以正常访问其它的字段
        // 之前的 User 结构体的定义中，有一处细节：我们使用了自身拥有所有权的 String 类型而不是基于引用的 &str 字符串切片类型。这是一个有意而为之的选择：因为想要这个结构体拥有它所有的数据，而不是从其它地方借用数据。
        // 也可以让 User 结构体从其它对象借用数据，不过这么做，就需要引入生命周期(lifetimes)这个新概念（也是一个复杂的概念），简而言之，生命周期能确保结构体的作用范围要比它所借用的数据的作用范围要小
    }
}

fn greet(name: String) {
    println!("hello {}", name)
}

fn say_hello(s: &str) {
    println!("{}", s);
}

fn return_multiple_value(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// 当函数参数和结构体字段同名时，可以直接使用缩略的方式进行初始化，跟 TypeScript 中一模一样。
fn build_user_like_typescript(
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
) -> User {
    return User {
        active,
        username,
        email,
        sign_in_count,
    };
}
fn build_user_in_a_dum_way(
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
) -> User {
    User {
        active: active,
        username: username,
        sign_in_count: sign_in_count,
        email: email,
    }
}
