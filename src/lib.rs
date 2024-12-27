// use std::fmt::Debug;
// use std::cmp::PartialOrd;
// use std::fmt::Display;

// pub struct Post {
//     content: String,
// }

// impl Post {
//     pub fn new() -> DraftPost {
//         DraftPost {
//             content: String::new(),
//         }
//     }

//     pub fn content(&self) -> &str {
//         &self.content
//     }
// }

// pub struct DraftPost {
//     content: String,
// }

// impl DraftPost {
//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }
//     pub fn request_review(self) -> PendingReview {
//         PendingReview {
//             content: self.content,
//         }
//     }
// }

// pub struct PendingReview {
//     content: String,
// }

// impl PendingReview {
//     pub fn approve(self) -> Post {
//         Post {
//             content: self.content,
//         }
//     }
// } 

// trait State {
//     // 请求审批
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     // 审批通过
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&self, _post: &'a Post) -> &'a str {
//         // 需要注意生命周期，post引用作为参数，但是返回的可能是post中某一部分的引用，所以返回值的生命周期跟post的生命周期是相关联的
//         ""
//     }
// }

// struct Draft {}

// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }

// struct PendingReview {}

// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }
// } 

// struct Published {}

// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }

// }

// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }

// impl Post {
//     pub fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }

//     pub fn add_text(&mut self, text: String) {
//         // 将文本内容附加到content字段里
//         self.content = text;
//     }

//     pub fn content(&self) -> &str {
//         // 因为需要调用Option值的引用，所以需要使用as_ref()方法
//         self.state.as_ref().unwrap().content(self)
//     }

//     pub fn request_review(&mut self) {
//         // Option<Box<dyn State>>
//         // take()方法会返回Option<Box<dyn State>>, 如果state为Some, 则返回Some(Box<dyn State>)
//         // 如果state为None, 则返回None
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review());
//         }
//     }

//     pub fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve());
//         }
//     }
// }

// pub trait Draw {
//     fn draw(&self);
// }

// pub struct Screen {
//     // Box<dyn Draw> 表示Box里面的元素都实现了Draw tarit, dyn表示动态分发
//     pub components: Vec<Box<dyn Draw>>,
// }

// impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// } 
// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     pub label: String,
// }
// impl Draw for Button {
//     fn draw(&self) {
//         println!("绘制一个按钮, Button");
//     }
// }

// // 实现Summary trait
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
