fn main() {
    let circle = Circle::new(1.0, 2.1, 5.34);
    println!("{:?}", circle.area());
    let rec1 = Rectangle {
        width: 30,
        height: 10,
    };
    println!("the area of the rec1 is {:?} square pixels", rec1.area());
}

// 定义方法
// rust使用impl来定义方法

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}
// 其它语言中所有定义都在 class 中，但是 Rust 的对象定义和方法定义是分离的，这种数据和使用分离的方式，会给予使用者极高的灵活度。
impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle { x, y, radius }
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

// 该例子定义了一个 Rectangle 结构体，并且在其上定义了一个 area 方法，用于计算该矩形的面积。
// impl Rectangle {} 表示为 Rectangle 实现方法(impl 是实现 implementation 的缩写)，这样的写法表明 impl 语句块中的一切都是跟 Rectangle 相关联的。
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 在 area 的签名中，我们使用 &self 替代 rectangle: &Rectangle，&self 其实是 self: &Self 的简写（注意大小写）。
// 在一个 impl 块内，Self 指代被实现方法的结构体类型，self 指代此类型的实例，换句话说，self 指代的是 Rectangle 结构体实例，
// 这样的写法会让我们的代码简洁很多，而且非常便于理解：我们为哪个结构体实现方法，那么 self 就是指代哪个结构体的实例。
// 需要注意的是，self 依然有所有权的概念：
// self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
// &self 表示该方法对 Rectangle 的不可变借用
// &mut self 表示可变借用

// 在 Rust 中，允许方法名跟结构体的字段名相同
// 当我们使用 rect1.width() 时，Rust 知道我们调用的是它的方法，如果使用 rect1.width，则是访问它的字段。
// 一般来说，方法跟字段同名，往往适用于实现 getter 访问器

// 如何为一个结构体定义一个构造器方法？也就是接受几个参数，然后构造并返回该结构体的实例。其实答案在开头的代码片段中就给出了，很简单，不使用 self 中即可。
// 这种定义在 impl 中且没有 self 的函数被称之为关联函数： 因为它没有 self，不能用 f.read() 的形式调用，因此它是一个函数而不是方法，它又在impl 中，与结构体紧密关联，因此称为关联函数。
// 在之前的代码中，我们已经多次使用过关联函数，例如 String::from，用于创建一个动态字符串。
// 因为是函数，所以不能用 . 的方式来调用，我们需要用 :: 来调用，例如 let sq = Rectangle::new(3, 3);。这个方法位于结构体的命名空间中：:: 语法用于关联函数和模块创建的命名空间。


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
}
