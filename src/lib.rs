extern crate uuid;
#[macro_use]
extern crate serde_derive;

mod function;
pub use function::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("TBA");
    }
}
