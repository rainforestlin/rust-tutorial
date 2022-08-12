// 通常会有需求 用同一功能的函数处理不同类型的数据，例如两个数的加法，无论是整数还是浮点数，甚至是自定义类型，
// 都能进行支持。在不支持泛型的编程语言中，通常需要为每一种类型编写一个函数

use core::str;

fn add_i8(a: i8, b: i8) -> i8 {
    a + b
}
fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}
fn add_f64(a: f64, b: f64) -> f64 {
    a + b
}
// 上述代码可以正常运行，但是很啰嗦

fn main() {
    println!("add i8: {}", add_i8(2i8, 3i8));
    println!("add i32: {}", add_i32(20, 30));
    println!("add f64: {}", add_f64(1.23, 1.23));
    let number_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let result = largest(&number_list);
    println!("{}", result);
    // 提前声明，跟泛型函数定义类似，首先我们在使用泛型参数之前必需要进行声明 Point<T>，
    // 接着就可以在结构体的字段类型中使用 T 来替代具体的类型
    // x 和 y 是相同的类型
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = PointT { x: 1.0, y: 2 };
    {
        // 枚举中使用泛型
        // 这个枚举和 Option 一样，主要用于函数返回值，与 Option 用于值的存在与否不同，Result 关注的主要是值的正确性。
        // 如果函数正常运行，则最后返回一个 Ok(T)，T 是函数具体的返回值类型，如果函数异常运行，则返回一个 Err(E)，E 是错误类型。
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }
    {
        // 使用泛型参数前，依然需要提前声明：impl<T>，只有提前声明了，我们才能在Point<T>中使用它，这样 Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型。需要注意的是，
        // 这里的 Point<T> 不再是泛型声明，而是一个完整的结构体类型，因为我们定义的结构体就是 Point<T> 而不再是 Point
        impl<T> Point<T> {
            fn new(x: T, y: T) -> Self {
                Point { x: x, y: y }
            }
            pub fn x(&self) -> &T {
                &self.x
            }
        }
    }
    {
        let coffee_store = CoffeeStore {
            name: "壹间万物".to_string(),
            coffee_type: "瑰夏".to_string(),
        };
        let car_shop = CarShop {
            name: "宝渝".to_string(),
            car_brand: "BMW".to_string(),
        };
        println!("{}", car_shop.sale());
        println!("{}", coffee_store.sale());
        let default = default_shop {
            name: "default".to_string(),
        };
        println!("{}", default.sale());
        println!("{}", default.Buy())
    }
}

// 实际上，泛型就是一种多态。泛型主要目的是为程序员提供编程的便利，减少代码的臃肿，同时可以极大地丰富语言本身的表达能
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//以下代码的问题
// 主要是返回的T，目前的rust会判定T没有实现Copy,所以，无法返回一个完整所有权，只能返回一个借用

// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// 结构体中的字段类型也可以用泛型来定义，下面代码定义了一个坐标点 Point，它可以存放任何类型的坐标值
struct Point<T> {
    x: T,
    y: T,
}

// 如果想让 x 和 y 即能类型相同，又能类型不同，需要使用不同的泛型参数

struct PointT<T, U> {
    x: T,
    y: U,
}
// 定义特征

pub trait Goods {
    fn sale(&self) -> String {
        String::from("this is what you sale")
    }
    fn Buy(&self) -> String {
        String::from("that is what you buy")
    }
}

struct CoffeeStore {
    name: String,
    coffee_type: String,
}

struct CarShop {
    name: String,
    car_brand: String,
}

struct default_shop {
    name: String,
}
// 特征。和golang中的interface很像
// 实现特征的语法与为结构体、枚举实现方法很像
// 关于特征实现与定义的位置，有一条非常重要的原则：如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！
// 例如我们可以为上面的 Post 类型实现标准库中的 Display 特征，这是因为 Post 类型定义在当前的作用域中。同时，我们也可以在当前包中为
//  String 类型实现 Summary 特征，因为 Summary 定义在当前作用域中。
// 但是你无法在当前作用域中，为 String 类型实现 Display 特征，因为它们俩都定义在标准库中，其定义所在的位置都不在当前作用域，跟你半毛钱关系都没有，看看就行了。
// 该规则被称为孤儿规则，可以确保其它人编写的代码不会破坏你的代码，也确保了你不会莫名其妙就破坏了风马牛不相及的代码。
// 默认实现允许调用相同特征中的其他方法，哪怕这些方法没有默认实现。如此，特征可以提供很多有用的功能而只需要实现指定的一小部分内容。
impl Goods for CoffeeStore {
    fn sale(&self) -> String {
        format!(
            "{} has some type of coffee,such as {}",
            self.name, self.coffee_type
        )
    }
}

impl Goods for CarShop {
    fn sale(&self) -> String {
        format!(
            "{} has some type of car, such as {}",
            self.name, self.car_brand
        )
    }
}

impl Goods for default_shop {}
