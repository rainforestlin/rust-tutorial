use std::net::TcpStream;

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
    // 枚举
    // 枚举类型是一个类型，它会包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例
    {
        let heart = PokerSuit::Hearts;
        let diamond = PokerSuit::Diamonds;
        print_suit(heart);
        let card1 = PokerCard::Clubs(String::from("A"));
        let card2 = PokerCard::Spades(String::from("10"));
    }
    // Option 枚举用于处理空值
    // 在其它编程语言中，往往都有一个 null 关键字，该关键字用于表明一个变量当前的值为空（不是零值，例如整型的零值是 0），
    // 也就是不存在值。当你对这些 null 进行操作时，例如调用一个方法，就会直接抛出null 异常，导致程序的崩溃，因此我们在
    // 编程时需要格外的小心去处理这些 null 空值。
    // Rust 吸取了众多教训，决定抛弃 null，而改为使用 Option 枚举变量来表述这种结果。
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // T 是泛型参数，Some(T)表示该枚举成员的数据类型是 T，换句话说，Some 可以包含任何类型的数据。
    {
        let some_num = Some(100);
        let some_string = Some("dont give up");
        // 使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。
        // 当有一个 Some 值时，我们就知道存在一个值，而这个值保存在 Some 中。当有个 None 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值
        let absent_num: Option<i32> = None;

        let x: i8 = 5;
        let y: Option<i8> = Some(5);
        // cannot add `Option<i8>` to `i8`
        // the trait `Add<Option<i8>>` is not implemented for `i8`
        // the following other types implement trait `Add<Rhs>`
        // 错误信息意味着 Rust 不知道该如何将 Option<i8> 与 i8 相加，因为它们的类型不同。
        // 当在 Rust 中拥有一个像 i8 这样类型的值时，编译器确保它总是有一个有效的值，
        // 我们可以放心使用而无需做空值检查。只有当使用 Option<i8>（或者任何用到的类型）的时候才需要担心可能没有值，
        // 而编译器会确保我们在使用值之前处理了为空的情况。
        // 不再担心会错误的使用一个空值，会让你对代码更加有信心。为了拥有一个可能为空的值，你必须要显式的将其放入对应类型的 Option<T> 中。
        // 接着，当使用这个值时，必须明确的处理值为空的情况。只要一个值不是 Option<T> 类型，你就 可以 安全的认定它的值不为空。
        // 这是 Rust 的一个经过深思熟虑的设计决策，来限制空值的泛滥以增加 Rust 代码的安全性。
        // 总的来说，为了使用 Option<T> 值，需要编写处理每个成员的代码。你想要一些代码只当拥有 Some(T) 值时运行，允许这些代码使用其中的 T。
        // 也希望一些代码在值为 None 时运行，这些代码并没有一个可用的 T 值。match 表达式就是这么一个处理枚举的控制流结构：
        // 它会根据枚举的成员运行不同的代码，这些代码可以使用匹配到的值中的数据。
        // let sum = x + y;
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("{:?}", six);
        println!("{:?}", none)
    }
    // 数组
    // 在 Rust 中，最常用的数组有两种，
    // 第一种是速度很快但是长度固定的 array，第二种是可动态增长的但是有性能损耗的 Vector
    // 一般可以称array为数组，Vector为动态数组
    // 这两个数组的关系跟 &str 与 String 的关系很像，前者是长度固定的字符串切片，后者是可动态增长的字符串。
    // 其实，在 Rust 中无论是 String 还是 Vector，它们都是 Rust 的高级类型：集合类型，和Java有点像
    // 数组是 Rust 的基本类型，是固定长度的，Rust 中的动态数组 Vector 类似于golang的切片
    {
        // 定义一个数组
        // 由于它的元素类型大小固定，且长度也是固定，因此数组 array 是存储在栈上，性能也会非常优秀。
        // 与此对应，动态数组 Vector 是存储在堆上，因此长度可以动态改变。
        // 数组的元素类型要统一，长度要固定
        let _a: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
        // 声明一个某个值重复N次的数组 [T;n] [初始值;长度]
        let _b = [3; 5];
        // 数组切片
        // 下面的数组切片 slice 的类型是&[i32]，与之对比，数组的类型是[i32;7]
        // 切片的长度可以与数组不同，并不是固定的，而是取决于你使用时指定的起始和结束位置
        // 创建切片的代价非常小，因为切片只是针对底层数组的一个引用
        // 切片类型[T]拥有不固定的大小，而切片引用类型&[T]则具有固定的大小，因为 Rust 很多时候都需要固定大小数据类型，因此&[T]更有用,&str字符串切片也同理
        let mut _slice = &_a[1..3];
        println!("{}", _slice.len())
        // 数组类型容易跟数组切片混淆，[T;n]描述了一个数组的类型，而[T]描述了切片的类型， 因为切片是运行期的数据结构，它的长度无法在编译期得知，因此不能用[T;n]的形式去描述
        // [u8; 3]和[u8; 4]是不同的类型，数组的长度也是类型的一部分
        // 在实际开发中，使用最多的是数组切片[T]，我们往往通过引用的方式去使用&[T]，因为后者有固定的类型大小
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
// Rust 默认不会为我们实现 Debug，为了实现，有两种方式可以选择：
// 手动实现
// 使用 derive 派生实现
// 后者简单的多，但是也有限制
// 还有一个简单的输出 debug 信息的方法，那就是使用 dbg! 宏，它会拿走表达式的所有权，
// 然后打印出相应的文件名、行号等 debug 信息，当然还有我们需要的表达式的求值结果。除此之外，它最终还会把表达式值的所有权返回！
// dbg! 输出到标准错误输出 stderr，而 println! 输出到标准输出 stdout
#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

struct PorkerCard {
    suit: PokerSuit,
    value: u8,
}

// 更好的方式
enum PokerCard {
    Clubs(String),
    Spades(String),
    Diamonds(String),
    Hearts(String),
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}

// 由于每个结构体都有自己的类型，因此我们无法在需要同一类型的地方进行使用，例如某个函数它的功能是接受消息并进行发送，那么用枚举的方式，
// 就可以接收不同的消息，但是用结构体，该函数无法接受 4 个不同的结构体作为参数。
// 而且从代码规范角度来看，枚举的实现更简洁，代码内聚性更强，不像结构体的实现，分散在各个地方。
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// 👆🏻枚举也可以用一下方式呈现
struct QuitMessage; // 单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}
