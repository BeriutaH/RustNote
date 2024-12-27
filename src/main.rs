// use mysql::*;
// use mysql::prelude::*;
// use serde::Deserialize;
// use std::fs;


// #[derive(Debug, Deserialize)]
// struct DatabaseConfig {
//     host: String,
//     port: u16,
//     user: String,
//     password: String,
//     name: String,
//     pool_size: Option<u32>,
// }

// #[derive(Debug, Deserialize)]
// struct Config {
//     database: DatabaseConfig,
// }

// fn load_config() -> Config {
//     let config_content = fs::read_to_string("config.yml").expect("配置文件读取失败");
//     serde_yaml::from_str(&config_content).expect("配置解析失败")
// }
// use core::num;
// use std::io;  // 获取用户输入的库 std 表示标准库
// use std::cmp::Ordering;  // 枚举类型，内含三个类型
// use rand::Rng;  // 引入第三方包

// use std::{error::Error, f32::consts::E, fs::File, io::{self, Read}};

// use std::fmt::Display;


// use core::{hash, num};
// use std::{cell::RefCell, os::unix::thread, rc::{Rc, Weak}};
// use std::{thread, time::Duration, vec};

// use std::env::consts;


// use rut::Post;
// #[tokio::main]
// async fn main() {
mod ticket;

