fn main() {
/*     let x = 5;
    let x = x + 1;
    let x = x * 2;
    let mut b = 12;
    /***
     * if the value was set as  mut  then it can be change by nomal operating
     */
/*     b=2;
    const B:u32  =12;
    let  a = "1234";

    /***
     * const value can't be changed
     */
    /***
     * this may make this value be changed because of this mechanism called "shadowing"
     */
/*     let mut c = (1,2,3);
    let d = [1,2,3,4];
    let s={
        let y  =2;
        y+2
    };
    
    println!("The value of x is: {0} {1} {2} {3} {4} {5} {6}", x,B,b,a,c.1,d[0],add(x, b));
    println!("this is the lambda?{0} {1}",s,getNum(s)); */ */ */
    let mut count =0;
    for i in 0..10 {
        count+=1;
    }
    let mut a =true;
    if a{
        println!("true");
        println!("{}",count);
    }else{
        println!("erro");
    }
    let b  =if count>0 {1}else{2};
    println!("{}",b);
    loop{
        if a!=true{
            println!("gg");
            break;
        }else{
            a = false;
        }
    }
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch == 'O' {
            break i;
        }
        i += 1;
    };
    println!(" \'O\' 的索引为 {}", location);
    let x =5;
    let b = 5;
    let str1 = String::from("kell");
    let str2 = str1.clone();
    println!("{} {}",str1,str2);
}
fn add(a :i32,b:i32)->i32{
    return a+b;
}
fn getNum(a:i32)->i32{
    fn sed(b:i32)->i32{
        return b+1;
    }
    return a+sed(a);
}
