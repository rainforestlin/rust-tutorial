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

        let s = &hello[0..2];
    }
    // 操作字符串
    {
        let mut s = String::from("hello");
        s.push_str(" world");
        s.insert_str(5, "!!!");
    }
}

fn greet(name: String) {
    println!("hello {}", name)
}

fn say_hello(s: &str) {
    println!("{}", s);
}