fn main() {
    // let secret_num = rand::thread_rng().gen_range(1, 101);
    // println!("猜数!请输入您所猜测的数字");
    // println!("随机数值是: {}", secret_num);

    // loop {
    //     let mut guess = String::new();  // mut 表示是可变的变量，如果没有声明就是不可变  :: 表示里面的静态函数
    //     io::stdin().read_line( &mut guess).expect("无法读取行");
    //     // guess.trim()把换行符,空格都去掉，parse()把字符串解析成某种数字类型
    //     let guess_new: u32 = match guess.trim().parse() {
    //         // Result 有两个值，一个是Ok,另一个是错误信息
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("您猜测的数值是: {}", guess_new);
    //     match guess_new.cmp(&secret_num) {  // cmp 内置方法，是比较大小
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }
    // let test_num: u32 = "42".parse().expect("Not a number/");
    // println!("{}", test_num);

    /* 标量类型 */ 
        // 整数类型：没有小数部分，默认类型就是i32
            //  u32： 一个无符号的整数类型，占据32位的空间
            //  无符号整数类型以U开头  没有负数
            //  有符号整数类型以i开头  有正有负
        // 浮点类型：含有小数部分的类型
            // f32，32位，单精度
            // f64，64位，双精度  默认
        // let x = 2.0;
        // let y: f32 = 3.0;
        // let sum = 5 + 10;
        // let difference = 95.5 - 4.3;
        // let product = 4 * 30;
        // let quotient = 56.7 / 32.2;
        // let reminder = 54 % 5;
        // let x = "z";
        // let y: char = '$';  // 这种文字需要用单引号
        // let x = "😁";
    
    /* 复合类型 */
        // 元组，可以将多个类型的多个值放在一个类型里面。长度是固定的，一旦声明就无法改变
            // let tup = (500, 6.4, 1);
            // println!("{}, {}, {}", tup.0, tup.1, tup.2);
            // // 使用模式匹配来结构元组
            // let (x, y, z) = tup;
            // println!("{} | {} | {}", x, y, z)
        // 数组，可以将多个值放在一个类型里，但是每个元素的类型必须相同，长度固定
            // let list: [i32; 6] = [1,2,3,4,5,6];
            // let a: [i32; 5] = [3;5];  // 相当于 let a = [3,3,3,3,3];
            // println!("{}", a[2]);
        // Vector 比数组灵活，由标准库提供
    // another_fun(5, 6);
    
    // let x = 5;
    // let y = {
    //     let x = 1;
    //     x + 3  // 加上; 这行代码就变成一个语句，而语句是没有返回值的，相当于()
    // };
    // println!("The value of y is : {}", y)

    /* 函数的返回值 */
        // 在 -> 符号后边声明函数返回值的类型，但是不可为返回值命名
        // 在Rust里面，函数值就是函数体里面最后一个表达式的值
        // 若是想提前返回，需要使用return关键字，并指定一个值，大多数函数都是默认使用最后一个表达式作为最后的返回值
        // let x = five(4);
        // println!("调用函数five: {}", x)

    /* if表达式 */
        // 条件必须是bool类型的
        // if表达式中，与条件相关联的代码块就叫做分支（arm）
        // let number = 18;
        // if number < 5 {
        //     println!("当表达式为true");
        // } else {
        //     println!("当表达式为false");
        // }
        // 如果用else if 当满足第一个判断时，就不会再往下走
        // if number % 4 == 0 {
        //     println!("当前数字可以被4整除");
        // } else if number % 3 == 0 {
        //     println!("当前数字可以被3整除");
        // } else if number % 2 == 0 {
        //     println!("当前数字不可以被4,3,2整除");
        // } else {
        //     println!("当前数字可以被2整除");
        // }
        // 如果使用多个else if，那么最好使用match来重构代码
        // match number {
        //     // n是一个模式变量,在每个分支中，n都代表了被匹配到的数字值
        //     n if n % 4 == 0 => println!("当前数字可以被4整除"),
        //     n if n % 3 == 0 => println!("当前数字可以被3整除"),
        //     n if n % 2 == 0 => println!("当前数字可以被2整除"),
        //     // _是通配符，所有不符合上述条件的都会走_这个分支
        //     _ => println!("当前数字可以被2整除"),
        // }
        // let condition = true;
        // let number = if condition {5} else {6};
        // println!("最终得到数字：{}", number);

    /* Rust循环 */
        // rust提供了3种循环，loop, while, for

        // loop, 告诉Rust反复的执行一块代码，直到触发停止条件，所以在循环里可以用break关键字来告诉程序合适停止循环
            // let mut counter = 0;
            // let result = loop {
            //     counter += 1;
            //     if counter == 10 {
            //         break counter * 2;
            //     }
            // };
            // println!("最终的数字结果为: {}", result)

        // while 条件循环，每次执行循环体之前都判断一次条件
            // let mut number = 3;
            // while number != 0 {
            //     println!("number 数值: {}", number);
            //     number = number - 1;
            // }
            // println!("循环结束！")
            
            // let alist = [10, 20, 30, 40, 50, 60];
            // let mut index = 0;
            // // 对于集合的遍历，下面的方法执行会比较慢，因为每次循环都要判断一下，所以一般使用for循环
            // while index < alist.len() {
            //     println!("获取到list值为 {}", alist[index]);
            //     index = index + 1;
            // }

        // for循环，可以针对集合中的每个元素执行一些代码
            // let alist = [10, 20, 30, 40, 50, 60];
            // for e in alist.iter() {
            //     println!("值为: {}", e)
            // }

        // range 循环，标准库，制定一个开始数字和一个结束数字，range可以生成之间的数字（不包含结束），rev方法可以反转range
            // for n in (1..4).rev() {  // 用了rev是倒序，3.2.1
            //     println!("数字: {}", n);
            // }
            // println!("循环结束")

    /* 所有权 */
        // rust的核心特性就是所有权
        // 所有程序运行时都必须管理它们使用计算机内存的方式 
        // 有些语言有垃圾收集机制（Java、C#、Python），在程序运行时，它们会不断地寻找不再使用的内存，但是会带来一些性能开销
        // 在其他语言中，程序员必须显式地分配和释放内存

        // rust采用了第三种方式
            // 内存是通过一个所有权系统来管理，其中包含一组编译器在编译时检查的规则
            // 当程序运行时，所有权特性不会减慢程序的运行速度 

        // 栈内存（stack）和 堆内存（heap）
            // 在rust中，一个值是在stack上还是在heap上对语言的行为和你为什么要做某些决定是有更大影响的
            // stack，按照值的接收顺序来储存，按相反的顺序将它们移除（后进先出，LIFO）
                // -添加数据叫 压入栈
                // -移除数据叫 弹出栈
                // 所有存储在stack上的数据必须拥有已知的固定的大小，编译时大小未知的数据或运行时大小可能发生变化的数据必须存放在heap上
            // heap，内存组织性差一点，当把数据放入heap时，会请求一定数量的空间
                // 操作系统在heap里找到一块足够大的空间，把它标记为在用，并返回一个指针，也就是这个空间的地址，这个过程叫做在heap上进行分配，有时仅仅称为”分配“
            // 把值压入stack上不叫分配，并且因为指针是已知的固定大小的，可以把指针存放在stack上
                // 如果想要实际数据，必须使用指针来定位
            // 在heap上分配空间需要做更多的工作，操作系统首先需要找一个足够大的空间来存放数据，然后做好记录方便下次分配
            // 访问heap中的数据要比访问stack中的数据慢，因为需要通过指针才能找到heap中的数据，对于现代的处理器来说，由于缓存的缘故，如果指令在内存中跳转的次数越少，那么速度就越快
            // 如果数据存放的距离比较近，那么处理器的处理速度就会更快一些（stack上）
            // 如果数据之间的距离比较远，那么处理速度就会慢一些（heap上）在heap上分配大量的空间也是需要时间的

        // 所有权解决的问题：
            // 1. 跟踪代码的那些部分正在使用heap的那些数据
            // 2. 最小化heap上的重复数据量
            // 3. 清理heap上未使用的数据以避免空间不足
            // 所以，管理heap数据就是所有权存在的原因

        // 所有权的规则
            // 每个值都有一个变量，这个变量是法治的所有者
            // 每个值同时只能有一个所有者
            // 当所有者超出作用域（scope）时，该值将被删除
        // 作用域
            // fn dd() {
            //     // s 作用域不可用
            //     let s = "hello";  // s 可用
            //     // 可以对s进行相关操作
            // }  // s 作用域到此结束，s 不可再用
            
        /* 函数调用 */
            // 当代码调用函数时，值被传入到函数（也包括指向heap的指针），函数本地的变量被压倒stack上，当函数结束后，这些值就从stack上弹出
        
        // string类型，比基础标量数据类型更复杂，所以在此以string类型举例使用所有权 
            // 创建string类型，可以使用from函数从字符串字面值创建出string类型
            // let mut str_info = String::from("hello");  // :: 表示from是String类型下的函数
            // str_info.push_str(", World!");
            // println!("{}", str_info);

            // let str_info = "hello";  
            // let str_info2 = "hello";  
            // let str_info3 = str_info.to_owned() + str_info2;  // .to_owned() 方法会在堆上分配内存,并将 &str 的内容复制到新分配的 String 中
            // println!("{}", str_info3);
            // 当走出作用域是，rust会自动调用drop这个函数

        /* 变量和数据交互的方式： 移动（move） */
            // 因为整数拥有copy trait，也就是说所有的都在stack上，所以复制之后之前的是可以使用的
                // let x = 5;
                // let y = x;
                // println!("{}, {}", x, y)  // 此时不会报错，由于整数是已知且固定大小的简单的值，这两个5被压到了stack中，所以不需要释放
            // 字符串类型只有一部分在stack上（指针），所以复制之后，之前的在stack上的就会失效，也就是所有权被转移
                // let s1 = String::from("我是在stack上的，我的数据则存放在heap上");
                // let s2 = s1;  // 此时，s2复制了一份s1在stack上的数据，但是并没有把heap上的数据加载进来，并且此时s1已经失效
                // println!("{}", s1)  // 所以此时如果再尝试打印s1，就会报错，因为s1的所有权被转移到了s2
            // 一些拥有Copy trait的类型
                // 任何简单标量的组合类型都可以Copy的
                // 任何需要分配内存或某种资源的都不是Copy的
                // 例如：
                    // 所有的整数类型，u32...
                    // bool
                    // char
                    // 所有的浮点类型 f64...
                    // Tuple(元组),如果里面所有的字段都是Copy的
                        // (i32,f64)是
                        // (i32, String)不是

        /* 所有权与函数 */
            // 函数的入参
                // let s = String::from("我是字符串");
                // take_own(s);   // s已经将所有权移动到这个函数的参数了
                // // println!("{}", s)  // 此时s的所有权一倍移交给take_own，所以无法再调用s
                // let x = 4;
                // make_copy(x);
                // println!("将x传入函数之后,仍然能调用x{}", x);

            // 函数的返回值与作用域
                // 函数子啊返回值的过程中同样会发生所有权的转移
                // 一个变量的所有权总是遵循同样的模式：把一个值赋给其他变量时就会发生移动，当一个包含heap数据的变量离开作用域时，它的值就会被drop函数清理，除非数据的所有权移动到另一个变量上了
                    // let s1 = gives_own();
                    // let s2 = String::from("New");
                    // let s3 = takes_and_give_back(s2);
                    // println!("\n S1: {}, S3: {}, S2被移交,无法再调用\n", s1, s3)
                // 如果 想要函数使用某个值，但是不获得其所有权，就需要使用完之后，将值返回
                    // let ss1 = String::from("我是SS1");
                    // let (ss2, lens) = calculate_length(ss1);
                    // println!("\n当前字符串为: {}, 字符串的长度为: {}\n", ss2, lens)
            
            // 引用和借用
                // 可变引用有一个重要的限制，在特定作用域内，对某一块数据，只能有一个可变的引用，这样在可编译时防止数据竞争
                    // let mut s4 = String::from("函数的引用");
                    // let lens = calculate_length(&mut s4);
                    // println!("\n字符串长度: {}\n", lens)
                // 数据竞争会出现以下三种场景
                    // 1. 两个或多个指针同时访问同一个数据
                    // 2. 至少有一个指针用于写入数据
                    // 3. 没有使用任何机制来同步对数据的访问
                        // let mut a1 = String::from("A1");
                        // let a2 = &mut a1;
                        // // {
                        // //     let a2 = &mut a1;
                        // //     println!("A2: {}", a2);
                        // // }
                        // let a3 = &mut a1;  // 在同一个作用域，在已经把a1借出去之后，不能再借给另外的变量，所以这里会报错
                        // println!("A2: {}, A3: {}", a2, a3);  
                // 不可以同时拥有一个可变引用和一个不变的引用，有以下原因
                    // 数据竞争：多个线程同时访问相同的数据，其中一个线程进行写操作，而其他线程进行读操作，这可能会导致数据不一致或者其他意外的结果。
                    // 内存不一致：由于可变引用允许修改数据，而不可变引用不允许修改数据，这可能导致数据在不同的引用之间不一致。
                        // let mut sq = String::from("可变数据");
                        // println!("{}", sq);
                        // sq.push_str("string");
                        // let r1 = &mut sq;
                        // println!("R1: {}", r1);
                        // let r2 = &sq;
                        // println!("R2: {}", r2);
                        // // println!("R1: {}, R2: {}", r1, r2)
                // 悬空引用
                    // 悬空指针，一个指针引用了内存中的某个地址，而这块内存可能已经释放并分配给其他人使用了
                // 引用的规则，在任何给定的时刻，只能满足下列条件之一
                    // 一个可变的引用
                    // 任意数量不可变的引用
                    // 引用必须一直有效 
            
            // rust另外一种不支持所有权的数据类型，切片
                // let mut s1 = String::from("你好 中国");
                // let w = first_world(&mut s1);
                // println!("{}", w);
                // s1 = format!("{}, Rust", s1);
                // println!("{}", s1)
                // let hello = &s1[..5];
                // println!("{}", hello);
                // let word = &s1[6..];
                // println!("{}", word);
                // let whole = &s1[..];
                // println!("{}", whole);
                // let word_index = first_world(&s1);
                // println!("{}", word_index)

                // let w1 = String::from("hi 你好");
                // let word = first_world(&w1[..]);
                // println!("{}", word);
                // let w2 = "heihei n";
                // let word2 = first_world(w2);
                // println!("{}", word2);
         
         /* struct */
            // 使用struct关键字，并为整个struct命名 
                // println!("现在是struct模块");
                // // 一旦struct的实例是可变的，那么实例中所有的字段都是可变的
                // let mut user = User {
                //     username: String::from("Beriuta"),
                //     email: String::from("abc@123.com"),
                //     active:true,
                // };
                // user.email = String::from("sdwdve@456.com");
                // user.username = String::from("小泼猴");
                // user.active = false;
                // println!("{:?}", user);
                // let user2 = User {
                //     username: String::from("小牛"),
                //     email: String::from("xiaoniu@123.com"),
                //     ..user
                // };
                // println!("{:?}", user2);
            // Tuple struct
                // 整体有个名，但里面的元素没有名
                // 适用于，像个整个tuple起名，并让它不属于其他tuple，而且又不需要给每个元素起名
                    // let black = Color (1,2,3);
                    // let origin = Point (11,22,33);
                    // println!("{:?}", black);
                    // println!("{:?}", origin);
                    // println!("Color第二个: {}", black.1);
                    // println!("Point第二个: {}", origin.1);
                // black 和 origin 是不同的类型，是不是的tuple struct的实例
            // struct 操作
                // let w = 30;
                // let l = 50;
                // println!("{}", area(w, l))
                // let rect = (30,50);
                // println!("{}", area(rect));
                // let rect = Rectangle {
                //     width: 30,
                //     length: 50,
                // };
                // println!("{}",area(&rect));
                // println!("{:#?}",rect) // 结构体添加#[derive(Debug)]，可以打印出观看友好的格式
            // struct方法与关联函数
                // 方法和函数类似：fn关键字，名称，参数，返回值
                // 方法与函数不同之处：
                    // 方法是在struct（或enum，trait对象）的上下文中定义
                    // 第一个参数是self，表示方法被调用的struct实例,在定义方法是，要有关键字impl，方法要在块里面定义
                        // let rect = Rectangle {
                        //     width: 30,
                        //     length: 50,
                        // };
                        // println!("{}",rect.area());
                // 方法调用的运算符
                    // c++/c 里面的方法调用： object->something() 和 (*object).something()一样
                    // rust没有->运算符
                        // 在调用方法时，rust根据情况自动添加&、&mut 或 * ，以便Object可以匹配方法的签名
                        // p1.distance(&p2);  == (&p1).distance(&p2);
                    // let rect1 = Rectangle {
                    //         width: 30,
                    //         length: 50,
                    //     };
                    // let rect2 = Rectangle {
                    //     width: 20,
                    //     length: 40,
                    // };
                    // let rect3 = Rectangle {
                    //     width: 60,
                    //     length: 70,
                    // };
                    // println!("1长方形是否能包含2长方形,{}", rect1.can_hold(&rect2));
                    // println!("2长方形是否能包含3长方形,{}", rect2.can_hold(&rect3))
                // 关联函数，可以在impl块中定义，不把self作为第一个参数的函数，注意：不是方法， 例如：String::from()
                    // 关联函数通常用于构造器 
                        // :: 1. 符号是调用关联函数  2. 模块创建的命名空间
                        // let s = Rectangle::square(50);
                        // println!("{:#?}", s);
        /* 枚举与模式匹配 */  
            // 枚举：允许我们列举所有可能的值来定义一个类型，用enum关键字
                // let four = IpAddrKind::Ipv4;
                // let six = IpAddrKind::Ipv6;
                // route(four);
                // route(six);
                // route(IpAddrKind::Ipv4);

                // 枚举类型可以作为struct内的字段类型
                    // let home = IpAddr {
                    //     kind: IpAddrKind::Ipv4,
                    //     address: String::from("127.0.0.1"),
                    // };
                    // let loopback = IpAddr {
                    //     kind: IpAddrKind::Ipv6,
                    //     address: String::from("::1"),
                    // };
                // 将数据附加到枚举的变体中
                    // let home = IpAddrKind::V4(127, 0, 0, 1);
                    // let loopback = IpAddrKind::V6(String::from("::1"));
                    // let q = Message::Quit;
                    // let m = Message::Move { x: 12, y: 13 };  // 匿名结构体
                    // let w = Message::Write(String::from("Hello!"));
                    // let c = Message::ChangeColor(0, 255, 255);
                // 给枚举定义方法
                    // m.call();
                    // w.call();
                    // c.call();
            // Option 枚举
                // 定义于标准库中，在Prelude（预导入模块）中，描述了，某个值可能存在（某种类型）或不存在的情况
                // rust没有null，所以rust中类似null概念的枚举 ： Option<T>
                    // let some_number = Some(5);
                    // let some_string = Some("A string");
                    // let absent_number: Option<i32> = None;
            // match 控制流运算符
                // 允许一个值与一系列模式进行匹配，并执行匹配的模式对应的代码
                // 模式可以是字面值、变量名、通配符...
                    // let c = Coin::Penny;
                    // value_in_coin(c);
            // 绑定值的模式
                // 匹配的分支可以绑定到被匹配对象的部分值，因此可以从enum变体中提取值
                    // let c = Coin::Quarter(UsState::Alaska);
                    // println!("{}", value_in_coin(c));
                    // let c2 = Coin::Quarter(UsState::Alabama);
                    // println!("{}", value_in_coin(c2));
            // Option<T>
                // let five = Some(5);
                // let six = plus_one(five);
                // let none = plus_one(None);
                // println!("{}", six.unwrap_or(-1));  // unwrap_or 此方法是获取当前枚举的值，如果有值就打印原本的值，如果没有就打印默认值-1
                // println!("{}", none.unwrap_or(-1))
            // match匹配必须穷举所有的可能，如果有很多没必要穷举的，可以使用_通配符来替代其余没列出来的值
                // let v = 10;
                // match v {
                //     1 => println!("one"),
                //     5 => println!("five"),
                //     8 => println!("eight"),
                //     10 => println!("ten"),
                //     _ => println!("nothing"),
                // }
            // if let:  
                // 处理直挂您一种匹配而忽略其他匹配的情况,但是，使用了if let 就放弃了穷举，可以把if let看作是match的语法糖
                    // let v = Some(3u8);
                    // println!("{:?}", &v);
                    // if let Some(3) = v {
                    //     println!("three")
                    // }
                // 也可以搭配else使用
                    // let vv = Some(76u8);
                    // if let Some(76) = vv {
                    //     println!("数字为76")
                    // } else {
                    //     println!("others")
                    // }
        /* Rust的代码组织 */
            // 代码组织主要包括
                // 那些细节是可以暴露，那些细节是私有的
                // 作用域内哪些名称有效等
            // 模块系统（等级依次往下）
                // Package(包)最顶层：Cargo特性，构建，测试，共享crate
                // Crate（单元包）：一个模块树，它可产生一个library（库）或可执行文件
                // Module（模块）：使用时要用use：关键字，让你控制代码的组织，作用域，私有路径
                // Path（路径）： 为struct，function或module等项命名的方式
            // Crate类型：
                // binary：二进制
                // library：库
            // Crate Root（根）：
                // 是源代码文件
                // Rust编译器从这里开始，组成你的Crate的根Module
            // Cargo的惯例（这两个都是入口文件）
                // src/main.rs 二进制的crate
                    //  binary crate 的crate root
                    // crate 名与package名相同
                // src/lib.rs   库crate
                    // package包含一个library crate
                    // library crate 的crate root
                    // crate名与package名相同
                // Cargo会把crate root 文件交给rustc 来构建library或binary
                
            // Package
                // 包含一个Cargo.toml,它描述了如何构建这些Crates
                // 只能包含0-1个library crate
                // 可以包含任意数量的binary crate
                // 但必须至少包含一个crate（library 或 binary）
                // 一个Package可以同时包含src/main.rs 和 src/lib.rs
                    // 一个binary crate，一个library crate
                    // 名称与Package名相同
                // 一个Package可以有多个binary crate
                    // 文件放在src/bin
                    // 每个文件是单独的binary crate
            // Module
                // 在一个crate内，将代码进行分组
                // 增加可读性，易于复用
                // 控制项目（item）的私有性，public， private
            // 建立Module
                // mod 关键字
                // 可嵌套
                // 可包含其他项（struct， enum，常量， trait，函数等）的定义
            // 路径（Path）
                // 为了在Rust的模块中找到某个条目，需要使用路径
                // 路径的两种形式
                    // 绝对路径: 从crate root开始，使用crate名或字面值crate
                    // 相对路径：从当前模块开始，使用self（本身），super（上一级）或当前模块的标识符
                // 路径至少由一个标识符组成，标识符之间使用::
            // 私有边界（private boundary）
                // 模块不仅可以组织代码，还可以定义私有边界
                // 如果想把函数或struct等设为私有，可以将它放在某个模块中
                // Rust中所有的条目（函数，方法，struct，enum，模块，常量）默认是私有的
                // 父级模块无法访问子模块中的私有条目
                // 子模块里可以使用所有祖先模块中的条目
            // pub 关键字
                // 使用pub关键字就可以把私有的变为公共的
                // pub struct
                    // pub放在struct前，struct是公共的
                    // 但是struct的字段值默认是私有的
                    // struct的字段需要单独设置pub来变成共有的
                // pub enum
                    // enum是公共的，enum的变体也默认为公共的
            // super关键字
                // 用来访问父级模块路径中的内容，类似文件系统中的
            // use关键字
                // 可以使用use关键字将路径导入到作用域内，但是仍然遵循私有性规则，也就是说，只有公共的可以用
                // 函数：惯用将函数的父级模块引入作用域（指定到父级）
                // struct, enum, 其他：指定完整路径（指定到本身）但是如果不同包内的struct名称相同，就要引用到父级
                    // use std::collections::HashMap;
                    // let mut map = HashMap::new();
                    // map.insert(1, "v");
                    // println!("{:?}", map)
                // 使用use将路径（名称）导入导作用域内后，该名称在此作用域内事私有的
            // 使用外部包（package）
                // Cargo.toml 添加依赖的包（package），通过https://crate.io/下载包
                // 用use将特定条目引入作用域
                    // use rand::Rng; 
                // 使用嵌套路径清理大量的use语句
                    // 如果使用同一个包或模块下的多个条目（例子）
                    // 可使用嵌套路径在同一行内将上述条目进行引入
                        // 路径相同的部分::{路径差异的部分}
                            // use std::{cmp::Ordering, io};  ==  use std::cmp::Ordering; use std::io;
                        // 如果两个use路径之一是另一个的子路径，使用self
                            // use std::io::{self,Write}  == use std::io; use std::io::Write;
                // 通配符*
                    // 可以使用*把路径中所有的公共条目都引入到作用域（谨慎使用）
                        // use std::collections::*;
                    // 测试：将所有被测试的代码引入tests模块
                    // 有时被用于预导入（prelude）模块
            // 将模块内容移动到其他文件
                // 模块定义时，如果模块名后边是;， 而不是代码块，Rust会从模块同名的文件中加载内容 
                // 模块树的结构不会发生变化
                // 随着模块逐渐变大，该技术让你可以把模块的内容移动到其他文件中
        /* 常用的集合,都是存放在heap中的，所以大小可以是不固定的 */  
            // Vector， Vec<T>,由标准库提供，可存储多个值，只能存储相同类型的数据，值在内存中连续存放
                // let ve: Vec<i32> = Vec::new();
                // println!("{:?}", ve)
            // 通常使用初始值创建Vec<T>，使用vec!宏
                // let ve1 = vec![1,2,3];
                // println!("{:?}", ve1)
            // 与任何其他struct一样，当Vector离开作用域后，它包括里面所有的元素都会被清理掉
            // 获取vector元素有两种方法，一种按照索引取值，一种用get方法，需要注意，如果用索引，一旦索引超出vec的位置，会报错，get则会返回None
                // let v_list = vec![4,5,6,1,9,10,2,3,7,8];
                // let night = &v_list[7];
                // println!("列表第8位的数字是: {}", night);
                // println!("用get方法取第三位数字: {:?}", v_list.get(2));
                // match v_list.get(100) {
                //     Some(third) => println!("用get方法取第5位数字:{}", third),
                //     None => println!("没有取到当前下标的值"),
                // }
            // Vector的所有权和借用规则
                // 不能在同一作用域内同时拥有可变和不可变引用
                    // let mut v_list1 = vec![1,2,3,4,5,6];
                    // let first = &v_list1[0];
                    // // v_list1.push(100);  // 这里就会报错，因为同一个作用域有一个可变和一个不可变
                    // println!("当前是不可变引用，值为{}", first);  // 这里使用了first，所以才可以继续使用可变引用，如果没有使用，就push，则会报错
                    // v_list1.push(80);
                    // println!("当前是可变引用：值为{:?}", v_list1)
                // 遍历Vector的值
                    // let v_list3 = vec![1,3,5,6,34,5,6,2,34344,53];
                    // for i in &v_list3 {
                    //     println!("{}", i);
                    // }

                    // let mut v_list3 = vec![1,3,5,6,34,5,6,2,34344,53];
                    // for i in &mut v_list3 {
                    //     // *i 是解引用
                    //     *i *= 10;
                    // }
                    // println!("{:?}", v_list3)
            // 使用enum在Vector中来存储多种数据类型
                // Enum的变体可以附件不同类型的数据
                // Enum的变体定义在同一个enum类型下
                    // let row = vec![
                    //     SpreadsheetCell::Int(34),
                    //     SpreadsheetCell::Text(String::from("blue!")),
                    //     SpreadsheetCell::Float(34.23),
                    // ];
                    // println!("{:?}", row)
            // String 类型
                // Rust的核心语言层面，只有一个字符串类型：字符串切片str（或&str）
                // 字符串切片：对存储在其他地方、utf-8编码的字符串的引用
                    //字符串字面值：存储在二进制文件中，也是字符串切片
                // String类型是标准库而不是核心语言 
                    // 可增长，可修改，可获得所有权，UTF-8编码
                // 通常说的字符串是指String和&str，因为在标准库里用的多，UTF-8编码
                // 其他类型字符串
                    // Rust的标准库还包含了很多其他的字符串类型，例如OsString、OsStr、CString、Cstr
                        // String后缀是可修改的，拥有所有权
                        // str后缀是借用所有权，是不可变的
                        // 可存储不同编码的文本或在内存中以不同的形式展现
                    // library crate针对存储字符串可提供更多的选项
                // 使用初始值来创建String
                    // to_string()方法，可用于实现了Display trait的类型，包括字符串字面值
                    // String::from()函数，从字面值创建String例子
                        // let data = "initial contents";
                        // let s = data.to_string();
                        // println!("{:?}", s);
                        // let ss = "init info".to_string();
                        // println!("{:?}", ss)
                // 更新String
                    // push_str:把一个字符串切片附加到String
                    // push：把单个字符附加到String
                    // + ：字符串拼接,使用了类似这个签名的方法 fn add(self, s: &str) -> String{...}
                    // format!宏：可以拼接字符串，并且不会获取任何字符串的所有权，并返回一个新的字符串
                        // let s1 = String::from("小泼猴！");
                        // let s2 = String::from("你要去爬树吗？");
                        // // let s3 = s1 + &s2;
                        // let s3 = String::from("别掉沟里去了！");
                        // // let s4 = s1 + "-" + &s2 + "-" + &s3;
                        // let s4 = format!("{}-{}-{}", s1, s2, s3);
                        // println!("{}", s4)
                // String按左右进行访问
                    // Rust不支持String按照索引进行访问，因为内部存放的是u8,也就是字节类型的Byte
                    // Rust有三种看待字符串的方式
                        // 字节（Bytes）
                        // 标量值（Scalar Values）
                        // 字形簇（Grapheme Clusters）：最接近“字母”
                            // let s1 = "小泼猴!";
                            // // for b in s1.bytes() { 
                            // //     println!("{}", b)
                            // // }
                            // for b in s1.chars() {
                            //     println!("{}", b)
                            // }
                //切割String
                    // 可以使用[]和一个范围来创建字符串切片
                        // 必须谨慎使用
                        // 如果切割时跨越了字符便捷，程序就会panic
                            // (b1,b2),(b3,b4),(b5,b6),(b7,b8) 如果切到了(b1,b2),(b3就会报错
        /* HashMap */
            // HashMap<K,V>: 键值对的形式存储数据，一个键（key）对应一个值（Value）
            // Hash函数：决定如何在内存中存放K和V
            // 适用场景：通过K（任何类型）来寻找数据，而不是通过索引
                // use std::collections::HashMap;
                // // let mut scores = HashMap::new();
                // // scores.insert(String::from("Blue"), 3);
                // // scores.insert(4, 5);
                // let mut scores = HashMap::new();
                // scores.insert(String::from("Blue"), 3);
                // scores.insert(4.to_string(), 5);
                // println!("{:?}", scores);
            // 由于HashMap用的比较少，不在预导入（Prelude）中
            // 标准库对其支持的比较少，没有内置的宏来创建HashMap
            // 数据存储在heap上
            // 一个HashMap中所有的K必须是同一种类型，所有的V必须是同一种类型
            // 另一种创建HashMap的方式：collect方法
                // 在元素类型为Tuple（元组）的Vector上使用collect方法，可以组件一个HashMap
                    // 要求Tuple有两个值，一个作为K，另一个作为V
                    // collect方法可以把数据整合成很多种集合类型，包括HashMap，返回值需要显示指明类型
                    // let teams = vec![String::from("Blue"), String::from("Yellow")];
                    // let initial_scores = vec![10, 50];
                    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
                    // println!("{:?}", scores);
            // HashMap的所有权
                // 对于实现了Copy trait的类型（例如i32），值会被复制到HashMap中
                // 对于拥有所有权的值（例如String），值会被移动，所有权会转移给HashMap
                // 如果将值的引用插入HashMap，值本身不会移动
                    // 在HashMap有效期间，被引用的值必须保持有效
            // 访问HashMap中的值
                // get：返回一个Option<&V>
                    // let mut scores = HashMap::new();
                    // scores.insert(String::from("Blue"), 10);
                    // scores.insert(String::from("Yellow"), 50);
                    // // let team_name = String::from("Blue");
                    // // let score = scores.get(&team_name);
                    // // match score {
                    // //     Some(s) => println!("{:?}", s),
                    // //     None => println!("None"),
                    // // };
                    // //  遍历HashMap
                    // for (key, value) in &scores {
                    //     println!("{}:{}", key, value);
                    // }
            // 更新HashMap中的值
                // HashMap大小可变
                // 每个Key只能有一个值
                // 更新HashMap中的数据
                    // K已存在，对应一个V
                        // 替换现有的V
                            // let mut scores = HashMap::new(); 
                            // scores.insert(String::from("Blue"), 10);
                            // scores.insert(String::from("Blue"), 25);
                            // println!("{:?}", scores);
                        // 保留现有的V，忽略新的V
                        // 合并现有的V和新的V
                            // let text = "hello world wonderful world";
                            // let mut map = HashMap::new();
                            // // 使用 split_whitespace() 方法拆分字符串，并迭代输出每个单词
                            // for word in text.split_whitespace() {
                            //     let count = map.entry(word).or_insert(0);
                            //     *count += 1;
                            // }
                            // println!("{:?}", map);  // {"world": 2, "hello": 1, "wonderful": 1}
                    // K不存在，插入新V，添加一对K，V,使用entry方法,Entry的方法or_insert,如果k存在，返回赌赢的v的一个可变引用，k不存在，插入新V
                        // let mut scores = HashMap::new();
                        // scores.insert(String::from("Blue"), 33);
                        // scores.entry(String::from("Blue")).or_insert(50);  
                        // scores.entry(String::from("Yellow")).or_insert(50);
                        // println!("{:?}", scores);  // {"Yellow": 50, "Blue": 33}
        /* 错误处理 */
            // Rust的可靠性：错误处理
                // 大部分情况下：在编译时提示错误，并处理
            // 错误的分类：
                // 可恢复的，例如文件未找到，可再次尝试
                // 不可恢复的，例如内存溢出，程序崩溃
            // Rust没有类似异常的机制
                // 1. panic! ：程序会打印一个错误信息，展开（unwind）、清理调用栈（stack），退出程序
                // 2. Result<T, E> 
                // 3. Option<T>
            // 为应对panic，展开或中止（abort）调用栈
                // 默认情况下，当panic发生
                    // 1.程序展开调用栈（工作量大）： Rust沿着调用栈往回走，清理每个遇到的函数中的数据
                    // 2.或立即中止调用栈：不进行清理，直接停止程序，内存需要Os（操作系统） 清理
                // 想让二进制文件更小，把设置从“展开”改为“中止”
                    // 在Cargo.toml中添加：
                    // [profile.release]
                    // panic = 'abort'
            // Result枚举
                // Result<T, E>枚举类型 
                    // T：操作成功情况下，OK变体里返回数据的类型
                    // E：操作失败情况下，Err变体里返回错误的类型
                // 处理Result的一种方式：match表达式
                    // use std::fs::File;
                    // let f = File::open("hello.txt");
                    // // println!("{:?}", f)
                    // let f1 = match f {
                    //     Ok(file) => file,
                    //     Err(error) => panic!("Problem opening the file: {:?}", error),
                    // };
                    // println!("{:?}", f1)
                // 对于多个match，还有一种方式：闭包（closure）Result<T, E>有很多方法
                    // 接收闭包作为参数，是match实现，使用这些方法让代码更简洁
                        // use std::io::ErrorKind;
                        // let f = File::open("hello.txt").unwrap_or_else(|error| {
                        //     if error.kind() == ErrorKind::NotFound {
                        //         File::create("hello.txt").unwrap_or_else(|error| {
                        //             panic!("Problem creating the file: {:?}", error);
                        //         })
                        //     } else {
                        //         panic!("Problem opening the file: {:?}", error);
                        //     }
                        // });
                        // println!("{:?}", f);
                // unwrap()和expect()方法
                    // unwrap()：如果Result是Ok，返回Ok里的值，如果Result是Err，调用panic!
                        // let f = File::open("hello.txt").unwrap();
                    // expect()：如果Result是Ok，返回Ok里的值，如果Result是Err，调用panic!，并打印自定义的错误信息
                        // let f = File::open("hello.txt").expect("打开文件失败 hello.txt");
                        // println!("{:?}", f);
                // 传播错误：在函数处处理错误，将错误返回给调用者
                    // let result = read_usernam_from_file();
                    // println!("{:?}", result)
                // ？运算符
                    // 如果Result是Ok，返回Ok里的值，然后继续执行程序
                    // 如果Result是Err，Err是整个函数的返回值，就像使用了return
                        // let result = read_usernam_from_file();
                        // println!("{:?}", result)
                // ？与 from 函数
                    // Trait std::convert::From 上的 from 函数：
                // 用于错误之间的转换
                    // 被？所应用的错误，会隐式的被 from 函数处理
                // 当？调用 from 函数时：
                    // 它所接收的错误类型会被转化为当前函数返回类型所定义的错误类型．用于：针对不同错误原因，返回同一种错误类型
                    // 只要每个错误类型实现了转换为所返回的错误类型的 from 函数
                // ?运算符与main函数
                    // main函数返回类型是:()
                    // mian函数返回类型也可以是:Result<T, E>
                    // Box<dyn Error>是trait对象：简单理解“任何可能的错误对象”
                        // fn main() -> Result<(), Box<dyn Error>> {
                        //     let f = File::open("hello.txt")?;
                        //     Ok(())
                        // }
            // 什么时候用panic
                // 在定义一个可能失败的函数时，优先考虑返回Result，否则就panic
            // 错误处理的指导性建议
                // 当代码最终可能处于损坏状态时，最好使用 panic!
                    // 损坏状态（ Bad state )：某些假设、保证、约定或不可变性被打破
                    // 例如非法的值、矛盾的值或空缺的值被传入代码
                // 以及下列中的一条：
                    // 这种损坏状态并不是预期能够偶尔发生的事情。
                    // 在此之后，您的代码如果处于这种损坏状态就无法运行。
                    // 在您使用的类型中没有一个好的方法来将这些信息（处于损坏状态）进行编码
            // 场景建议
                // 调用你的代码，传入无意义的参数值： panic!
                // 调用外部不可控代码，返回非法状态，你无法修复： panic! 如果失败是可预期的： Result 
                // 当你的代码对值进行操作，首先应该验证这些值： panic!
            // 为验证创建自定义类型
                // 创建新的类型，把验证逻辑放在构造实例函数里
                    // loop {
                    //     let guess = "321";
                    //     // 这里只是校验的是否是数字格式的字符串
                    //     let guess_num: i32= match guess.trim().parse() {
                    //         Ok(num) => num,
                    //         Err(_) => continue,
                    //     };
                    //     // 这里限制的是数字是否在1~100之间
                    //     let guess_struct = Guess::new(guess_num);
                    //     println!("你输入的数字是 {}", guess_struct.value());
                    // }
        /* 代码复用，泛型，Trait，生命周期 */
            // 代码复用
                // let num_list = vec![314,56,33,44,5,122,3443];
                // let largest = largest_fn(&num_list);
                // println!("largest: {}", largest)
            // 泛型
                // 提高代码复用能力，处理重复代码的问题
                // 泛型是具体类型或其他属性的抽象代替，编写的代码不是最终的代码，而是一种模板，里面有一些“占位符”，编译器在编译时将“占位符”替换为具体的类型
                    // pub fn largest<T>(list: &[T]) -> T {...}
                // 在struct定义泛型
                    // let integer = Point { x: 5, y: 10 };
                    // let float = Point { x: 5.0, y: 10.0 };
                    // println!("integer: {:?}, float: {:?}", integer, float)
                // 可以只用多个泛型的类型参数，但是太多的类型参数，代码就需要充足为多个更小的单元
                // 在Enum定义泛型
                        // enum Option<T> {
                        //     Some(T),
                        //     None,
                        // }
                        // enum Result<T, E> {
                        //     Ok(T),
                        //     Err(E),
                        // }
                // struct里的泛型类型参数可以和方法的泛型类型参数不同
                    // let p1 = Point { x: 5, y: 10.4 };
                    // let p2 = Point {x: "hello", y:"connect"};
                    // let p3 = p1.mixup(p2);
                    // println!("p3: {:?}", p3);
            // Trait（特性，特征）
                //  某种类型具有哪些并且可以与其他类型共享的功能，一个抽象的定义共享行为
                // Trait bounds(约束)：泛型类型参数指定为实现了特征的类型
                // Trait与其他语言的接口(interface)类似，但是有些区别
                // 定义一个 Trait 
                    // Trait 的定义：把方法签名放在一起，来定义实现某种目的所必需的一组行为。
                    // 关键字： trait 
                    // 只有方法签名，没有具体实现
                    // trait 可以有多个方法：每个方法签名占一行，以；结尾-实现该 trait 的类型必须提供具体的方法实现
                // 在类型上实现Trait
                    // 相同点：与为类型实现方法类似
                    // 不同点：impl Xxxx for Tweet {...}, 在impl的块里，需要对Trait里的方法签名进行实现
                        // use rut::Tweet;
                        // use rut::Summary;
                        // let tweet = Tweet {
                        //     username: String::from("小泼猴"),
                        //     content: String::from("小泼猴的小秘密"),
                        //     reply: false,
                        //     retweet: false,
                        // };
                        // println!("推特信息: {}", tweet.summarize());
                // 实现 trait 的约束
                    // 可以在某个类型上实现某个 trait 的前提条件是：
                        // 这个类型或这个 trait 是在本地 crate 里定义的
                    // 无法为外部类型来实现外部的 trait :
                        // 这个限制是程序属性的一部分（也就是一致性）。
                        // 更具体地说是孤儿规则：之所以这样命名是因为父类型不存在。
                        // 此规则确保其他人的代码不能破坏您的代码，反之亦然。
                        // 如果没有这个规则，两个 crate 可以为同一类型实现同一个 trait , Rust 就不知道应该使用哪个实现了。
                // Trait 作为参数
                    // impl Trait 语法，适用于简单情况
                    // Trait bound语法：适用于复杂情况，impl Trait语法是Trait bound的语法糖
                    // 使用 + 指定多个Trait bound
                // 实现Trait作为返回类型
                    // impl Trait语法，只能返回确定的同一类型，如果返回可能不同类型，即便这个类型实现了这个Trait，也会报错
                // 使用 Trait Bound 有条件的实现方法
                    // 在使用泛型类型参数的 impl 块上使用 Trait bound ，我们可以有条件的为实现了特定 Trait 的类型来实现方法
                    // 也可以为实现了其它 Trait 的任意类型有条件的实现某个 Trait
            // 生命周期
                // Rust的每个引用都有自己的生命周期，引用保持有效的作用域
                // 大多数情况下，生命周期是隐式的，可被推断的
                // 当引用的生命周期可能以不同的方式互相关联时，手动指定生命周期参数
                    // let r;
                    // {
                    //     let x = 5;
                    //     r = &x;
                    // }
                    // println!("r: {}", r);
                // 生命周期标注
                    // let s1 = String::from("hello");
                    // let s2 = "小泼猴 ";
                    // let res = longest(s1.as_str(), s2);
                    // print!("{}", res)
                    // let novel = String::from("Call me Ishmael. Some years ago... We ? and I met a traveller from an island far far away.");
                    // let first_sentence = novel.split('.').next().expect("Could not find a '.'");
                    // let ii = ImportantExcerpt {
                    //     part: first_sentence,
                    //     excerpt: novel.split('?').next().expect("Could not find a '.'"),
                    // };
                    // println!("{:?}", ii.part);
                    // println!("{:?}", ii.excerpt)
                // 生命周期的省略
                    // 每个引用都有生命周期，需要为使用生命周期的函数或struct指定生命周期参数
                // 生命周期省略规则
                    // 在 Rust 引用分析中所编入的模式称为生命周期省略规则。﹣这些规则无需开发者来遵守
                    // 它们是一些特殊情况，由编译器来考虑
                    // 如果你的代码符合这些情况，那么就无需显式标注生命周期
                // 生命周期省略规则不会提供完整的推断：
                    // 如果应用规则后，引用的生命周期仍然模糊不清→编译错误﹣解决办法：添加生命周期标注，表明引用间的相互关系
                // 生命周期省略的三个规则
                    // 编译器使用3个规则在没有显式标注生命周期的情况下，来确定引用的生命周期-
                        // 规则1应用于输入生命周期
                        // 规则2、3应用于输出生命周期
                        // 如果编译器应用完3个规则之后，仍然有无法确定生命周期的引用→报错
                        // 这些规则适用于 fn 定义和 impl 块
                    // 规则1：每个引用类型的参数都有自己的生命周期
                    // 规则2：如果只有1个输入生命周期参数，那么该生命周期被赋给所有的输出生命周期参数
                    // 规则3：如果有多个输入生命周期参数，但其中一个是＆self 或＆mut self （是方法），那么 self 的生命周期会被赋给所有的输出生命周期参数
                // 方法定义中的生命周期标注
                    // 在 struct 上使用生命周期实现方法，语法和泛型参数的语法一样
                    // 在哪声明和使用生命周期参数，依赖于：
                        // 生命周期参数是否和字段、方法的参数或返回值有关
                    // struct 字段的生命周期名：
                        // 在 impl 后声明
                        // 在 struct 名后使用
                        // 这些生命周期是 struct 类型的一部分
                    // impl 块内的方法签名中：
                        // 引用必须绑定于 struct 字段引用的生命周期，或者引用是独立的也可以
                        // 生命周期省略规则经常使得方法中的生命周期标注不是必须的
                // 静态生命周期
                    // 'static 是一个特殊的生命周期：整个程序的持续时间
                        // 例如：所有的字符串字面值都拥有'static 生命周期
                            // let s :&'static str =" I have a static lifetime .";
                    // 为引用指定'static 生命周期前要三思：
                        // 是否需要引用在程序整个生命周期内都存活
        /* 断言 Assert */
            // assert！宏，来自标准库，用来确定某个状态是否为true
                // true -> 断言成功，false -> 断言失败，程序panic
                // assert! 宏不会返回断言的结果，而是在断言失败时触发 panic
                // larger_can_hold_smaller()
            // 使用 assert_eq! 宏 和 assert_ne! 宏测试相等性
                // 都来自标准库，判断两个参数是否相等或不等，实际上，他们使用的是== 和 != 运算符
                // 断言失败会自动打印出两个参数的值
                    // it_add_two()
            // 添加自定义的错误信息
                // assert_eq! 宏和 assert_ne! 宏都接受一个额外的参数，用来提供错误信息，前两个参数都是必填的，自定义消息作为第三个参数
                // assert!：第一个参数必填，自定义消息作为第二个参数
                // 自定义消息参数会被传递给format! 宏，用来格式化错误信息
                // greetings_contain_name()
        /* 命令行项目编写 */
            // 12.1 接收命令行参数
            // 12.2 读取文件
            // 12.3 重构：改进模块和错误处理
            // 12.4 使用TDD（测试驱动开发）开发库功能
            // 12.5 使用环境变量
            // 12.6 将错误信息写入标准错误，而不是标准输出
            // 项目在/Users/bariuta/RustProjects/minigrep中
        /* 迭代器（iterators）和闭包（closures）*/
            // 闭包：可以捕获其所在环境的匿名函数
                // 闭包：匿名函数， 保存为变量，作为参数，可以在一个地方创建闭包，然后在另一个上下文中调用闭包来完成运算，可从其定义的作用域捕获值
                // 闭包的类型推断
                    // 闭包不要求标注参数和返回值的类型
                    // 把闭包通常很短，只在狭小的上下文中工作，编译器通常能推断出类型
                    // 注意，闭包的定义最终只会为参数、返回值推断出唯一具体的类型
                        // let example_closure = |x| x;
                        // let s = example_closure(String::from("hello"));
                        // let n = example_closure(5);  // 这里就会报错，闭包的类型已经被推断出来了，所以不能再更改了
                // 对于会重复调用闭包的代码
                    // 1. 将闭包调用结果存储在变量中
                    // 2. 创建一个struct来保存闭包和执行结果，这个模式通常叫做记忆化（memoization）或延迟计算（lazy evaluation）
                        // struct的定义需要知道所有字段的类型，需要指明闭包的类型
                        // 每个闭包实例都有自己唯一的匿名类型，及时两个闭包签名完全一样
                        // 所以需要使用泛型和 Trait Bound 来指定闭包类型
                        // 所有闭包都至少实现了已下trait之一：
                            // FnOnce：闭包可以调用一次
                            // FnMut：闭包可以修改捕获的外部变量
                            // Fn：闭包可以不修改捕获的外部变量
                            // generate_workout(10, 3)
                        // 使用缓存器（Cacher）实现的限制
                // 闭包可以捕获环境
                    // 闭包可以捕获环境，即闭包可以访问其定义的作用域中的变量，但是会产生开销
                    // 捕获方式
                        // FnOnce：闭包可以调用一次，获得所有权
                        // FnMut：闭包可以修改捕获的外部变量， 可变借用
                        // Fn：闭包可以不修改捕获的外部变量， 不可变借用
                    // 创建闭包时，通过闭包对环境值的使用， Rust 推断出具体使用哪个 trait :
                        // 所有的闭包都实现了 FnOnce 
                        // 没有移动捕获变量的实现了 FnMut 
                        // 无需可变访问捕获变量的闭包实现了 Fn
                // move关键字
                    // 在参数列表使用move关键字，可以强制闭包取得它所使用的环境值的所有权
                    // 使用场景：档表传递给新线程以移动数据使其归新线程所有时，此技术最为有用
                        // let y = vec![1, 2, 3];
                        // let equal_to_y = move |z: Vec<i32>| z == y;
                        // println!("equal_to_x = {:?}", y);  // 报错 value borrowed here after move
            // 迭代器
                    // let v1 = vec![1, 2, 3];
                    // let v1_iter = v1.iter();
                    // for i in v1_iter {
                    //     println!("{:?}", i);
                    // }
                // Iterator trait 仅仅要求实现一个方法：next 
                    // 每次返回迭代器的一项
                    // 返回结果包裹在Some里
                    // 迭代结束，返回None 
                    // 可直接在迭代器上调用next方法
                // 几个迭代方法的区别
                    // iter方法：在不可变引用上创建迭代器
                    // iter_mut方法：在可变引用上创建迭代器
                    // into_iter方法：创建的迭代器会获得所有权并销毁原始值
                // 消耗迭代器的方法
                    // 在标准库中， Iterator trait 有一些带默认实现的方法
                    // 其中有一些方法会调用 next 方法
                        // 实现 Iterator trait 时必须实现 next 方法的原因之一
                    // 调用 next 的方法叫做"消耗型适配器"
                        // 因为调用它们会把迭代器消耗尽
                    // 例如： sum 方法（就会耗尽迭代器）
                        // 取得迭代器的所有权
                        // 通过反复调用 next ，遍历所有元素
                        // 每次迭代，把当前元素添加到一个总和里，迭代结束，返回总和
                            // iterator_run()
                // 产生其它迭代器的方法
                    // 定义在 Iterator trait 上的另外一些方法叫做"迭代器适配器"
                        // 把迭代器转换为不同种类的迭代器
                    // 可以通过链式调用使用多个迭代器适配器来执行复杂的操作，这种调用可读性较高。
                    // 例如： map 
                        // 接收一个闭包，闭包作用于每个元素
                        // 产生一个新的迭代器
                    // collect 方法：消耗型适配器，把结果收集到一个集合类型中。
                        // let v1 = vec![1,2,3,4,5];
                        // let v3: Vec<_> = v1.iter().map(|x| x * 2).collect();
                        // println!("{:?}", v3)
                // 使用闭包捕获环境
                    // filter方法，接收一个闭包，这个闭包在遍历迭代器的每个元素时，返回bool类型，如果返回true，则保留该元素，否则丢弃。
                        // let shoes = vec![
                        //     Shoe { size: 10, style: String::from("sneaker") },
                        //     Shoe { size: 13, style: String::from("sandal") },
                        //     Shoe { size: 10, style: String::from("boot") },
                        // ];
                        // let in_my_size = shoes_in_size(shoes, 10);
                        // println!("in_my_size: {:?}", in_my_size)
                // 使用Iterator trait来创建自定义迭代器
                    // 提供一个next方法的实现
                        // let mut counter = Counter::new();
                        // println!("{:?}", counter.next());
                        // println!("{:?}", counter.next());
                        // println!("{:?}", counter.next());
                        // println!("{:?}", counter.next());
                        // println!("{:?}", counter.next());
                        // println!("{:?}", counter.next());
                    // skip 从下标1开始
                        // let sum1: u32 = Counter::new().zip(Counter::new().skip(1))
                        //     .map(|(a, b)| a * b).filter(|x| x % 3 == 0)
                        //     .sum();
                        // println!("{}", sum1)
        /* Cargo */
            // release profile
                // 是预定义的
                // 可自定义：可使用不同的配置，对代码编译拥有更多的控制
                // 每个 profile 的配置都独立于其它的 profile 
            // Cargo 主要的两个 profile :
                // dev profile ：适用于开发， cargo build 
                // release profile ：适用于发布， cargo build - release
            // 自定义 profile 
                // 针对每个 profile , Cargo 都提供了默认的配置
                // 如果想自定义 xxxx profile 的配置：
                    // 可以在 Cargo . toml 里添加［ profile . xxxx ］区域，在里面覆盖默认配置的子集
                // 对于每个配置的默认值和完整选项，请参见：https://doc.rust-lang.org/cargo/.
            // 生成 HTML 文档的命令
            // cargo doc：  它会运行 rustdoc 工具（ Rust 安装包自带）
                // 把生成的 HTML 文档放在 target / doc 目录下. 
            // cargo doc - open : 构建当前 crate 的文档（也包含 crate 依赖项的文档）
                // 在浏览器打开文档
        /* 智能指针 */
            // 指针：一个变量在内存中包含的是一个地址（指向其他数据）
            // Rust中最常见的指针就是“引用”：使用&，借用它指向的值，没有其余开销，最常见的指针类型
            // 智能指针：智能指针的行为和指针相似，有额外的元数据和功能
            // 引用计数（reference counting）智能指针类型
                // 通过记录所有者的数量，使一份数据被多个所有者同事持有
                // 并在没有任何所有者时自动清理数据
            // 引用和智能指针的不同
                // 引用：只借用数据
                // 智能指针：拥有数据，并允许访问数据
            // 智能指针的例子
                // String 和 Vec < T >
                // 都拥有一片内存区域，且允许用户对其操作
                // 还拥有元数据（例如容量等）
                // 提供额外的功能或保障（ String 保障其数据是合法的 UTF -8编码）
            // 智能指针的实现
                // 智能指针通常使用struct实现，并且实现了Deref 和 Drop 这两个trait
                // Deref trait：实现 Deref trait 的类型，当通过 * 运算符访问其值时，会自动调用 Deref trait 的 deref 方法，允许智能指针struct的实例像引用一样使用
                // Drop trait：实现 Drop trait 的类型，当其被丢弃时，会自动调用 Drop trait 的 drop 方法，允许智能指针struct的实例在离开作用域时执行一些清理工作
            // 常见的智能指针
                // Box < T >：Box < T > 是一个智能指针，它允许在堆上分配数据，而不是在栈上。
                // Rc < T >：Rc < T > 是一个引用计数智能指针，允许在多个地方拥有对同一数据的引用。
                // Ref<T> 和 RefMut < T >， 通过RefCell < T >访问，在运行时而不是编译时，情执借用规则的类型
                // 此外，内部可变模式：不可变类型暴露出可修改其内部值的API
                // 循环引用：如何导致内存泄漏，以及如何防止其发生
            // 使用Box < T >来指向heap堆内存上的数据
                // 最简单的智能指针，允许你在heap上存储数据（而不是stack）
                // stack上指向heap数据的指针
                // 没有性能开销，没有额外功能
                // 实现了Deref trait 和 Drop trait
                // Box < T >的常用场景
                    // 在编译时，某类型的大小无法确定。但使用该类型时，上下文却需要知道它的确切大小。
                    // 当你有大量数据，想移交所有权，但需要确保在操作时数据不会被复制。
                    // 使用某个值时，你只关心它是否实现了特定的 trait ，而不关心它的具体类型。
                // 使用Box赋值能递归类型
                    // 在编译时，Rust需要知道一个类型所占的空间大小，而递归类型的大小无法再编译时确定，但Box类型的大小确定，在递归类型中使用Box就可解决上述问题
                    // 类似函数式语言中的Cons List
                        // Cons List 是来自 Lisp 语言的一种数据结构。
                            // Cons List 里每个成员由两个元素组成。
                                // 当前项的值
                                // 下一个元素
                            // Cons List 里最后一个成员只包含一个 Nil 值，没有下一个元素
                                // let list1 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
                                // println!("{:?}", list1)
                // 使用 Box 来获得确定大小的递归类型
                    // Box < T ＞是一个指针， Rust 知道它需要多少空间，因为：指针的大小不会基于它指向的数据的大小变化而变化。
                    // Box < T >:
                        // 只提供了"间接"存储和 heap 内存分配的功能,没有其它额外功能,没有性能开销
                        // 适用于需要"间接"存储的场景，例如 Cons List -实现了 Deref trait 和 Drop trait
            // Deref trait
                // 实现Deref trait 的类型，当通过 * 运算符访问其值时，会自动调用 Deref trait 的 deref 方法，允许智能指针struct的实例像引用一样使用，可以自定义解引用运算符*的行为
                // 解引用运算符
                    // let x = 3;
                    // // let y = &x;
                    // let y = Box::new(x);
                    // println!("{}", x == 3);
                    // println!("{}", *y == 3);
                // 自定义解引用运算符
                    // let x = 3;
                    // let y = my_box(x);
                    // println!("{}", x == 3);
                    // println!("{}", *y == 3);  // *y 相当于 *(y.deref())
                // 函数和方法的隐式解引用转化(Deref Coercion)
                    // 隐式解引用转化是为函数和方法提供的一种便捷特性
                    // 假设 T 实现了 Deref trait :
                        // Deref Coercion 可以把 T 的引用转化为 T 经过 Deref 操作后生成的引用
                    // 当把某类型的引用传递给函数或方法时，但它的类型与定义的参数类型不匹配：
                        // Deref Coercion 就会自动发生
                        // 编译器会对 deref 进行一系列调用，来把它转为所需的参数类型
                        // 在编译时完成，没有额外性能开销
                            // let m = MyBox(String::from("hello"));
                            // hello(&m); // 1. &m &MyBox<String> 实现了Deref trait ，所以 &m 可以转化为 &String. 2. &String 也实现了 Deref trait ，所以 &String 可以转化为 &str。 Rust会一直调用deref这个方法
                            // hello("dss");
                // 解引用与可变性
                    // 可使用DerefMut trait重载可变引用的 * 运算符
                    // 在类型和trait在下列三种情况发生时，Rust会执行Deref Coercion
                        // 当T: Deref<Targrt=U>,允许&T转化为&U
                        // 当T: DerefMut<Target=U>,允许&mut T转化为&mut U
                        // 当T: Deref<Target=U>,允许&mut T转化为&U
            // Drop trait
                // 实现 Drop Trait ，可以让我们自定义当值将要离开作用域时发生的动作。
                    // 例如：文件、网络资源释放等
                    // 任何类型都可以实现 Drop trait 
                    // Drop trait 只要求你实现 drop 方法：参数：对 self 的可变引用
                    // Drop trait 在预导入模块里（ prelude )
                        // let cc = CustomStruct {
                        //     data: String::from("汇源"),
                        // };
                        // drop(cc);
                        // let gg = CustomStruct {
                        //     data: String::from("果汁"),
                        // };
                // 使用 std :: mem :: drop 来提前 drop 值
                    // ·很难直接禁用自动的 drop 功能，也没必要
                    // - Drop trait 的目的就是进行自动的释放处理逻辑
                    // . Rust 不允许手动调用 Drop trait 的 drop 方法
                    // ．但可以调用标准库的 std :: mem :: drop 函数，来提前 drop 值
            // Rc<T> 
                // Rc<T> 是一个引用计数智能指针，允许多个 ownership,一个值会有多个所有者，也就是说，会有多个要素指向它
                // 为了支持多重所有权，Rc<T> reference counting（引用计数），会追踪所有到值的引用，当0个引用时就代表该值可以被清理掉
                // 使用场景
                    // 需要在heap上分配数据，这些数据被程序的多个部分读取（只读），但在编译时无法确定哪个部分最后使用完这些数据
                    // RC<T> 只能用于单线程场景
                    // Rc<T> 不在预导入模块中，所以需要手动导入
                    // Rc::clone(&a)函数：增加引用计数
                    // Rc::strong_count(&a)函数：获取引用计数
                    // Rc::weak_count(&a)函数：获取弱引用计数
                        // let aaa = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
                        // println!("aaa 强引用个数 = {}", Rc::strong_count(&aaa));
                        // let _bbb = Cons(3, Rc::clone(&aaa));
                        // println!("aaa 强引用个数 = {}", Rc::strong_count(&aaa));
                        // {
                        //     let _ccc = Cons(4, Rc::clone(&aaa));
                        //     println!("aaa 强引用个数 = {}", Rc::strong_count(&aaa));
                        // }
                        // println!("aaa 在离开作用域之后的强引用个数 = {}", Rc::strong_count(&aaa))
                    // Rc::clone(&a) 和 类型的clone()方法
                            // Rc::clone不会执行数据的深度拷贝操作，只会增加引用的计数
                            // 类型的clone()方法很多会执行数据的深度拷贝操作
                        // let aa = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
                        // aa.clone();  // 会增加引用计数 和 Rc::clone区别在，Rc::clone不会执行数据的深度拷贝操作，只会增加引用的计数
                        // let bb = Cons(3, Rc::clone(&aa));
                        // let cc = Cons(4, Rc::clone(&aa));
                    // Rc<T>通过不可变引用，是你可以在程序不同部分之间共享只读数据
            // RefCell<T> 和内部可变性
                // 内部可变性是Rust的设计模式之一
                // 它允许你在只持有不可变引用的前提下对数据进行修改，数据结构中使用了unsafe代码来绕过Rust正常的可变性和借用规则
                // 只能用于单线程的场景
                // Box<T> 和 RefCell<T> 的区别
                    // Box<T>：编译阶段强制代码遵守借用规则，否则就出现错误
                    // RefCell<T>：只会在运行时检查借用规则，否则触发panic
                // RefCell<T>的两个方法（安全接口）
                    // borrow() 和 borrow_mut() 都是返回一个Rc<T>的智能指针，但borrow()返回的是一个不可变的引用，borrow_mut()返回的是一个可变的引用
                    //  RefCell<T>会记录当前存在多少个活跃的Ref<T>和 RefMut<T>智能指针
                        // 每次调用borrow：不可变借用计数+1
                        // 任何一个Ref<T>的值离开作用域被释放时：不可变借用计数-1
                        // 每次调用borrow_mut：可变借用计数+1
                        // 任何一个RefMut<T>的值离开作用域被释放时：可变借用计数-1
                            // let value = Rc::new(RefCell::new(5));
                            // let aaaa = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
                            // let bbbb = Cons(Rc::new(RefCell::new(6)), Rc::clone(&aaaa));
                            // let cccc = Cons(Rc::new(RefCell::new(10)), Rc::clone(&aaaa));
                            // *value.borrow_mut() += 10;
                            // println!("aaaa after = {:?}", aaaa);
                            // println!("bbb after = {:?}", bbbb);
                            // println!("cccc after = {:?}", cccc); 
                    // 其他方法
                        // Cell<T>：通过复制来访问数据
                        // Mutex<T>：用于实现跨线程情况下的内部可变性模式
                // Rust循环引用可能会发生内存泄漏
                    // Rust的内存安全机制可以保证很难发生内存泄漏，但不是不可能
                    // 例如使用Rc<T>和RefCell<T>就可能创造出循环引用，从而发生内存泄漏，因为每个项的引用数量都不会变成0，值也就不会被处理
                        // let aa = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
                        // println!("aa的强引用个数 = {}", Rc::strong_count(&aa));  // 1
                        // println!("aa的下一个item = {:?}", aa.tail());  // Some(RefCell { value: Nil })
                        // let bb = Rc::new(Cons(10, RefCell::new(Rc::clone(&aa))));
                        // println!("aa的强引用个数 = {}", Rc::strong_count(&aa));  // 2
                        // println!("bb的强引用个数 = {}", Rc::strong_count(&bb));  // 1
                        // println!("bb的下一个item = {:?}", bb.tail());  // Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

                        // if let Some(link) =  aa.tail(){
                        //     // 把aa的第二个元素next指向bb
                        //     *link.borrow_mut() = Rc::clone(&bb);
                        // }
                        // println!("aa的强引用个数 = {}", Rc::strong_count(&aa)); // 2
                        // println!("bb的强引用个数 = {}", Rc::strong_count(&bb));  // 2
                // 防止循环引用，把Rc<T>替换为Weak<T>
                    // Rc::clone()为Rc<T>实例的strong_count+1，增加引用计数，Rc<T>实例只有在strong_count为0时候才会被清理
                    // Rc<T>实例通过调用Rc::downgrade()方法，可以创建一个Weak<T>实例(弱引用)，Weak<T>实例不会增加引用计数，返回的类型是Weak<T>(智能指针)
                    // 调用Rc::downgrade 会为 weak_count +1
                    // Rc<T>使用weak_coumt来追踪存在多少Weak<T>实例
                    // weak_count 不为0并不影响Rc<T>实例的清理
                // Strong vs Weak 
                    // Strong Reference （强引用）是关于如何分享 Rc < T ＞实例的所有权. Weak Reference （弱引用）并不表达上述意思
                // 使用 Weak Reference 并不会创建循环引用：
                    // 当 Strong Reference 数量为0的时候， Weak Reference 会自动断开.在使用 Weak < T ＞前，需保证它指向的值仍然存在：
                    // 在 Weak < T ＞实例上调用 upgrade 方法，返回 Option < Rc < T >>
                            // let leaf = Rc::new(Node{
                            //     value: 5,
                            //     parent: RefCell::new(Weak::new()),
                            //     children: RefCell::new(vec![])
                            // });
                            // let branch = Rc::new(Node{
                            //     value: 10,
                            //     parent: RefCell::new(Weak::new()),
                            //     children: RefCell::new(vec![Rc::clone(&leaf)])
                            // });
                            // // borrow() 获得不可变引用，upgrade() 获得可变引用-->把Weak<T>转换成Rc<T>
                            // println!("keaf parent = {:?}", leaf.parent.borrow().upgrade());
                            // //  把leaf的父节点指向branch，通过downgrade方法，把branch的Rc<Node>转换成Weak<Node>
                            // *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
                            // println!("keaf parent 现在 = {:?}", leaf.parent.borrow().upgrade());
                            // println!("{:?}", branch)
                    // 当强引用被丢弃时，弱引用即便有引用，也会被丢弃
                        // let leaf = Rc::new(Node{
                        //     value: 5,
                        //     parent: RefCell::new(Weak::new()),
                        //     children: RefCell::new(vec![])
                        // });
                        // println!("keaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
                        // {
                        //     let branch = Rc::new(Node{
                        //         value: 10,
                        //         parent: RefCell::new(Weak::new()),
                        //         children: RefCell::new(vec![Rc::clone(&leaf)])
                        //     });
                           
                        //     //  把leaf的父节点指向branch，通过downgrade方法，把branch的Rc<Node>转换成Weak<Node>
                        //     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
                        //     println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
                        //     println!("keaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
                        // }
                        // println!("keaf parent = {:?}", leaf.parent.borrow().upgrade());
                        // println!("keaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        /* 并发 */
            // councurrent: 程序的不同部分之间独立执行，互不干扰
            // parallel: 程序的不同部分同时执行
            // Rust中，并发编程通过线程来实现，线程是Rust中并发编程的基本单元
            // 注意，本文中的”并发“泛指 concurrent 和 parallel

            // 使用线程同时运行代码
                // 进程跟线程
                    // 大部分OS里，代码运行在进程(process)中，OS同时管理多个进程，在程序里，各独立部分可以同时运行，运行这些独立部分的就是线程（thresd）
                    // 多线程运行，提升性能，但是会增加复杂性，因为无法保障各线程的执行顺序
                // 多线程可导致的问题
                    // 竞争状态：现成以不一致的顺序访问数据或资源
                    // 死锁：两个现成彼此等待对方使用完所持有的资源，线程无法继续
                    // 只有在某些情况下发生的Bug，很难可靠的复现和修复
                // 实现线程的方式
                    // 通过调用OS的API来创建线程： 1:1模型--->需要较小的运行时
                    // 语言自己实现的现成（绿色线程）： M:N 模型--->需要较大的运行时
                    // Rust标准库仅提供1:1模型的线程
                    // Rust中，线程通过std::thread::spawn()函数创建，参数是一个闭包（在新线程里运行的代码）
                        // thread::spawn(|| {
                        //     for i in 1..10 {
                        //         println!("hi number {} from the spawned thread!", i);
                        //         // 暂停一毫秒
                        //         thread::sleep(Duration::from_millis(1));
                        //     }
                        // });

                        // for i in 1..5 {
                        //     println!("number {} from the main thread!", i);
                        //     thread::sleep(Duration::from_millis(1));
                        // }
                    // 通过join Handle，等待线程结束
                        // thread::spawn 函数的返回值类型是JoinHandle
                        // JoinHandle持有值的所有权，调用其Join方法，可以等待对应的其他线程的完成
                        // join方法：调用Handle的join方法会阻止当前运行线程的执行，直到对应的线程结束
                            // let t = thread::spawn(|| {
                            //     for i in 1..10 {
                            //         println!("hi number {} from the spawned thread!", i);
                            //         // 暂停一毫秒
                            //         thread::sleep(Duration::from_millis(1));
                            //     }
                            // });

                            // for i in 1..5 {
                            //     println!("number {} from the main thread!", i);
                            //     thread::sleep(Duration::from_millis(1));
                            // }
                            // // 等待子线程的结束，因为join会返回一个Result<T, E>，所以使用unwrap
                            // t.join().unwrap()
                    // 使用move闭包
                        // move闭包通常和thread::spawn一起使用，因为闭包会捕获变量的引用，而spawn需要闭包获取变量的所有权
                        // move闭包会强制闭包获取变量的所有权，而不是引用
                        // 创建线程时，会把值的所有权从一个线程转移到另一个线程
                            // let v = vec![1, 2, 3];
                            // let handle = thread::spawn(move || {
                            //     println!("Here's a vector: {:?}", v);
                            // });
                            // // 这里如果没有等待，那会出现上面的线程闭包函数还没来得及执行，v就失效了
                            // handle.join().unwrap();
            // 使用消息传递来跨线程传递数据
                // 一种很流行且能保证安全并发的计数就是：消息传递
                // 线程（或Actor）通过彼此发送消息（数据）来进行通信
                // Go语言的名言：不要用共享内存来通信，要用通信来共享内存
                // Rust：Channel（标准库提供）
                // Channel
                    // 包含发送端，接收端
                    // 调用发送端的方法，发送数据
                    // 接收端会检查和接收到达的数据
                    // 如果发送端、接收端中任意一段被丢弃了，那么Channel就”关闭“了
                    // 创建Channel
                        // 使用mpsc::channel()创建一个通道，返回一个元组，包含发送端和接收端
                        // mpsc:表示 multi-producer，single-consumer(多个生产者，一个消费者)，多个发送端，只有一个接收端
                            // let (tx, rx) = mpsc::channel();
                            // thread::spawn(move || {
                            //     let val = String::from("hi");
                            //     tx.send(val).unwrap();
                            // });
                            // // recv()一直阻塞直到接收到数据
                            // let received = rx.recv().unwrap();
                            // println!("Got: {}", received);
                        // 发送端send方法
                            // 参数：想要发送的数据
                            // 返回值：Result<T, E>：如果有问题（接收端已经被丢弃），就返回一个错误
                        // 接收端的方法：
                            // recv方法：
                                // 阻塞当前线程，知道Channel中有值被送来
                                // 一旦有值收到，就返回Result<T, E>
                                // 当发送端关闭，就会收到一个错误
                            // try_recv方法：
                                // 不会阻塞当前线程，立即返回Result<T, E>
                                // 如果Channel中没有值，就返回一个错误
                                // 如果有值，就返回一个Result<T, E>
                                // 通常会使用循环用来检查try_recv的结果
                    // Channel所有权转移
                        // 所有权在消息传递中非常重要，能帮你编写安全并发的代码
                            // let (tx, rx) = mpsc::channel();
                            // thread::spawn(move || {
                            //     let val = String::from("hi");
                            //     tx.send(val).unwrap();
                            //     println!("val sent {}", val);  // 这里报错，value borrowed here after move，表示val已经转移了所有权
                            // });
                            // // recv()一直阻塞直到接收到数据
                            // let received = rx.recv().unwrap();
                            // println!("Got: {}", received);
                    // 发送多个值，接受者等待
                        // let (tx, rx) = mpsc::channel();
                        // thread::spawn(move || {
                        //     let vals = vec![
                        //         String::from("hi"),
                        //         String::from("from"),
                        //         String::from("the"),
                        //     ];
                        //     for i in vals {
                        //         tx.send(i).unwrap();
                        //         thread::sleep(Duration::from_secs(2));
                        //     }
                        // });
                        // for received in rx {
                        //     println!("Got: {}", received);
                        // }
                    // 克隆多个发送者
                        // let (tx, rx) = mpsc::channel();
                        // let tx1 = mpsc::Sender::clone(&tx);  // xxx.clone()会增加counter+1，所以当tx counter==0,rx就知道已经结束了
                        // thread::spawn(move || {
                        //     let vals = vec![
                        //         String::from("1: hi"),
                        //         String::from("1: from"),
                        //         String::from("1: the"),
                        //         String::from("1: thread"),
                        //     ];

                        //     for i in vals {
                        //         tx.send(i).unwrap();
                        //         thread::sleep(Duration::from_secs(1));
                        //     }
                        // });
                        // thread::spawn(move || {
                        //     let vals = vec![
                        //         String::from("2: hi"),
                        //         String::from("2: from"),
                        //         String::from("2: the"),
                        //         String::from("2: thread"),
                        //     ];

                        //     for i in vals {
                        //         tx1.send(i).unwrap();
                        //         thread::sleep(Duration::from_secs(1));
                        //     }
                        // });
                        // for received in rx {
                        //     println!("Got: {}", received);
                        // }
            // 共享状态的并发
                // Go语言：不要用共享内存来通信，要用通信来共享内存
                // Rust支持通过共享状态来实现并发
                // Channel类似单所有权，一旦将值的所有权转移至Channel，就无法使用它了
                // 共享内存病啊类似多所有权：多个线程可以同时访问同一块内存
                // 使用Mutex来每次只允许一个线程来访问数据
                // 想要访问数据，线程必须首先获取互斥锁（lock），lock数据结构时mutex的一部分，它能跟踪谁对数据有独占访问权
                // mutex通常被描述为：通过锁定系统来保护它所持有的数据
                // 使用mutex的两条规则
                    // 在使用数据之前，必须尝试获取锁(lock)
                    // 使用完mutex所保护的数据，必须对数据进行解锁，以便其他线程可以获取锁
                // Mutex<T>：
                    // 通过Mutx::new(数据)来创建Mutex<T>
                    // Mutex<T>是一个智能指针，它包装了T，并允许我们通过Deref来获取T
                    // 访问数据前，通过lock方法来获取锁，会阻塞当前线程，lock可能会失败
                    // 返回的是MutexGuard<T>，它实现了Deref和 Drop
                        // let m = Mutex::new(5);
                        // {
                        //     let mut num = m.lock().unwrap();
                        //     *num = 6;
                        // }
                        // println!("m = {:?}", m);
                // 多线程共享Mutex,使用Arc<T>来进行原子引用计数，跟Rc相似，但是它可以用于并发情景，性能差一点
                    // let counter = Arc::new(Mutex::new(1));
                    // let mut handles = vec![];
                    // for _ in 1..10 {
                    //     let counter = Arc::clone(&counter);
                    //     let handle = thread::spawn(move || {
                    //         let mut num = counter.lock().unwrap();
                    //         *num += 1
                    //     });
                    //     handles.push(handle);
                    // }

                    // for h in handles {
                    //     h.join().unwrap();
                    // }

                    // println!("Result: {}", *counter.lock().unwrap());
                // Send 和 Sync trait
                    // Rust语言的并发特性较少，目前讲的并发都来自标准库而不是语言本身
                    // 无需局限于标准库的并发，可以自己实现并发
                    // 但在Rust语言中有两个并发概念：
                        // std::marker::Sync 和 std::marker::Send 这两个trait
                    // Send
                        // 允许线程间转移所有权
                        // 实现Send trait 的类型可在线程间转移所有权
                        // Rust中几乎所有的类型都实现了Send
                        // 但Rc<T>没有实现Send，它只用于单线程情景
                        // 任何完全由Send类型组成的类型也被标记为Send
                        // 撤了原始指针之外，几乎所有的基础类型都实现了Send
                    // Sync
                        // 实现Sync的类型可以安全的呗多个线程引用
                        // 也就是说，如果T是Sync，那么&T就是Send，引用可以被安全的送往另一个线程
                        // 基础类型都是Sync
                        // 完全由Sync类型组成的类型也是Sync
                        // 但Rc<T>不是Sync，RefCell<T>和Cell<T>家族都不实是Sync的
                        // Mutex<T>是Sync的
                    // 转注意：手动来实现Send和Sync是不安全的
        /* 面向对象 */
            // 特性
                // 封装：隐藏实现细节，只暴露接口，调用对象外部的代码无法直接访问对象内部的实现细节，唯一可以与对象进行交互的方法就是通过它公开的API
                    // Rust： pub 关键字
                // 继承：子类继承父类的方法，使对象可以沿用另外一个对象的数据和行为，且无需重复定义相关代码
                    // Rust：没有继承，代码复用默认trait方法进行代码共享
                // 多态：父类可以指向子类，子类可以指向父类
                    // Rust：泛型和trait约束 （限定参数化多态）
                // 抽象：父类定义接口，子类实现接口 
                // 封装、继承、多态、抽象是面向对象编程的三大特性
            // 为共有行为定义一个trait
                // Rust避免将struct或enum称为对象，因为它们与impl块是分开的
                // trait对象有些类似于其他语言中的对象，他们某种程度上组合了数据与行为
                // trait对象与传统对象不同的地方：无法为trait对象添加数据
                // trait对象被专门用于抽象某些共有行为，它没其他语言中的对象那么通用
                    // let screen = Screen {
                    //     components: vec![
                    //         Box::new(SelectBox {
                    //             width: 75,
                    //             height: 10,
                    //             options: vec![
                    //                 String::from("Yes"),
                    //                 String::from("Maybe"),
                    //                 String::from("No"),
                    //             ],
                    //         }),
                    //         Box::new(Button {
                    //             width: 50,
                    //             height: 10,
                    //             label: String::from("OK"),
                    //         }),
                    //     ]
                    // };
                    // screen.run();
            // Trait 对象执行的是动态派发
                // 将 trait 约束作用于泛型时， Rust 编译器会执行单态化：
                    // 编译器会为我们用来替换泛型类型参数的每一个具体类型生成对应函数和方法的非泛型实现。
                // 通过单态化生成的代码会执行静态派发（ static dispatch )，在编译过程中确定调用的具体方法
                // 动态派发（ dynamic dispatch ):
                    // 无法在编译过程中确定你调用的究竟是哪一种方法
                    // 编译器会产生额外的代码以便在运行时找出希望调用的方法
                // 使用 trait 对象，会执行动态派发：
                    // 产生运行时开销
                    // 阻止编译器内联方法代码，使得部分优化操作无法进行
            // Trait 对象必须保证对象安全
                // 只能把满足对象安全（object-safe）的trait转化为trait对象
                // Rust采用一系列规则来判定某个对象是否安全，只需要记住两条
                    // 方法返回类型不是Self
                    // 方法中不包含任何泛型参数
            // 实现面向对象的设计模式
                // 状态模式：是一种面向对象设计模式
                // 一个值拥有的内部状态由数个状态对象组成，当值需要改变状态时，会改变状态对象，而不是改变值本身
                // 使用状态模式意味着：
                    // 业务需求变化时，不需要修改持有状态的值的代码，或者使用这个值的代码
                    // 只需要更新状态对象内部的代码，以便改变其规则，或者增加一些新的状态对象
                        // let mut post = Post::new();
                        // post.add_text("I ate a salad for lunch today");
                        // let post1 = post.request_review();
                        // let post2 = post1.approve();
                        // println!("{}", post2.content())
        /* 模式匹配 */
            // 模式是 Rust 中的一种特殊语法，用于匹配复杂和简单类型的结构
            // 将模式与匹配表达式和其他构造结合使用，可以更好地控制程序的控制流.模式由以下元素（的一些组合）组成：
                // 字面值
                // 解构的数组、 enum 、 struct 和 tuple 
                // 变量
                // 通配符
                // 占位符
            // 想要使用模式，需要将其与某个值进行比较：如果模式匹配，就可以在代码中使用这个值的相应部分
            // 用到模式的地方
                // match 的Arm（分支）
                    // match表达式的要求：尽可能的包含所有的可能性
                    // _ 通配符：匹配任何值，但是不会绑定到变量，通常用于match的最后一个arm，或用于忽略某些值
                // if let 表达式
                    //  if let 表达式主要作为一种简短的方式来等价的代替只有一个匹配项的match
                    // if let 可选的可以拥有else，else if 或 else if let，但是if let 不会检查穷尽性
                        // let favorite_color: Option<&str> = None;
                        // let favorite_color: Option<&str> = Some("pink");
                        // let is_tuesday = false;
                        // let age: Result<u8, _> = "35".parse();
                        // if let Some(color) = favorite_color {
                        //     println!("使用你喜欢的 {} 作为背景色", color);
                        // } else if is_tuesday {
                        //     println!("今天是周二，所以今天是休息日");
                        // } else if let Ok(age) = age {
                        //     if age > 30 {
                        //         println!("你太老了，不能工作了");
                        //     } else {
                        //         println!("你太年轻了，继续努力吧");
                        //     }
                        // } else {
                        //     println!("使用蓝色作为背景色颜色");
                        // }
                // while let条件循环
                    // 只要模式继续满足匹配条件，while循环就一直运行 
                        // let mut stack = Vec::new();
                        // stack.push(1);
                        // stack.push(2);
                        // stack.push(3);
                        // while let Some(x) = stack.pop() {
                        //     // pop从最后面一个区域弹出一个元素
                        //     println!("{}", x);
                        // }
                // for 循环
                    // let v = vec![1, 2, 3];
                    // for i in &v {
                    //     println!("{}", i);
                    // }
                    // for (i, v) in v.iter().enumerate() {
                    //     println!("第{}个元素是{}", i, v);
                    // }
                // let 模式
                    // let a = "d";
                    // let (b,c,d) = (1,2,"str");
                    // println!("{},{},{}, {}",a,b,c,d);
                // 函数的参数也是模式
            // 模式的两种方式
                // 可辨驳的，可失败,对于某些可能得值，无法进行匹配模式
                    // if let Some(x) = a_value;
                // 无可辩驳的
                    // let x = 5;
                // if let 和 while let 接收可辨驳和无可辩驳的模式
            // 模式匹配的语法（全是很简单的例子，不在这里写了）
                // @ 绑定：可以创建一个变量，该变量可以在测试某个值上进行匹配时，同时保存该值
                    // let msg = Message::Hello { id: 5 };
                    // match msg {
                    //     Message::Hello {
                    //         // id 要在3-7之间的范围内
                    //         id: id_variable @ 3..=7,
                    //     } => {
                    //         println!("id 是 {}", id_variable);
                    //     }
                    //     Message::Hello { id: 10..=12 } => {
                    //         println!("找到id另一个");
                    //     }
                    //     Message::Hello { id } => {
                    //         println!("id 是 {}", id);
                    //     }
                    // }
        /* Rust高级使用 */
            // 不安全的Rust，unsafe
            // Unsafe Rust存在的原因
                // 静态分析是保守的，使用unsafe Rust表示，我知道自己在做什么，并承担相应的风险
                // 计算机硬件本身就是不安全的，Rust需要能够进行底层系统编程
            // unsafe 使用方式
                // 使用unsafe关键字来切换到unsafe Rust，开启一个块，里面放着unsafe代码
                // unsafe Rust里面可执行的四个动作：
                    // 解引用原始指针
                    // 调用unsafe函数或方法
                    // 访问或修改可变的静态变量
                    // 实现unsafe trait
                // 注意：
                    // unsafe并没有关闭借用检查或停用其他安全检查
                    // 任何内存安全相关的错误必须留在unsafe块里
                    // 尽可能隔离unsafe代码，最好将其封装在安全的抽象里，提供安全的API
                // 解引用原始指针
                    // 原始指针
                        // 可变的：* mut T 
                        // 不可变的：* const T 。意味着指针在解引用后不能直接对其进行赋值
                        // 注意：这里的＊不是解引用符号，它是类型名的一部分。
                    // 与引用不同，原始指针：
                        // 允许通过同时具有不可变和可变指针或多个指向同一位置的可变指针来忽略借用规则
                        // 无法保证能指向合理的内存
                        // 允许为 null 
                        // 不实现任何自动清理
                    // 放弃保证的安全，换取更好的性能／与其它语言或硬件接口的能力
                        // let mut num = 5;
                        // // r1, r2, r 都为原始指针
                        // let r1 = &num as *const i32;
                        // let r2 = &mut num as *mut i32;
                        // let address = 0x012345usize;
                        // unsafe {
                        //     println!("r1 is: {}", *r1);
                        //     println!("r2 is: {}", *r2)
                        // }
                        // let r = address as *const i32;
                        // unsafe {
                        //     println!("r is: {}", *r);  // 运行会报错
                        // }
                    // 为什么要使用原始指针
                        // 1.与C语言进行接口
                        // 2.构建借用检查器无法理解的安全抽象
                    // 调用 unsafe 函数或方法
                        // unsafe 函数或方法：在定义前加上了 unsafe 关键字
                        // 调用前需手动满足一些条件（主要靠看文档），因为 Rust 无法对这些条件进行验证
                        // 需要在 unsafe 块里进行调用
                            // unsafe {
                            //     dangerous();
                            // }

                            // let mut tl = vec![1,2,3,4,5,6,7,8,56];
                            // let r = &mut tl[..];
                            // let (a, b) = r.split_at_mut(3);
                            // println!("{:?}", a);
                            // println!("{:?}", b)
                    // 使用extern函数调用外部代码
                        // extern关键字：简化创建和使用外部函数接口（FFI：Foreign Function Interface），它允许一种编程语言定义函数，并让其他编程语言能调用这些函数
                        // 任何在extern中声明的函数都是不安全的
                            // unsafe {
                            //     println!("abs(-3) = {}", abs(-3))
                            // }
                        // 应用二进制接口（ ABI , Application Binary Interface )：定义函数在汇编层的调用方式.
                            // " C " ABI 是最常见的 ABI ，它遵循 C 语言的 ABI
                        // 从其它语言调用 Rust 函数
                            // 可以使用 extern 创建接口，其它语言通过它们可以调用 Rust 的函数．在 fn 前添加 extern 关键字，并指定 ABI 
                            // 还需添加#[ no _ mangle ]注解：避免 Rust 在编译时改变它的名称
                            // 这类的不需要使用unsafe
                    // 访问或修改一个可变静态变量
                        // 在Rust里，全局变量叫做静态（static）变量
                            // println!("{}", HELLO_WORLD);
                        // 静态变量
                            // 静态变量与常量类似
                            // 命名： SCREAMING _ SNAKE _ CASE 
                            // 必须标注类型
                            // 静态变量只能存储' static 生命周期的引用，无需显式标注
                        // 常量和不可变静态变量的区别
                            // 静态变量：有固定的内存地址，使用它的值总会访问同样的数据
                            // 常量：允许使用它们的时候对数据进行复制
                            // 静态变量：可以是可变的，访问和修改静态可变变量是不安全（ unsafe ）的
                                // add_to_count(3);
                                // // println!("The count is: {}", COUNTER);  // 报错 use of mutable static
                                // unsafe {
                                //     println!("The count is: {}", COUNTER);
                                // }
            // 高级 Trait  
                // 在 Trait 定义中使用关联类型来指定占位类型
                    // 关联类型（ associated type ）是 Trait 中的类型占位符，它可以用于 Trait 的方法签名中：
                    // 可以定义出包含某些类型的 Trait ，而在实现前无需知道这些类型是什么  
                // 默认泛型参数和运算符重载
                    // 可以在使用泛型参数时为泛型指定一个默认的具体类型。
                    // 语法：< PlaceholderType = ConcreteType >
                    // 这种技术常用于运算符重载（ operator overloading )
                    // Rust 不允许创建自己的运算符及重载任意的运算符
                    // 但可以通过实现 std :: ops 中列出的那些 trait 来重载一部分相应的运算符
                        // let pp = Point { x: 1, y: 0 } + Point { x: 2, y: 3 };
                        // println!("pp = {:?}", pp)
            // 高级类型
                // newtype 模式可以
                    // 用来静态的保证各种值之间不会混淆并表明值的单位
                    // 为类型的某些细节提供抽象能力
                    // 通过轻量级的封装来隐藏内部实现细节
                // 使用类型别名创建类型同义词
                    // . Rust 提供了类型别名的功能：
                    // ﹣为现有类型生产另外的名称（同义词）
                    // ﹣并不是一个独立的类型
                    // -使用 type 关键字
                    // ·主要用途：减少代码字符重复
                        // let x = 5;
                        // let y: Kilometers = 5;
                        // println!("x + y = {}", x + y);
                // Never 类型
                    // 有一个名为！的特殊类型：
                    // 它没有任何值，行话称为空类型（ empty type )
                    // 我们倾向于叫它 never 类型，因为它在不返回的函数中充当返回类型．不返回值的函数也被称作发散函数（ diverging function )
                    // never 类型的实例永远不存在．可以被强制的转化成其他任意类型
                        // let guess = "";
                        // loop {
                        //     let guess: i32 = match guess.trim().parse() {
                        //         Ok(num) => num,
                        //         // 这里的continue就会返回一个never，所以会被强制的转化成第一个分支返回的值类型
                        //         Err(_) => continue,
                        //     };
                        // }
                // 动态大小和 Sized Trait 
                    // Rust 需要在编译时确定为一个特定类型的值分配多少空间。
                    // 动态大小的类型（ Dynamically Sized Types , DST ）的概念：
                    // 编写代码时使用只有在运行时才能确定大小的值
                    // str 是动态大小的类型（注意不是＆ str )：只有运行时才能确定字符串的长度﹣下列代码无法正常工作：
                        // let s1: str =" Hello there !";
                        // let s2: str =" How ' s it going ?";
                    // &str里面存储的是str的地址和str的长度
                // Sized trait 
                    // 为了处理动态大小的类型， Rust 提供了一个 Sized trait 来确定一个类型的大小在
                    // 编译时是否已知
                        // 编译时可计算出大小的类型会自动实现这一 trait 
                        // Rust 还会为每一个泛型函数隐式的添加 Sized 约束
                            // fn gen<T>(t: T) {}  == fn gen<T: Sized>(t: T) {}
            // 高级函数和闭包
                // 函数指针
                    // 可以将函数传递给其他函数
                    // 函数在传递过程中会被强制转化成fn类型
                    // fn类型就是”函数指针（function pointer）“
                        // let answer = do_twice(add_one, 4);
                        // println!("The answer is: {}", answer)
                // 函数指针与闭包的不同
                    // fn 是一个类型，不是一个 trait 
                        // 可以直接指定 fn 为参数类型，不用声明一个以 Fn trait 为约束的泛型参数
                    // 函数指针实现了全部3种闭包 trait ( Fn , FnMut , FnOnce ):
                        // 总是可以把函数指针用作参数传递给一个接收闭包的函数
                        // 所以，倾向于搭配闭包 trait 的泛型来编写函数：可以同时接收闭包和普通函数
                        // 某些情景，只想接收 fn 而不接收闭包：
                            // 与外部不支持闭包的代码交互： C 函数
                                // let list_of_numbers = vec![1, 2, 3];
                                // let list_of_strings: Vec<String> = list_of_numbers
                                //     .iter()
                                //     .map(|i| i.to_string())
                                //     .collect();
                                // let list_of_numbers1 = vec![6, 7, 8];
                                // let list_of_strings1: Vec<String> = list_of_numbers1.iter().map(ToString::to_string).collect();
                                // println!("{:?}", list_of_strings1);
                                // println!("{:?}", list_of_strings);
                                // #[derive(Debug)]
                                // enum Status {
                                //     Value(i32),
                                //     Stop,
                                // }
                                // // 实例化Status枚举
                                // let v = Status::Value(3);
                                // println!("{:?}", v);
                                // let list_of_status: Vec<Status> = (0i32..20).map(Status::Value).collect();
                                // println!("{:?}", list_of_status)
                    // 返回闭包
                        // 闭包使用Trait进行表达，无法在函数中直接返回一个闭包，可以讲一个实现了该Trait的具体类型作为返回值Box<gyn Fn(i32) -> i32>
                            // fn return_c() -> Box<dyn Fn(i32) -> i32> {
                            //     Box::new(|x| x + 1)
                            // }
            // 宏 macro
                // 宏在 Rust 里指的是一组相关特性的集合称谓：
                    // 使用 macro_rules ！构建的声明宏（ declarative macro )
                    // 3种过程宏
                        // 自定义#[ derive ］宏，用于 struct 或 enum ，可以为其指定随 derive 属性添加的代码
                        // 类似属性的宏，在任何条目上添加自定义属性
                        // 类似函数的宏，看起来像函数调用，对其指定为参数的 token 进行操作 
                // 函数与宏的差别
                    // 本质上，宏是用来编写可以生成其它代码的代码（元编程， metaprogramming )函数在定义签名时，必须声明参数的个数和类型，宏可处理可变的参数
                    // 编译器会在解释代码前展开宏
                        // 宏的定义比函数复杂得多，难以阅读、理解、维护
                        // 在某个文件调用宏时，必须提前定义宏或将宏引入当前作用域；
                        // 函数可以在任何位置定义并在任何位置使用
                        // macro_rules! 即将弃用
                // 基于属性来生成代码的过程宏
                    // 这种形式更像函数（某种形式的过程）一些
                        // 接收并操作输入的 Rust 代码
                        // 生成另外一些 Rust 代码作为结果
                    // 三种过程宏：
                        // 自定义派生
                        // 属性宏
                        // 函数宏
                    // 创建过程宏时：
                        // 宏定义必须单独放在它们自己的包中，并使用特殊的包类型
                // 自定义derive宏
                    // 需求：（使用工作空间）
                        // 1.创建一个 hello _ macro 包，定义一个拥有关联函数 hello _ macro 的 HelloMacro trait 
                        // 2.我们提供一个能自动实现 trait 的过程宏
                        // 3.在它们的类型上标注＃[ derive ( HelloMacro )]，进而得到 hello _ macro 的默认实现
                // 类似属性的宏
                    // 属性宏与自定义 derive 宏类似﹣允许创建新的属性
                        // 但不是为 derive 属性生成代码
                    // 属性宏更加灵活：
                        // derive 只能用于 struct 和 enum 
                        // 属性宏可以用于任意条目，例如函数
                // 类似函数的宏
                    // 函数宏定义类似于函数调用的宏，但比普通函数更加灵活
                    // 函数宏可以接收 TokenStream 作为参数
                    // 与另外两种过程宏一样，在定义中使用 Rust 代码来操作 TokenStreamlet config = load_config();
    // // 从配置文件加载 `Config` 实例
    // let config = load_config();
    // // 构建 MySQL 连接 URL
    // let url = format!(
    //     "mysql://{}:{}@{}:{}/{}",
    //     config.database.user,
    //     config.database.password,
    //     config.database.host,
    //     config.database.port,
    //     config.database.name
    // );

    // // 创建连接池
    // let pool = Pool::new(OptsBuilder::from_opts(Opts::from_url(&url).unwrap())).unwrap();

    // // 获取连接并执行简单查询
    // let mut conn = pool.get_conn().expect("无法连接到数据库");

    // let result: Vec<String> = conn.query("SELECT 'Hello from MySQL!'").unwrap();
    // println!("{:?}", result);
                    
    ticket::run_ticket_sale();
}

