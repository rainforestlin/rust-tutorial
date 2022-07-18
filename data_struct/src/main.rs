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
     let a:i32 = 2;
     // 二进制为00000011
     let b:i32 = 3;
 
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
}
