use num::complex::Complex;
fn main() {
    // 8 位	    i8	u8
    // 16 位	i16	u16
    // 32 位	i32	u32
    // 64 位	i64	u64
    // 128 位	i128	u128
    // 视架构而定	isize	usize
    // 假设有一个 u8 ，它可以存放从 0 到 255 的值。
    // 那么当你将其修改为范围之外的值，比如 256，
    // 则会发生整型溢出。关于这一行为 Rust 有一些有趣的规则：
    // 当在 debug 模式编译时，Rust 会检查整型溢出，若存在这些问题，
    // 则使程序在编译时 panic
    // let x:i8 = 127;
    let x: i16 = 1 << 15 - 1;

    println!("{}", x);
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }
    // 加法
    let _sum = 5 + 10;

    // 减法
    let _difference = 95.5 - 4.3;

    // 乘法
    let _product = 4 * 30;

    // 除法
    let _quotient = 56.7 / 32.2;

    // 求余
    let _remainder = 43 % 5;
    // 二进制为00000010
    let a: i32 = 2;
    // 二进制为00000011
    let b: i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
    // range
    for i in 1..=10 {
        println!("{i}")
    }
    // 复数
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.5);
    let result = a + b;
    println!("{}+{}i", result.re, result.im);
    // Rust 拥有相当多的数值类型. 因此你需要熟悉这些类型所占用的字节数，这样就知道该类型允许的大小范围以及你选择的类型是否能表达负数
    // 类型转换必须是显式的. Rust 永远也不会偷偷把你的 16bit 整数转换成 32bit 整数
    // Rust 的数值上可以使用方法. 例如你可以用以下方法来将 13.14 取整：13.14_f32.round()，在这里我们使用了类型后缀，因为编译器需要知道 13.14 的具体类型

    // 字符
    // Rust 的字符不仅仅是 ASCII，所有的 Unicode 值都可以作为 Rust 字符，
    // 包括单个的中文、日文、韩文、emoji 表情符号等等，都是合法的字符类型。
    // Unicode 值的范围从 U+0000 ~ U+D7FF 和 U+E000 ~ U+10FFFF。
    // Unicode 都是 4 个字节编码
    // Rust 的字符只能用 '' 来表示， "" 是留给字符串的
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!("字符{}占用了{}字节的内存",c,std::mem::size_of_val(&c));
    println!("字符{}占用了{}字节的内存",z,std::mem::size_of_val(&z));
    println!("字符{}占用了{}字节的内存",g,std::mem::size_of_val(&g));
    println!("字符{}占用了{}字节的内存",heart_eyed_cat,std::mem::size_of_val(&heart_eyed_cat));
    // 布尔
    // Rust 中的布尔类型有两个可能的值：true 和 false，布尔值占用内存的大小为 1 个字节
    let f:bool= false;
    if f {
        println!("{}",f)
    }else {
        println!("{}",!f)
    }
    // 单元类型
    // 单元类型就是 () ，唯一的值也是 ()
    // main 函数就返回这个单元类型 ()
    // 不能说 main 函数无返回值，因为没有返回值的函数在 Rust 中是有单独的定义的：发散函数( diverge function )，顾名思义，无法收敛的函数
    // 常见的 println!() 的返回值也是单元类型 ()。
    // 再比如，你可以用 () 作为 map 的值，表示我们不关注具体的值，只关注 key。 这种用法和 Go 语言的 struct{} 类似，可以作为一个值用来占位，但是完全不占用任何内存
}
