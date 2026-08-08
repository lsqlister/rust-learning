use num::complex::Complex;

fn main() {
    method1();//数值类型
    method2();//字符、布尔、单元类型
}

//字符、布尔、单元类型
fn method2() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
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
