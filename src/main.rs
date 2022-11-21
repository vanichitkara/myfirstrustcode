fn main() {
    println!("Hello, world!");
    let var = 8;//any lifetime, can be deleted anytime
    let var1 = "name";
    let var2 = true;
    let mut var3 = 9;
    var3 = 10;
    let r = 10.1;
    let r = "";
    let e:i64 = 0;

    //for const and static, the type needs to be inferred explicitly
    //we can't write const f=0 or static t=0, we need to mention the type (i32 or i64 etc) too
    //mutable const variables are not a possibility, we can only have immutable const variables. static can be mutable though
    const f:i32=0;
    static t:i64=0;
    static mut q:i32 = 0; //it can only have a lifetime as static,i.e., it won't be deleted during the whole execution

    let d = (); //unit type -> void type in other languages

    let st = "/"; //&str -> reference to pointer str -> getting stored in stack
    //str cannot be stored on stack, that's why we use &str to store the variable in stack.
    let st1 = String::from("abc"); //String type -> it is not a reference but is directly getting stored in heap
    let st2:Box<str> = "vani".into(); //str type -> the type whose size is unknown during compile time will always get stored in heap
    //Box is used to store the string from stack to heap

    let str = String::from("name");
    let slice_str=&str[1..2]; //slice of variable string from index 1 to 2. slice of "name" from [1..2] = "am"

    //match statement
    let eat = true;

    match eat {
        true =>{

        },
        false =>{

        }
    }

    let r=8;
    match r {
        2 => {
            println!("it is 2");
        },
        3=> {
            print!("it is 3");
        },
        8 => {
            print!("it is 8");
        },
        _ => {
            print!("let's ignore it")
        }
    }

    loop {
        let r=8;
        match r {
            2 => {
                println!("it is 2");
            },
            3=> {
                println!("it is 3");
            },
            8 => {
                println!("it is 8");
            },
            _ => {
                break;
            }
        }
    }

    let range = (1..9); //with 9 excluded;
    let range_included = (1..=9); //9 is included in the range now

    // for loop
    for i in range {
        println!("{}", i);
    }

    // while loop
    while true{
        
    }
}

fn print_my_name(){
    println!("Vani"); //whatever we see with a ! in rust is a macros
    //macros is used for metaprogramming in rust
    //panic!(); is a macros which stops the execution of code as soon as it is encountered
}

fn addition(a:i32, b:i32)->i32{
    a+b //we can also write it as return a+b;
}

fn return_void() -> (){
    ()
}

//diverging function -> will never return to main

fn diverging_fn() -> !{
    panic!();
}

//if-else statement
/*if true{
 
}else {

}*/

/*
signed integers: i8 to i28
unsigned integers: u8 to u256
float integers: f8 to f128
whichever file has the main function is called the binary file, the rest of the files are called library files
.toml file has the dependencies and versions
*/