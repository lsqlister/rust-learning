fn main() {
    //可变性很简单，只要在变量名前加一个 mut 即可
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //使用下划线开头忽略未使用的变量
    let _x = 5;
    let _y = 10;

    //变量解构
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);
    // 此时 a = true, b = false

    b = true;               // b 被重新赋值为 true
    assert_eq!(a, b);       // a == b (true == true)，断言成功


    //在赋值语句的左式中使用元组、切片和结构体模式了
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    //变量遮蔽(shadowing)
    //Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的
    let m = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let m = m + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let m = m * 2;
        println!("The value of m in the inner scope is: {}", m);
    }

    println!("The value of m is: {}", m);   

}


struct Struct {
    e: i32
}