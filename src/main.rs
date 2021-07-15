#![allow(dead_code)]
use std::mem;
mod pm;
mod sh;

const MEANING_OF_LIFE: u8 = 42; // no fixed address

static ZA: i32 = 123;
static mut ZB: i32 = 123;

fn operators() {
    let mut a = 2 + 3 * 4;
    println!("{}", a);
    a = a + 1;
    a -= 2;
    println!("{}", a);

    let a_cubed = i32::pow(a, 3);
    println!("{}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    println!("{}", b_cubed);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{}", b_to_pi);

    //bitwise
    let c = 1 | 2; // | or    & and     ^ XOR    ! NOR
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10;
    println!("2^8 = {}", two_to_10);
    //logical
    let ss = std::f64::consts::PI < 4.0; //true // > <= == >=
    println!("ss = {}", ss);
}

fn fundamental_data_types() {
    // unsigned 0 +
    let a: u8 = 123; //8bit

    // signed
    let c: i16 = 555; //8bit

    println!("a = {}", a);
    println!("c = {}", c);

    //mut that value can change
    let mut b: i8 = 0;
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut d = 123456789; // 32 bit signed i32
    println!("c = {}, size = {} bytes", d, mem::size_of_val(&d));
    d = -1;
    println!(
        "c = {}, size = {} bytes, after modification.",
        d,
        mem::size_of_val(&d)
    );

    let z: isize = 123; //take the maximum size base on the operation system x86 or x64
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit os",
        z,
        size_of_z,
        size_of_z * 8
    );

    let f = 'x';
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));

    let e = 2.5;
    let g: f32 = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));

    //true false
    let j = false;
    println!("j = {}, size = {} bytes", j, mem::size_of_val(&j));
}

fn scope_and_shadowing() {
    let a = 123;
    {
        let b = 456;
        println!("inside b = {}", b);
        let a = 777; //it shadows outer a
        println!("inside a = {}", a);
    }

    println!("outside a = {}", a);
    //println!("outside b = {}",b); //illegal
}

fn if_statement() {
    let temp = 25;
    if temp > 30 {
        println!("really hot");
    } else if temp < 10 {
        println!("really cold");
    } else {
        println!("temperature is ok");
    }
    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("day= {}", day);
}

fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        // 64 will skip to print
        if x == 64 {
            continue;
        }

        println!("x = {}", x);
    }

    let mut y = 1;
    loop
    //while true
    {
        y *= 2;
        println!("y={}", y);
        if y == 1 << 10 {
            break;
        }
    }
}

fn for_loop() {
    for x in 1..11 {
        if x == 3 {
            continue;
        }
        if x == 8 {
            break;
        }
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}:{}", pos, y);
    }
}

fn match_statement() {
    let country_code = 98;
    let country = match country_code {
        44 => "UK",
        98 => "IR",
        7 => "RU",
        46 => "SW",
        1...999 => "unKnown",
        _ => "inValid",
    };

    println!("{} : {}", country_code, country);
}

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({},{})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2 };
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), //tuple
    Cmyk {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, //struct
}
fn enums() {
    //let c: Color = Color::Red;
    //let c: Color = Color::RgbColor(13,4,55);
    let c: Color = Color::Cmyk {
        cyan: 0,
        magenta: 1,
        yellow: 10,
        black: 13,
    };
    match c {
        Color::Red => println!("r"),
        Color::Blue => println!("b"),
        Color::Green => println!("g"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        Color::Cmyk {
            cyan: _,
            yellow: _,
            black: _,
            magenta: _,
        } => println!("cmyk()"),
        _ => (),
    }
}

union IntOrFloat {
    i: i32,
    f: f32,
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life");
            }
            IntOrFloat { f } => {
                println!("f32: {}", f);
            }
        }
    }
}

fn unions() {
    let mut iof = IntOrFloat { i: 132 };

    unsafe {
        iof.i = 42;
    }

    let value = unsafe { iof.i };

    process_value(iof);
    process_value(IntOrFloat { f: 1.23 });
}

