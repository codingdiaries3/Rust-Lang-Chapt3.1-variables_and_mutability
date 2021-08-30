### Notes

## Variables and Mutability

1. Variables are immutable by default.

2. Immutable/Mutable
   ```
   let mut x = 5 \\mut indicates the variable is intended to change its value
   ```

## Differences Between Variables and Constants

1. You can't use 'mut' with constants. Constants are always immutable.

2. Constant value type must be annotated.

3. Constants can be declared in any scope and should be used if you want a value to stay the same throughout the whole program.

## Shadowing

1. The difference between using `mut` to make a variable mutable vs shadowing, is that the mutable variable will be mutable for the rest of the program but the shadowed variable, once the value has been changed, will be immutable.

2. To shadow a variable you must use `let`.
   ```
   let x = 5
   let x = 6 // this is now immutable
   ```
3. When shadowing a variable we can change the type of the value and reuse the same name because we are effectively creating a new variable when using let.

4. The limitations of using `mut` is that you can't reassign it a different type value. The value changed can only be of the same type. Example:
   ```
   let mut x = 5;
   x = 'string';
   ```
   This will throw an error becuase the type value is different.
