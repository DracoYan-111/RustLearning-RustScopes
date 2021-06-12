fn main() {
    //s不可用
    let a: &str = "hello";//a可用
    //以下可以对a进行相关操作

    //---------------------
    let mut b = String::from("Hello");
    //push_str:向b变量后加入某字符串
    b.push_str(".World");
    println!("{}", b);//输出Hello.World

    //---------------------
    //当c声明时，字符内容会被保存在heap(堆)
    let c = String::from("Hello");
    //当c1 = c时，c的内容会被移动
    let c1 = c;
    println!("{}", c);//此时c将不再可用

    //---------------------
    let d: String = String::from("hello");
    //clone:对其他变量进行拷贝
    let d1 = d.clone();
    println!("{},{}", d, d1);//d:hello,d1:hello,两个变量都可用，d的变量未发生移动
}//a作用域结束，a不再可用
