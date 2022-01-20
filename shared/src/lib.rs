#[macro_export]
macro_rules! test_factory_inner { 
   ($a_path: path) => {{
       $a_path{}
   }}
}

#[macro_export]
macro_rules! test_factory { 
   () => {{
       $crate::test_factory_inner!(crate_of_a::a::A)
   }}
}