fn option() {
    //option<T> instead of null handling we have it here
    // Some(z) or None

    let x = 3.0;
    let y = 2.0;

    let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };

    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("can not devide by 0"),
    }

    // if let / while let used to check the return value of fund or every value then process something
    if let Some(z) = result {
        println!("z= {}", z);
    }
}

fn arrays() {
    //let mut a = [1,2,3,4,5];
    //size is exact value
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a[0] is {}", a[0]);

    println!("{:?}", a);

    if a == [321, 2, 3, 4, 5] {
        println!("match");
    }

    let b = [1; 10];
    //println!("{:?}",b);
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b too up {} bytes", mem::size_of_val(&b));

    let mtx: [[f64; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];

    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn vectors() {
    //can grow dynamically and not fixed size as an array
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("{:?}", a);
    a.push(44);
    println!("{:?}", a);
    println!("a[0] = {}", a[0]);
    //println!("a[0] = {}",a[111]); //crashed

    match a.get(3) {
        Some(x) => println!("a[3]={}", x),
        None => println!("error, no such element"),
    }

    for x in &a {
        println!("{}", x);
    } //print all elements

    a.push(77);
    println!("{:?}", a);
    let last_elm = a.pop(); //pop return option , some or none
    println!("{:?}", a);
    println!("last elem is {:?} and a is {:?}", last_elm, a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn use_slice(slice: &mut [i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices() {
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]);
    //use_slice(&mut data);
    println!("{:?}", data);
}

fn strings() {
    //let s = "hello there";
    let s: &'static str = "hello there";

    for c in s.chars()
    //.rev()
    {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first char is = {}", first_char);
    }

    //heap
    //string
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    let z = letters + "aaaaa";
    println!("{}", z);

    let mut abc = String::from("helllo");
    println!("{}", abc.replace("ll", "ff"));
    let mut aaa = "asdsalkdsj".to_string();
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("sp is {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    //destructring
    let (a, b) = sp;
    println!("a={},b={}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c, d), (e, f)) = combined;
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    let meaning = (42,);
    println!("{:?}", meaning);
}

struct Pointt<T> {
    x: T,
    y: T,
}

struct Linee<T> {
    start: Pointt<T>,
    end: Pointt<T>,
}

fn generics() {
    let a: Pointt<f64> = Pointt { x: 0.0, y: 4.0 };
    let b = Pointt { x: 1.2, y: 3.4 };

    let myline = Linee { start: a, end: b };
    //println!("{:?}",myline);
}

fn increase(x: &mut i32) {
    *x += 1;
}
fn functions() {
    let mut z = 1;
    increase(&mut z);
    println!("z = {}", z);
    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("p={}", p);
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

struct Pointtt {
    x: f64,
    y: f64,
}

struct Lineee {
    start: Pointtt,
    end: Pointtt,
}

impl Lineee {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn methods() {
    let p = Pointtt { x: 3.0, y: 4.0 };
    let p2 = Pointtt { x: 5.0, y: 10.0 };
    let myLine = Lineee { start: p, end: p2 };

    println!("lenght= {}", myLine.len());
}

fn say_hello() {
    println!("hello");
}

fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x: i32| -> i32 { x + 1 };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let plus_two = |x| {
        let mut z = x;
        z += 2;
        z
    };
    println!("{} + 2 = {}", 3, plus_two(3));

    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}

fn main() {
    closures();
    //methods();
    //functions();
    //generics();
    //pm::pattern_matching();
    //tuples();
    //strings();
    //slices();
    //vectors();
    //arrays();
    //option();
    //unions();
    //enums();
    //fundamental_data_types();
    //operators();
    //scope_and_shadowing();
    // println!("{}", MEANING_OF_LIFE);
    // println!("{}", ZA);
    // unsafe{
    //     ZB = 777;
    //     println!("{}", ZB);
    // }
    //sh::stack_and_heap();
    //if_statement();
    //while_and_loop();
    //for_loop();
    //match_statement();
    //structures();
}
