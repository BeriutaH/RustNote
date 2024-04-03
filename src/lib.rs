// use std::fmt::Debug;
// use std::cmp::PartialOrd;
// use std::fmt::Display;

// 实现Summary trait
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// // 新闻文章类型
// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// // 为类型实现Summary trait
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// // 推特类型
// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }
// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// // 表示只要实现了Summary trait的任意类型都可以作为参数
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// // 使用Trait bound语法
// pub fn notify<T: Summary>(item: T, item2: T) {
//     println!("Breaking news! {}", item.summarize());
// }

// // 指定多个Trait可以使用+
// pub fn notify<T: Summary + Display>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }

// 两个参数，一个需要实现Summary跟Display trait，一个需要实现Clone + Debug trait，使用where子句
// pub fn notify<T, U>(a: T, b: U) -> String
// where
//     T: Summary + Display,
//     U: Clone + Debug,
// {
//     format!("大新闻啊：{}", a.summarize())
// }

// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     // 表示，无论T是什么类型，都有new这个函数
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }
// impl<T: Display + PartialOrd> Pair<T> {
//     // 只有在T实现了Display和PartialOrd这两个trait的时候，才会有cmp_display这个方法
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // 因为当前函数跟front_of_house模块在同一个文件下，所以front_of_house不需要加pub关键字，但是如果在别的文件夹下，要想调用就必须要加上pub关键字
//     // 绝对路径
//     crate::front_of_house::hosting::add_to_waitlist();
//     // 相对路径
//     front_of_house::hosting::add_to_waitlist()
// ;
// }

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         // super关键字调用上层目录
//         super::serve_order();
//         // 从根目录中导入
//         crate::serve_order();

//     }
//     fn cook_order() {}
// }

// mod back_of_house {
//     pub struct BreakFast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
//     impl BreakFast {
//         // 关联函数
//         pub fn summer(toast: &str) -> BreakFast {
//             BreakFast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }

//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::BreakFast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//     // 因为是私有的，所以没有办法访问
//     meal.seasonal_fruit = String::from("value");
// }

// mod back_of_house {
//     pub enum Appetizer {
//         //  这里的枚举变体都为公共的
//         Soup,
//         Salad,
//     }
// }

// // use关键字使用
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn some_fn() {

//         }
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     // 访问私有函数就会报错
//     hosting::some_fn();

// }

// // pub use 关键字使用
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn some_fn() {

//         }
//     }
// }

// // // 使用pub use，可以使外部代码可以访问到hosting
// mod front_of_house;
// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     // 访问私有函数就会报错
//     hosting::some_fn();

// }