// use proc_macro;
// // 过程宏
// #[some_attribute]
// pub fn some__name(input: TokenStream) -> TokenStream {
    
// }
// // 定义宏
// #[macro_export]  // 标注当前宏必须在引入包的作用域后才可以使用
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }

// fn add_one(x: i32) -> i32 {
//     x + 1
// }

// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }
// type Kilometers = i32;


// use std::ops::Add;
// struct Millimeters(i32);
// struct Meters(i32);

// impl Add<Meters> for Millimeters {
//     type Output = Millimeters;
//     fn add(self, other: Meters) -> Millimeters {
//         Millimeters (self.0 + (other.0 * 1000))
//     }
    
// }

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Add for Point {
//     type Output = Point;
//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// pub trait Iterator {
//     // Item就是关联类型
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
// static mut COUNTER: u32 = 0;

// fn add_to_count(inc: u32) {
//     unsafe {
//         COUNTER += inc;
//     }
// }

// static HELLO_WORLD:&str = "Hello, world!";
// #[no_mangle]
// pub extern "C" fn call_from_c() {
//     println!("当前函数编译完连接后就可以被C语言访问了")
// }

// extern "C" {
//     fn abs(input: i32) -> i32;
// }

// use std::slice;

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     assert!(mid <= len);
//     unsafe {
//         (
//             slice::from_raw_parts_mut(slice.as_mut_ptr(), mid),
//             slice::from_raw_parts_mut(slice.as_mut_ptr().add(mid), len - mid),
//         )
//     }
// }

// unsafe fn dangerous() {
//     println!("unsafe函数");
// }

// enum Message {
//     Hello { id: i32 },
// }
// use rut::{Draw, Screen, Button};

// struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>,
// }

// impl Draw for SelectBox {
//     fn draw(&self) {
//         println!("绘制一个选择框")
//     }
// }

// use std::sync::{Arc, Mutex};

// use std::sync::mpsc;
// #[derive(Debug)]
// struct Node {
//     value: i32,
//     parent: RefCell<Weak<Node>>,
//     children: RefCell<Vec<Rc<Node>>>,
// }

// use Li23::{Cons, Nil};
// #[derive(Debug)]
// enum Li23 {
//     Cons(i32, RefCell<Rc<Li23>>),
//     Nil,
// }

// impl Li23 {
//     fn tail(&self) -> Option<&RefCell<Rc<Li23>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// #[derive(Debug)]
// enum LiTwo {
//     Cons(Rc<RefCell<i32>>, Rc<LiTwo>),
//     Nil,
// }
// use LiTwo::{Cons, Nil};

// use Lii::{Cons, Nil};
// use std::rc::Rc;
// enum Lii {
//     Cons(i32, Rc<Lii>),
//     Nil,
// }

// enum Lii {
//     Cons(i32, Box<Lii>),
//     Nil,
// }

// struct CustomStruct {
//     data: String,
// }

// impl Drop for CustomStruct {
//     fn drop(&mut self) {
//         println!("{}", self.data);
//     }
// }


// use std::ops::Deref;
// struct MyBox<T>(T); //是一个元组类型

// impl<T> MyBox<T> {
//     fn new(a: T) -> MyBox<T>{
//         MyBox(a)
//     }
// }

// impl <T> Deref for MyBox<T> {
//     type Target = T;  // 定义了Deref trait 的 关联类型
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn hello(name: &str) {
//     println!("Hello, {}!", name)
// }

// 创建异步运行时
// az().await;
// use tokio::fs::File as AsyncFile;
// use tokio::io::AsyncReadExt;

// // 异步函数，读取文件流并传递给另一个函数
// async fn az() {
//     // 异步读取文件流
//     let mut file = AsyncFile::open("example.txt").await.unwrap();
//     let mut contents = Vec::new();
//     file.read_to_end(&mut contents).await.unwrap();
//     // 异步函数执行完毕后调用另一个函数并将文件流内容作为参数传递
//     // 将字节向量转换为字符串
//     let contents_string = String::from_utf8_lossy(&contents).to_string();
//     another_function(contents_string);
// }

// // 第二个函数，接收文件流内容作为参数
// fn another_function(contents: String) {
//     // 这里可以对文件流内容进行处理

//     println!("Received contents: {:?}", contents);
// }

// use std::thread;
// use std::time::Duration;
// use std::iter::Iterator;

// use crate::List::{Cons, Nil};
// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }


// struct Counter {
//     count: u32,
// }

// impl Counter {
//     fn new() -> Counter {
//         Counter { count: 0 }
//     }
// }

// // 创建自定义迭代器
// impl Iterator for Counter {
//     // 关联类型
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.count < 5 {
//             self.count += 1;
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }

// #[derive(Debug)]
// struct Shoe {
//     size: i32,
//     style: String,
// }

// fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
//     // into_iter 创建一个迭代器，迭代器会遍历Vec中的每个元素，并返回一个迭代器。
//     // collect 产生一个集合，这个集合是Vec类型。
//     shoes.into_iter()
//         .filter(|s| s.size == shoe_size)
//         .collect() 
// }

// fn iterator_run() {
//     let v1 = vec![1, 2, 3];
//     let v1_iter = v1.iter();
//     let total: i32 = v1_iter.sum();
//     assert_eq!(total, 6)
// }
// struct Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     calculation: T,
//     value: Option<u32>,
// }


// impl<T> Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             // 如果有值，直接返回，如果没有值，执行一次计算，并保存结果，而且fn返回v
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     } 
// }  

// fn generate_workout(intensity: u32, random_number: i32) {

//     // 定义一个闭包
//     let mut expensive_closure = Cacher::new(|num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     });

//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_closure.value(intensity));
//         println!("Next, do {} situps!", expensive_closure.value(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_closure.value(intensity));
//             }
//         }
// }

// pub fn greeting(name: &str) -> String {
//     format!("Hello {}!", name)
// }

// pub fn greetings_contain_name() {
//     let result = greeting("Carol");
//     assert!(result.contains("Carole"), "未获取到名字包含Carole的数据, :{}", result)
// }


// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// pub fn it_add_two() {
//     // assert_eq!(4, add_two(3))
//     assert_ne!(4, add_two(2))
// }

// fn longest_with_an_answer<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
// where
//     T: Display,
// {
//     println!("ann{}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// struct ImportantExcerpt<'a> {
//     part: &'a str,
//     excerpt: &'a str,
// }

// impl <'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         4
//     }

//     fn another_fun(&self, an: &str) -> &str {
//         // 因为第一个参数是self，所以将所有的入参以及返回参数的生命周期都绑定于self的生命周期
//         println!("执行another_fun: {}", an);
//         self.part
//     }
// }

// // 如果直接这样，会报错，因为x，y都不知道生命周期
// // fn longest(x: &str, y: &str) -> &str {
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// // 定义一个Trait
// pub trait Summary {
//     fn summarize(&self) -> String;
//     fn summarize1(&self) -> String; 
// }

// #[derive(Debug)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl <T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl <T> Point<T> {
//     // 这里是针对泛型T中实现Point的x和y方法
//     pub fn x(&self) -> &T {
//         &self.x
//     }
//     pub fn y(&self) -> &T {
//         &self.y
//     }
// } 

// impl Point<i32> {
//     // 这里仅仅是实现Point的方法
//     fn x1(&self) -> &i32 {
//         &self.x
//     }
    
// }

// fn largest_fn(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &num in list {
//         if num > largest {
//             largest = num;
//         }
//     }
//     largest
// }

// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("请传入1~100之间的数字, 你输入的是 {}.", value);
//         }
//         Guess { value }
//     }

//     pub fn value(&self) -> i32 {
//         self.value
//     }
// } 


// // 链式调用
// fn read_usernam_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     // 从文件中读取内容到s
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

// // ?运算符
// fn read_usernam_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     // 从文件中读取内容到s
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn read_usernam_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut f1 = match f {
//         Ok(file) => file,
//         Err(error) => return Err(error),
//     };
//     let mut s = String::new();
//     match f1.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(error) => Err(error),
//     }
    
// }

// #[derive(Debug)]
// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// // as 指定别名
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn f1() -> Result{}
// fn f2() -> IoResult{}

// use std::fmt;
// use std::io;
// // 因为Result名称相同，所以要引入父级
// fn f1() -> fmt::Result{}
// fn f2() -> io::Result{}



// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
    
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
    
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
    
// }

// fn value_in_coin(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         // 绑定值的模式匹配
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?} !", state);
//             25
//         },
//     }
// }

// enum Message {
//     Quit,
//     Move {x: u32, y: u32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
    
// }

// impl Message {
//     fn call(&self){
//         println!("调用 call 方法")
//     }
// }

// enum IpAddrKind {
//     // 不需要额外使用struct
//     // 每个变体可以拥有不同类型以及关联的数据量
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// enum IpAddrKind {
//     Ipv4,
//     Ipv6,
    
// }

// fn route(_ip_kind: IpAddrKind) {}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// #[derive(Debug)]  // 添加此列方便打印结构体
// struct User {
//     username: String,
//     email: String,
//     active: bool,
// }

// #[derive(Debug)] 
// struct Color (i32, i32, i32);
// #[derive(Debug)] 
// struct Point (i32, i32, i32);

// #[derive(Debug)] 
// struct Rectangle {
//     width: u32,
//     length:u32,  
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.length
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         // 判断当前长方形是否能包含另一个长方形
//         self.width > other.width && self.length > other.length
        
//     }

//     fn square(size: u32) -> Rectangle {
//         Rectangle {
//             width: size,
//             length: size,
//         }
//     }
    
// }
// // 运用assert！宏来判断
// fn larger_can_hold_smaller() {
//     let larger = Rectangle {
//         width: 8,
//         length: 7,
//     };
//     let smaller = Rectangle {
//         width: 5,
//         length: 1,
//     };
//     // assert! 宏不会返回断言的结果，而是在断言失败时触发 panic
//     assert!(larger.can_hold(&smaller));
// }

// fn area(rect: &Rectangle) -> u32{
//     // 计算长方形的面积，长 * 宽
//     rect.width * rect.length
// }

// fn area(dim:(u32,u32)) -> u32{
//     // 计算长方形的面积，长 * 宽
//     dim.0 * dim.1
// }

// fn area(width:u32, length:u32) -> u32{
//     // 计算长方形的面积，长 * 宽
//     width * length
// }

// fn first_world(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

// fn first_world(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn danling_info() ->&String {
//     let s = String::from("hello");
//     // 当函数结束时，s就不在当前的作用域，就会被销毁，但是引用被返回，那么这个引用的地址已经被分配给别的地方使用了，所以报错
//     &s
// }

// fn calculate_length(s: &mut String) -> usize {
//     // 引用,借用的如果是不可变变量，值则不能被修改
//     s.push_str("，添加的数据");
//     s.len()
// }

// fn calculate_length(s: String) -> (String, usize){
//     let length = s.len();
//     (s, length)
// }

// fn gives_own() -> String {
//     let some_str = String::from("函数内部");
//     some_str
// }

// fn takes_and_give_back(str_info: String) -> String {
//     str_info
// }

// fn take_own(some_str: String) {
//     println!("{}", some_str);
// }

// fn make_copy(some_num: i32) {
//     println!("{}", some_num);
// }

// fn another_fun(x: i32, y: i32) {  // 函数参数必须指定类型
//     println!("执行另一个函数: {} and {}", x, y)
    
// }

// fn five(x: i32) -> i32 {
//     x + 5
// }

