use std::mem;

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

fn main() {
    //fundamental_data_types();
    //operators();
    //scope_and_shadowing();
    println!("{}", MEANING_OF_LIFE);
    println!("{}", ZA);
    unsafe{
        ZB = 777;
        println!("{}", ZB);
    }
    
}
