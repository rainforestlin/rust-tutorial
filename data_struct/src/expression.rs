pub(crate) fn expression() {
    let (a, b): (i32, i32) = (100, 200);
    let result = add_with_extra(a, b);
    println!("{result}")
}
// 函数名和变量名使用蛇形命名法(snake case)，例如 fn add_two() -> {}
// 函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
// 每个函数参数都需要标注类型
// 在 Rust 中函数就是表达式，因此我们可以把函数的返回值直接赋给调用者。
// 函数的返回值就是函数体最后一条表达式的返回值，当然我们也可以使用
//  return 提前返回，下面的函数使用最后一条表达式来返回一个值
// 当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )，特别的，这种语法往往用做会导致程序崩溃的函数：
fn add_with_extra(x: i32, y: i32) -> i32 {
    // 语句会执行一些操作但是不会返回一个值，而表达式会在求值后返回一个值
    let x = x + 1; //语句
    let y = y + 15; // 语句
                    // 不能以分号结尾，否则就会从表达式变成语句， 表达式不能包含分号
                    // 表达式如果不返回任何值，会隐式地返回一个 ()
    if x > 1000 {
        return x - y;
    }
    let _whatever = x + y; //语句
    x + y //表达式
}
