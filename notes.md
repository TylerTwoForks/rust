# Variables

- Immutable - once a value is bound to a name, you can't change that value.
  - `let x = 5;`
  - variables are immutable by default
  - you can add the `mut` keyword before a var to make it mutable
  - `let mut x = 5;`
- Constants
  - similar to immutable variables.
  - always immutable, you cannot mutate them.
  - must be set to a constant expression. not the result of a value that can only be computed at run time.
  - Type of the value must always be annotated.
  - `const THIS_IS_MY_CONST: u32 = 60 * 60 * 3;`
- Shadowing
  - declaring a new variable with the same name as an existing variable.
  - example below.  but shadowing would allow us to make a transformation to the value without marking the variable as `mut`, allowing it to remain non-mutable after transformation.
  - this also allows us to change the Type of the variable but reuse the same name.  Common in casting situations.
    - this will throw an error if we use `let mut`

```rust
fn main() {
    let x = 5;
    let x = x + 1; //shadowed the first instanc of x == 6

    {
        let x = x * 2; //shadowed the second instance of x == 12
        println!("The value of x in the inner scope is: {x}");
    }
    //the x == 12 is out of scope here (leaving the crulies) so it's dropped. 

    println!("The value of x is: {x}"); //prings 6 now that  the 12 has been dropped. 

    //casting example. shdowing often used to cast from one type to another. 
    let spaces = "   ";
    let spaces = spaces.len();

}
```

# Data Types
Every value in rust has a Type
- statically typed language - must know types of variables at compile time. 
- compiler can often infer type you want, but in cases where multiple are possible, you must specify. 
```rust
let guess: u32 = "42".parse().expect("Not a number!");
         //^^^---> this is us annotating the type in a situation where multiple types are possible
```

## Scalar Types
- represents a single value
- Rust has 4 scalar types: integers, floating-point, Booleans, and characters.

### Integers
