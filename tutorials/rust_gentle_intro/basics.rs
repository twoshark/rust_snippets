use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

// hello.rs
fn main() {
    cli_args();
    even_odd(true);
    even_odd(false);
    println!("{}", sum_snippets(true));
    println!("{}", sum_snippets(false));
    sqr_snippets(true);
    sqr_snippets(false);
    printing_snippets();
    slice_snippets();
    vector_snippets();
    iterator_snippets();
    slice_segmentation_snippets();
    vector_pop_push();
    vector_other_commands();
    string_snippets();
    match_snippets();
    file_reading_snippets();
    result_type_snippet();
    read_file_with_result();
}

fn even_odd(long_technique: bool) -> () {
    if long_technique {
        println!("~~~~~~~Even Odd Technique Long: ");
        for i in 0..5 {
            if i % 2 == 0 {
                println!("even {i}");
            } else {
                println!("odd {i}");
            }
        }
    } else {
        println!("~~~~~~~Even Odd Technique Short: ");
        for i in 0..5 {
            let even_odd = if i % 2 == 0 { "even" } else { "odd" };
            println!("{} {}", even_odd, i);
        }
    }
}

fn sum_snippets(without_cast: bool) -> String {
    if without_cast {
        println!("~~~~~~~Summing");
        let mut sum = 0;
        for _ in 0..5 {
            sum += 1;
        }
        return format!("Sum: {sum}");
    } else {
        println!("~~~~~~~Summing with Cast");
        let mut sum = 0.0;
        for _ in 0..5 {
            sum += 1 as f64;
        }
        return format!("Sum: {sum}");
    }
}

fn sqr_snippets(with_return: bool) -> () {
    if with_return {
        println!("~~~~~~~Square function");
        let res = sqr(2.0);
        println!("square is {}", res);
    } else {
        println!("~~~~~~~Square function without `return`");
        let res = sqr_no_return(2.0);
        println!("square is {}", res);
    }
}

fn sqr(x: f64) -> f64 {
    return x * x;
}

// this is the most common method
fn sqr_no_return(x: f64) -> f64 {
    x * x
}

fn printing_snippets() -> () {
    println!("~~~~~~~Print - standard");
    println!("print test");
    let x: &str = "print test";

    println!("~~~~~~~Print - var inline");
    println!("{x}");

    println!("~~~~~~~Print - var param ref");
    println!("{}", x);

    println!("~~~~~~~Print - arrays with `debug` print");
    let ints = [1, 2, 3];
    let floats = [1.1, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);
}

fn slice_snippets() -> () {
    println!("~~~~~~~Slice - ints");
    let ints = [1, 2, 3, 4, 5];
    println!("full_list {:?}", ints);

    println!("~~~~~~~Slice - closed range view");
    let closed_range = &ints[0..2];
    println!("closed_range {:?}", closed_range);

    println!("~~~~~~~Slice - open range view");
    let open_range = &ints[1..]; // open range!
    println!("open_range {:?}", open_range);

    println!("~~~~~~~Slice - safe access with arr.get() vs. unsafe arr[] ");
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(4);
    println!("first {:?}", first);
    println!("last {:?}", last);

    println!(
        "first is_some: {} is_none {}",
        first.is_some(),
        first.is_none()
    );
    println!("first value unwrapped {}", first.unwrap());

    println!(
        "last is_some: {} is_none {}",
        last.is_some(),
        last.is_none()
    );
    println!("last value unwrapped {}", last.unwrap());

    println!("~~~~~~~Slice - .get() if .is_some() .unwrap() else constant value");
    let maybe_last = slice.get(5);
    let last = if maybe_last.is_some() {
        *maybe_last.unwrap()
    } else {
        -1
    };

    println!("last {}", last);

    println!("~~~~~~~Slice - .unwrap_or(&default)");
    let last = *slice.get(5).unwrap_or(&-1);
    println!("last {}", last);
}

fn vector_snippets() -> () {
    println!("~~~~~~~Vector - push values onto new vector");
    let mut v = Vec::new(); // the `mut` here is critical
    v.push(10); // this fails if the `Vec` is not `mut`
    v.push(20);
    v.push(30);

    println!("vector is {:?}", v);
    println!("direct accces first is {}", v[0]);
    println!("maybe_first via .get() is {:?}", v.get(0));

    println!("~~~~~~~Vector - coerce slice from `Vec`");
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    dump(&v);

    let slice = &v[1..];
    println!("slice from index 1 is {:?}", slice);
}

fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

fn iterator_snippets() -> () {
    println!("~~~~~~~Iterator - simple example");
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    println!("~~~~~~~Iterator - improved sum tactic");
    let sum: i32 = (0..5).sum();
    println!("sum directly using an iterator was {}", sum);

    let sum: i64 = [0, 1, 2, 3, 4].iter().sum();
    println!("sum indirectly using an iterator from an array was {}", sum);
}

