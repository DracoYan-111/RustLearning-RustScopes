fn main() {
    //s不可用
    let a: &str = "hello";//a可用
    //以下可以对a进行相关操作

    //=================特殊的String类型=================
    let mut b = String::from("Hello");
    //push_str:向b变量后加入某字符串
    b.push_str(".World");
    println!("{}", b);//输出Hello.World

    //=================所有权的转移=================
    //当c声明时，字符内容会被保存在heap(堆)
    let c = String::from("Hello");
    //当c1 = c时，c的内容会被移动
    let c1 = c;
    println!("{}", c);//此时c将不再可用

    //=================String类型的Clone=================
    let d: String = String::from("hello");
    //clone:对其他变量进行拷贝
    let d1 = d.clone();
    println!("{},{}", d, d1);//d:hello,d1:hello,两个变量都可用，d的变量未发生移动

    //=================i32整形的复制=================
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

    //=================返回值与作用域=================
    let g = gives_ownership();//g得到gives_ownership的返回值some_string，所有权将转移给g
    let g1 = String::from("hello");//g1进入作用域
    let g2 = takes_and_gives_back(g1);//takes_and_gives_back方法的返回值被移动到g2上

    //=================函数使用某值，但不获得其所有权=================
    let h = String::foem("hello");//h进入作用域
    let (h2, len) = calculate_length(h);//将h传入calculate_length方法
    println!("{},{}", h2, len);//h2==于h

    //=================引用和借用=================
    let i = String::from("hello");//i进入作用域
    let lenTwo = calculate_lengthTwo(&i);//把i的引用作为参数传给calculate_lengthTwo方法
    println!("{},{}", i, lenTwo);//i仍然可以使用
}//a作用域结束，a不再可用

//=================所有权与函数=================
fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

//=================返回值与作用域=================
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string//返回值所有权移动
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string//j1所有权被移动到该函数里
}

//=================函数使用某值，但不获得其所有权=================
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len;//获取到传入字符串的长度
    (s, length)//将s原封不动的返回和字符长度
}

//=================引用和借用=================
fn calculate_lengthTwo(s: &String) -> usize {//String类型的引用
    s.len()
}