#[macro_export]
macro_rules! curry_fn (
    // ($s:ident, $( $arg:ident: $arg_type:tv ), * ) => {
    ($first_arg:ident:$first_arg_type:ty $(> $arg:ident:$arg_type:ty )* => $ret_type:ty {
        $function_body:expr 
    }) => {
       move |$first_arg:$first_arg_type| $(move |$arg:$arg_type|)* {
          $function_body
       }
    };
);

#[macro_export]
macro_rules! call_curried (
    ($function_name:ident, $( $arg:expr),*) => {
        $function_name$(($arg))*
    }
);


#[cfg(test)]
mod tests {
    fn uncurried_function(x:i32, y:i32, z:i32) -> i32 { x+y+z }

    #[test]
    fn curried_function_returns_same_as_uncurried_function() {
        let curried_function = curry_fn!(x:i32 > y:i32 > z:i32 => i32 {
            x+y+z
        });

        assert!(curried_function(1)(2)(3) == uncurried_function(1,2,3))
    }

    #[test]
    fn call_curried_function_works() {
        let curried_function = curry_fn!(x:i32 > y:i32 > z:i32 => i32 {
            x+y+z
        });

        assert!(call_curried!(curried_function, 1, 2, 3) == uncurried_function(1,2,3));
    }
}
