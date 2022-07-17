fn main() {
    let x = 120;
    println!("print x = {x}");
    // cannot assign twice to immutable variable
    // x = 6;
    // 显式的声明可变变量
    // 选择可变还是不可变取决于使用场景。
    // 不可变变量带来的好处是安全性，损失的是灵活性和需要改变量时需要重新分配内存
    // 可变变量最大的好处是使用上的灵活性和性能上的提升
    // rust更倾向于称该行为为变量绑定而非变量声明
    // rust在变量上和go的区别在于对内存的使用。rust认为内存都是有主的，讲内存与变量绑定后，该内存就只属于该变量。而go中，变量的内存地址可能会发生变化。
    // 这可能就是rust没有gc的原因之一？
    let mut y = 120;
    println!("y = {y}");
    y = 130;
    println!("changed y = {y}");
    // 此时的z可以声明后不使用。
    let mut _z = 500;
    // 变量解构
    let (a, mut b): (bool, bool) = (true, true);
    println!("a={:?},b={:?}", a, b);
    assert_eq!(a, b);
}

struct Struct {
    e: i32,
}
