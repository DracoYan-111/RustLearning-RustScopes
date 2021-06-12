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

    //---------------------
    let e: i32 = 5;//整数类型将完整的数据存在了stack中
    let e1: i32 = e;//复制操作都是快速的，无法阻止x仍然有效
    println!("{},{}", e, e1);//xy都有效，且都为5

    //=================所有权与函数=================

    let f = String::from("Hello World");//f进入作用域

    take_ownership(f);//f的值被移动到take_ownership方法里

    println!("{}", f);//此处f将不在有效

    let f1 = 5;//f1进入作用域

    makes_copy(f1);//i32类型copy副本到makes_copy方法里

    println!("{}", f1);//f1仍然有效
}//a作用域结束，a不再可用

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}