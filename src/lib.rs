pub fn hello() -> () {
    println!("Hello, Noodle!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
