fn main() {
    //栈:栈按照顺序存储值并以相反顺序取出值，这也被称作后进先出。
    //堆:与栈不同，对于大小未知或者可能变化的数据，我们需要将它存储在堆上。

    //所有权原则
    /*
    Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
    一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
    当所有者（变量）离开作用域范围时，这个值将被丢弃(drop) 
    */
    
    /*
    {                      // s 在这里无效，它尚未声明
        let s = "hello";   // 从此处起，s 是有效的

        // 使用 s
    }                      // 此作用域已结束，s不再有效
     */

    //可以使用下面的方法基于字符串字面量来创建 String 类型：
    let s = String::from("hello");
    //:: 是一种调用操作符，这里表示调用 String 类型中的 from 关联函数，
    //由于 String 类型存储在堆上，因此它是动态的
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}", s); // 将打印 `hello, world!`

    //变量绑定背后的数据交互
    //转移所有权
    let x = 5;
    let y = x;
    /*这段代码并没有发生所有权的转移，原因很简单： 代码首先将 5 绑定到变量 x，
    接着拷贝 x 的值赋给 y，
    最终 x 和 y 都等于 5，因为整数是 Rust 基本数据类型，是固定大小的简单值，
    因此这两个值都是通过自动拷贝的方式来赋值的，都被存在栈中，完全无需在堆上分配内存。
    整个过程中的赋值都是通过值拷贝的方式完成（发生在栈中），因此并不需要所有权转移。*/


    let s1 = String::from("hello");
    let s2 = s1;
    /*上面一样，把 s1 的内容拷贝一份赋值给 s2，实际上，并不是这样。
    之前也提到了，对于基本类型（存储在栈上），Rust 会自动拷贝，
    但是 String 不是基本类型，而且是存储在堆上的，因此不能自动拷贝。*/
    /*String 类型是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量共同组成，
    其中堆指针是最重要的，它指向了真实存储字符串内容的堆内存，至于长度和容量，
    如果你有 Go 语言的经验，这里就很好理解：
    容量是堆内存分配空间的大小，长度是目前已经使用的大小。*/
    //println!("{}, world!", s1);//这行代码会报错，因为 s1 的所有权已经转移给了 s2，s1 不再有效
    println!("{}, world!", s2);//这行代码不会报错，因为 s2 仍然有效

    /*现在再回头看看之前的规则，相信大家已经有了更深刻的理解：
    Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
    一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
    当所有者（变量）离开作用域范围时，这个值将被丢弃(drop)
    如果你在其他语言中听说过术语 浅拷贝(shallow copy) 和 深拷贝(deep copy)，
    那么拷贝指针、长度和容量而不拷贝数据听起来就像浅拷贝，
    但是又因为 Rust 同时使第一个变量 s1 无效了，因此这个操作被称为 移动(move)，而不是浅拷贝。
    上面的例子可以解读为 s1 被移动到了 s2 中。*/

    let x: &str = "hello, world";
    let y = x;
    println!("{},{}",x,y);
    /*这段代码和之前的 String 有一个本质上的区别：
    在 String 的例子中 s1 持有了通过String::from("hello") 创建的值的所有权，
    而这个例子中，x 只是引用了存储在二进制可执行文件( binary )中的
    字符串 "hello, world"，    并没有持有所有权。*/

    //(深拷贝)
    /*首先，Rust 永远也不会自动创建数据的 “深拷贝”。
    因此，任何自动的复制都不是深拷贝，可以被认为对运行时性能影响较小。
    如果我们确实需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，
    可以使用一个叫做 clone 的方法。
    */
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    //但是对于执行较为频繁的代码（热点路径），使用 clone 会极大的降低程序性能，需要小心使用！

    //拷贝(浅拷贝)
    //浅拷贝只发生在栈上，因此性能很高，在日常编程中，浅拷贝无处不在。
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    //因为整数是固定大小的简单值，因此这些值会被存储在栈上，拷贝数据只需要很少的开销。
    //但是注意：可变引用 &mut T 是不可以 Copy的

    //函数传值与返回
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效
    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x
    // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
    // 所以不会有特殊操作
    
    //println!("在move进函数后继续使用s: {}",s);
    //这行代码会报错，因为 s 的所有权已经转移给了 takes_ownership 函数，s 不再有效


    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中,
                                        // 它也将返回值移给 s3




    //2.引用与解引用
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);//如果希望对 y 的值做出断言，
    //必须使用 *y 来解出引用所指向的值（也就是解引用）。

    let s1 = String::from("hello");
    //我们用 s1 的引用作为参数传递给 calculate_length 函数，而不是把 s1 的所有权转移给该函数
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    /*能注意到两点：
    无需像上章一样：先通过函数参数传入所有权，然后再通过函数返回来传出所有权，代码更加简洁
    calculate_length 的参数 s 类型从 String 变为 &String*/



    /*fn main() {
        let s = String::from("hello");
        change(&s);
    }
    fn change(some_string: &String) {
        some_string.push_str(", world");
    } */
   //正如变量默认不可变一样，引用指向的值默认也是不可变的，没事，来一起看看如何解决这个问题。
   /*
   fn main() {
    let mut s = String::from("hello");
    change(&mut s);
  }
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    */
    //声明 s 是可变类型，其次创建一个可变的引用 &mut s 
    //和接受可变引用参数 some_string: &mut String 的函数。

    //不过可变引用并不是随心所欲、想用就用的，它有一个很大的限制： 
    //同一作用域，特定数据只能有一个可变引用：
    let mut s = String::from("hello");
    let r1 = &mut s; println!("{}", r1);
    let r2 = &mut s; println!("{}", r2);

    //上面代码ok,下面代码报错
    /*let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);*/

    /*对于新手来说，这个特性绝对是一大拦路虎，
    也是新人们谈之色变的编译器 borrow checker 特性之一，不过各行各业都一样，限制往往是出于安全的考虑，Rust 也一样。

    这种限制的好处就是使 Rust 在编译期就避免数据竞争，数据竞争可由以下行为造成：
    两个或更多的指针同时访问同一数据
    至少有一个指针被用来写入数据
    没有同步数据访问的机制 */
    
    //很多时候，大括号可以帮我们解决一些编译不通过的问题，通过手动限制变量的作用域：
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
    

    //可变引用与不可变引用不能同时存在
    //下面的代码会导致一个错误：
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{}, and {}", r1, r2);
    //let r3 = &mut s; // 大问题
    //println!("{}, {}, and {}", r1, r2, r3);
    //正在借用不可变引用的用户，肯定不希望他借用的东西，被另外一个人莫名其妙改变了
    //注意，引用 r1,r2,r3 的作用域从创建开始，一直持续到它最后一次使用的地方 println!(....)，
    //这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号 }


    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束
    let r3 = &mut s;
    println!("{}", r3);
    // 在老版本的编译器中（Rust 1.31 前），将会报错，
    // 因为 r1 和 r2 的作用域在花括号 } 处结束，
    // 那么 r3 的借用就会触发 无法同时借用可变和不可变 的规则。



    /*NLL
    对于这种编译器优化行为，
    Rust 专门起了一个名字 —— Non-Lexical Lifetimes(NLL)，专门用于找到某个引用在作用域(})
    结束前就不再被使用的代码位置。*/
    //let reference_to_nothing = dangle();

    //借用规则总结
    /*总的来说，借用规则如下：
    同一时刻，你只能拥有要么一个可变引用，要么任意多个不可变引用
    引用必须总是有效的*/
}

//fn dangle() -> &String { // dangle 返回一个字符串的引用
//    let s = String::from("hello"); // s 是一个新字符串
    //&s // 返回字符串 s 的引用
//} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！

fn calculate_length(s: &String) -> usize {
    s.len()
}


// 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
                                             // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作


/*所有权很强大，避免了内存的不安全性，但是也带来了一个新麻烦： 
总是把一个值传来传去来使用它。 传入一个函数，很可能还要从该函数传出去，
结果就是语言表达变得非常啰嗦，幸运的是，Rust 提供了新功能解决这个问题。 */