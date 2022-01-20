fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod b {
  #[test]
  fn test_something() {
    let my_a = shared::test_factory!();
    
    dbg!(my_a);

    panic!("fail to see the logs");
  }
}