fn slice_segmentation_snippets() -> () {
    println!("~~~~~~~slice.windows(n)");
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s);
    }
    println!("~~~~~~~slice.chunks(n)");
    for s in slice.chunks(2) {
        println!("chunks {:?}", s);
    }
}

fn vector_pop_push() -> () {
    println!("~~~~~~Vector Pop/Push");
    let mut v1 = vec![10, 20, 30, 40];
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2);

    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);
}

fn vector_other_commands() -> () {
    println!("~~~~~~Vector - sort and dedup");
    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];

    v1.sort();
    v1.dedup();
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
}

fn dump_str(s: &str) {
    println!("str '{}'", s);
}

fn string_snippets() -> () {
    println!("~~~~~~`String` - from `&str`");
    let text = "hello dolly"; // the string slice
    let s = text.to_string(); // it's now an allocated string

    dump_str(text);
    dump_str(&s);

    println!("~~~~~~`String` - pushes and pops");
    let mut s = String::new();
    // initially empty!
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!"; // short for `push_str`
                   // remove the last char
    s.pop();

    assert_eq!(s, "Hello World");

    println!("~~~~~~`String` - arr to str with format!()");
    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);

    assert_eq!(res, "hello [10,20,30]");
    println!("~~~~~~`String` - slice access");
    let text = "static";
    let string = "dynamic".to_string();

    let text_s = &text[1..];
    let string_s = &string[2..4];

    println!("slices {:?} {:?}", text_s, string_s);

    println!("~~~~~~`String` - length vs count");
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("'{}' ", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }
}

fn array_to_str(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

fn cli_args() -> () {
    println!("~~~~~~CLI Arguments - iterate over ::args()");
    for arg in std::env::args() {
        println!("'{}'", arg);
    }

    println!("~~~~~~CLI Arguments - collect() into Vec");
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0 {
        // we have args!
        for arg in std::env::args() {
            println!("'{}'", arg);
        }
    }

    println!("~~~~~~CLI Arguments - nth(index)");
    let first = env::args().nth(1).expect("please supply an argument");
    let _n: String = first.parse().expect("not a String!");
}

fn match_snippets() -> () {
    println!("~~~~~~`match` - find example");
    let multilingual = "Hi! ¡Hola! привет!";
    match multilingual.find('п') {
        Some(idx) => { // unwrapped result from `find` bound to idx
            let hi = &multilingual[idx..];
            println!("Russian hi {}", hi);
        },
        None => println!("couldn't find the greeting, Товарищ")
    };
    

    let n = 8;
    println!("~~~~~~`match` - switch statement like example");
    let _text = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "many", // _ is the default case
    };

    println!("~~~~~~`match` - range matching with iterators");
    let _text = match n {
        0..=3 => "small",
        4..=6 => "medium",
        _ => "large",
     };
}


fn file_reading_snippets()->() {
    println!("~~~~~~File Reading - ");
    let first = env::args().nth(1).expect("please supply a filename");

    let mut file = File::open(&first).expect("can't open the file");
    let mut file2 = File::open(&first).expect("can't open the file");

    let mut text = String::new();
    let mut bytes = Vec::<u8>::new();
    file.read_to_string(&mut text).expect("can't read the file as string"); //fails if not utf-8
    println!("file had {} bytes", text.len());
    file2.read_to_end(&mut bytes).expect("can't read the file as bytes"); 
    println!("file had {} bytes", bytes.len());
    
}

fn result_type_snippet() ->() {
    println!("{:?}",good_or_bad(true));
    //Ok(42)
    println!("{:?}",good_or_bad(false));
    //Err("bad")

    match good_or_bad(true) {
        Ok(n) => println!("Cool, I got {}",n),
        Err(e) => println!("Huh, I just got {}",e)
    }
    // Cool, I got 42
}

fn good_or_bad(good: bool) -> Result<i32,String> {
    if good {
        Ok(42)
    } else {
        Err("bad".to_string())
    }
}

fn read_file_with_result()-> (){
    let file = env::args().nth(1).expect("please supply a filename");
    let text = read_to_string(&file).expect("bad file man!");
    println!("file had {} bytes", text.len());

    let file2 = env::args().nth(1).expect("please supply a filename");
    let text2 = read_to_string_with_io_result(&file).expect("bad file man!");
    println!("file had {} bytes", text.len());
}

fn read_to_string(filename: &str) -> Result<String,io::Error> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}

fn read_to_string_with_io_result(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?; //returns the error here if there is one
    Ok(text) // otherwise Ok()
}
