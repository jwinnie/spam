
extern crate spam;

fn main() {
    let a = spam::repository::tap("https://raw.githubusercontent.com/hyperium/hyper/master/Cargo.toml");
    println!("{:?}", a);
}