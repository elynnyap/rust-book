/* technically, outermost does not need to be public in order for
 * try_me() to refer to it, since try_me() is located in the same 
 * root module as outermost. but we add pub to get rid of the unused
 * functions warning in Rust. */
pub mod outermost {

    // middle_function() and middle_secret_function() both need to be public
    // to be accessible outside of outermost.
    pub fn middle_function() {}

    pub fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {
            // we can start with :: to refer to the root module and access the outermost module's
            // function this way.
            ::outermost::middle_secret_function();
        }

        pub fn secret_function() {}
    }
}

pub fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
