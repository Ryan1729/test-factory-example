pub mod a {
  #[derive(Debug)]
  pub struct A {}
  
  #[macro_export]
  macro_rules! test_factory { 
     () => { 
         shared::test_factory_inner!($crate::a::A)
     }
  }

  #[test]
  fn test_something() {
    let my_a = test_factory!();
    
    dbg!(my_a);

    panic!("fail to see the logs");
  }
}