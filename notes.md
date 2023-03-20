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

Ownership
* Memory is managed through a system of ownership with a set of rules that the compiler checks.
* Rust will never automatically create “deep” copies of your data.
* If an object is on the stack then a copy is performed on assignment. If it is on the heap then a move is performed on assignment.
    * https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
    * Nit: This may not be completely true, what really determines if a copy happens is if the `Copy` trait is present for that type. The `Copy` trait essentially requires that the type has no `Drop` trait, i.e. does not require Heap allocation.
* `clone` performs deep copies.
    * https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-clone
* Passing a variable to a function will move or copy, just as assignment does.
    * https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions
* Returning values can also transfer ownership.
    * https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope

References and Borrowing
* The action of creating a reference is called "borrowing". 
    * https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing
* References are also immutable by default.
* In order to prevent data races at compile time, mutable references restrict you from having any other simultaneous references to the same value.
* Te compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

Slices
* A type of reference to a contiguous sequence of elements in a collection rather than the a whole collection.
    * https://doc.rust-lang.org/book/ch04-03-slices.html

Structs
* "Field init shorthand" allows you to avoid repeating the name of a parameter to a struct initialization function if it matches the field name of the struct.
    * https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-the-field-init-shorthand
* "Struct update syntax" (`..`) allows you to copy from another struct while updating just a few values. Keep in mind that it will move any non-copyable values since this is an assignment.
    * https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
* "Tuple structs" allow for naming a tuple type and specifying the types of the fields, but without needing to name the fields.
    * https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
* "Unit-like structs" are structs without any fields. They behave similarly to a "unit tuple", which has no values.
* The Debug trait enables debug formatting in format strings, which you indicate by adding :? within {} placeholders. You can also use the `dbg!` macro.
* Methods are functions defined within the context of a struct (or an enum or trait object) and their first parameter is always `self`.
* A method is defined like a function but within an `impl` (implementation) block for the struct.
* Methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably, just as they can any other parameter.
* Having a method that takes ownership of `self` is rare; this technique is usually used when the method transforms `self` into something else and you want to prevent the caller from using the original instance after the transformation.
* Rust doesn’t have an equivalent to the `->` operator in C++; instead, Rust has a feature called automatic referencing and dereferencing.
* All functions defined within an `impl` block are called "associated functions" because they’re associated with the type named after the `impl`.
    * https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions
* All methods are "associated functions", however there are also non-method "associated functions" which are those that do not take `self` as the first parameter.
* Each struct is allowed to have multiple impl blocks.
    * https://doc.rust-lang.org/book/ch05-03-method-syntax.html#multiple-impl-blocks
