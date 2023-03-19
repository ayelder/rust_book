Variables
* Variables are immutable by default.
    * https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
* Shadowing is commonly used for transforming a variable to a different type, e.g. string to int.
    * https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing


Statements & Expressions
* If a line ends in a `;` then it is a statement and would not reflect a value, i.e. cannot be assigned to a variable.
    * https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions

Functions
* The last expression of a function is the value returned

Control Flow
* `if` can be used during variable assignment, similar to `?` operator in C++.
    * https://doc.rust-lang.org/book/ch03-05-control-flow.html#using-if-in-a-let-statement
* `loop` is basically like `while(true)` in C++.
    * https://doc.rust-lang.org/book/ch03-05-control-flow.html#repeating-code-with-loop
* `break` can return a value from a `loop`.
    * https://doc.rust-lang.org/book/ch03-05-control-flow.html#returning-values-from-loops
* Loops can be labeled. Those labels can then be used in `break` and `continue`.
    * https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels-to-disambiguate-between-multiple-loops
* `for` loops are range based, like Python.