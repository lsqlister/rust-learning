#![allow(unused_variables)]
//全局关闭未使用变量警告

use num::complex::Complex;
use std::fmt::Debug;

fn main() {
    method1();//数值类型
    method2();//字符、布尔、单元类型
    method3();//语句和表达式
    method4();//函数
}
//函数
fn method4() {
    let result = add(5, 6);
    println!("5 + 6 = {}", result);

    another_function(5, 6.1);

    let x = plus_five(5);
    println!("The value of x is: {}", x);

    let x = plus_or_minus(5);
    println!("The value of x is: {}", x);
}

//下面的函数创建了一个无限循环，该循环永不跳出，因此函数也永不返回
fn forever() -> ! {
  loop {
    //...
  };
}

//当用 ! 作函数返回类型的时候，表示该函数永不返回( diverging functions )，
//特别的，这种语法往往用做会导致程序崩溃的函数
fn dead_end() -> ! {
  panic!("你已经到了穷途末路，崩溃吧！");
}
//下面的 report 函数会隐式返回一个 ()
fn report<T: Debug>(item: T) {
  println!("{:?}", item);
}
//与上面的函数返回值相同，但是下面的函数显式的返回了 ()
fn clear(text: &mut String) -> () {
  *text = String::from("");
}

fn plus_or_minus(x:i32) -> i32 {
    if x > 5 {
        return x - 5
    }
    x + 5
}

/*函数的返回值就是函数体最后一条表达式的返回值，
当然我们也可以使用 return 提前返回，
下面的函数使用最后一条表达式来返回一个值*/
fn plus_five(x:i32) -> i32 {
    x + 5
}

fn another_function(x: i32, y: f32) {//去掉 x 或者 y 的任何一个的类型，都会报错
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn add(i: i32, j: i32) -> i32 {
   i + j
}
//语句和表达式
fn method3() {
    //语句
    let a = 8;
    let b: Vec<f64> = Vec::new();
    let (a, c) = ("hi", false);
    //以上都是语句，它们完成了一个具体的操作，但是并没有返回值，因此是语句。

    //由于 let 是语句，因此不能将 let 语句赋值给其它值，如下形式是错误的：
    //let b = (let a = 8);

    //表达式
    //调用一个函数是表达式，因为会返回一个值，调用宏也是表达式，
    //用花括号包裹最终返回一个值的语句块也是表达式，总之，能返回值，它就是表达式:
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    assert_eq!(ret_unit_type(), ());
}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 或者写成一行
    let z = if x % 2 == 1 { "odd" } else { "even" };
}

//Rust 的函数体是由一系列语句组成，最后由一个表达式来返回值
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

//字符、布尔、单元类型
fn method2() {
    //字符类型(char)
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", size_of_val(&x));

    //布尔(bool)
    let t = true;
    let f: bool = false; // 使用类型标注,显式指定f的类型
    if f {
        println!("这是段毫无意义的代码");
    }

    //单元类型(unit)
    /*单元类型就是 () ，对，你没看错，就是 () ，唯一的值也是 () ，
    一些读者读到这里可能就不愿意了，你也太敷衍了吧，管这叫类型？
    只能说，再不起眼的东西，都有其用途，在目前为止的学习过程中，
    大家已经看到过很多次 fn main() 函数的使用吧？那么这个函数返回什么呢？
    没错， main 函数就返回这个单元类型 ()，因此 main 函数有返回值，
    而没有返回值的函数在 Rust 中是有单独的定义的：
    发散函数( diverging functions )（-> !），顾名思义，无法收敛的函数。
    例如常见的 println!() 的返回值也是单元类型 ()。
    再比如，你可以用 () 作为 map 的值，表示我们不关注具体的值，只关注 key。 
    这种用法和 Go 语言的 struct{} 类似，可以作为一个值用来占位，
    但是完全不占用任何内存，内存占用为0字节。*/
}

//数值类型
fn method1() {
    //类型推导与标注
    //let guess = "42".parse().expect("Not a number!");
    let guess: i32 = "42".parse().expect("Not a number!");
    println!("guess = {}", guess); // 输出: guess = 42

    /*显式处理可能的溢出，可以使用标准库针对原始数字类型提供的这些方法：
    使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
    如果使用 checked_* 方法时发生溢出，则返回 None 值
    使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
    使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值，*/
    //例如:
    assert_eq!(100u8.saturating_add(1), 101);
    assert_eq!(u8::MAX.saturating_add(127), u8::MAX);

    // 声明一个 u8 类型的变量 a，赋值为 255（u8 的最大值）
    let a: u8 = 255;
    // 使用 wrapping_add 方法对 a 进行加法运算
    // 普通加法 255 + 20 = 275，但 u8 只能表示 0~255
    // wrapping_add 会让结果"环绕"回 0：275 - 256 = 19
    let b = a.wrapping_add(20);
    // 打印结果，输出: 19
    println!("{}", b);  // 19


    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x = {}, y = {}", x, y);

    // 断言0.1 + 0.2与0.3相等
    assert!(0.1 + 0.2 != 0.3);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    //assert!(xyz.0 + xyz.1 == xyz.2);
    assert!(xyz.0 + xyz.1 != xyz.2);
    /*0.1 + 0.2 的结果是 3e99999a，0.3 也是 3e99999a，
    因此 f32 下的 0.1 + 0.2 == 0.3 通过测试，
    但是到了 f64 类型时，结果就不一样了，因为 f64 精度高很多，
    因此在小数点非常后面发生了一点微小的变化，0.1 + 0.2 以 4 结尾，
    但是 0.3 以3结尾，这个细微区别导致 f64 下的测试失败了，并且抛出了异常。*/

    //所有跟 NaN 交互的操作，都会返回一个 NaN，
    //而且 NaN 不能用来比较，下面的代码会崩溃：
    let x = (-42.0_f32).sqrt();
    println!("{}", x);  // NaN
    //assert_eq!(x, x);

    //出于防御性编程的考虑，可以使用 is_nan() 等方法，可以用来判断一个数值是否是 NaN ：
    if x.is_nan() {
        println!("未定义的数学行为")
    }

    //加法、减法、乘法、除法和取模运算
    // 加法
    let sum = 5 + 10;
    println!("{}", sum);  // 15
    // 减法
    let difference = 95.5 - 4.3;
    println!("{}", difference);  // 91.2
    // 乘法
    let product = 4 * 30;
    println!("{}", product);  // 120
    // 除法
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);  // 1.7608695652173912
    // 求余
    let remainder = 43 % 5;
    println!("{}", remainder);  // 3


    //综合性的示例：
    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);


    //位运算
    // 无符号8位整数，二进制为00000010
    let a: u8 = 2; // 也可以写 let a: u8 = 0b_0000_0010;
    // 二进制为00000011
    let b: u8 = 3;
    // {:08b}：左高右低输出二进制01，不足8位则高位补0
    println!("a value is        {:08b}", a);
    println!("b value is        {:08b}", b);
    println!("(a & b) value is  {:08b}", a & b);
    println!("(a | b) value is  {:08b}", a | b);
    println!("(a ^ b) value is  {:08b}", a ^ b);
    println!("(!b) value is     {:08b}", !b);
    println!("(a << b) value is {:08b}", a << b);
    println!("(a >> b) value is {:08b}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {:08b}", a);

    let a: u8 = 255;
    let b = a>>7; // ok
    let b = a<<7; // ok
    //let b = a>>8; // overflow
    //let b = a<<8; // overflow
    
    //序列(Range)
    for i in 1..=5 {     
       println!("{}",i);
    }

    for i in 'a'..='z' {
        println!("{}",i);
    }

    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}
