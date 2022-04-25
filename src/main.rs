mod lib;
fn main() {
    lib::hello();

    let x = lib::compute(1, 2);

    println!("{}", x);

    let v = lib::create_vector();

    println!("{:?}", v);
    
}
