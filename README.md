# curry-macro
 a curry! macro for rust

## example usage
 ```rust

// define function as ">"-separated list of arguments and types, and "=>" before return type
let curried_function = curry_fn!(x:i32 > y:i32 > z:i32 => i32 {
	x+y+z
});

 ```
