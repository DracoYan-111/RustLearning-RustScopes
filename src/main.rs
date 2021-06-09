fn main() {
    //s不可用
    let s: &str = "hello";//s可用
    //以下可以对s进行相关操作
    //---------------------
    let mut m = String::from("Hello");
    //push_str:向m变量后加入某字符串
    m.push_str(".World");

    println!("{}", m);//输出Hello.World
}//s作用域结束，s不再可用

