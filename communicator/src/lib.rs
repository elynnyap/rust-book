/*in Rust, all code is private by default
by marking functions as public, we indicate that the function is intended to be used
by code outside the program
this gets rid of the 'unused' warnings bc of the theoretical external usage.*/

pub mod client;

pub mod network; 

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
