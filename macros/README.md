# Rust Macro Practice Tasks

## 🟢 Beginner: Basic Declarative Macros (`macro_rules!`)

1. **Hello Macro**  
   Write a macro `hello!()` that expands to `println!("Hello, macros!");`.

2. **Greet Someone**  
   Create a macro `greet!(name)` that prints `"Hello, <name>!"`.

3. **Add Two Numbers**  
   Implement `add!(a, b)` that returns the sum of two literals (e.g., `add!(3, 5)` → `8`).

4. **Repeat Print**  
   Write `repeat_print!(text, n)` that prints `text` `n` times using a loop.

5. **Conditional Debug Print**  
   Create `dbg_if!(cond, msg)` that prints `msg` only if `cond` is true.

---

## 🟡 Intermediate: Pattern Matching & Repetition

1. **Sum Any Number of Args**  
   Implement `sum!(a, b, ...)` that sums all provided integer literals (e.g., `sum!(1, 2, 3)` → `6`).

2. **Vector Literal (Simplified `vec!`)**  
   Write `my_vec![x; n]` → creates a vector with `n` copies of `x`, and `my_vec![a, b, c]` → `[a, b, c]`.

3. **Map-Like Macro**  
   Create `map_vec!(|x| expr, in vec)` that applies `expr` to each element of `vec` and returns a new `Vec`.

4. **Assert Equals with Custom Message**  
   Implement `assert_eq_msg!(a, b, "msg")` that panics with `"msg"` if `a != b`.

5. **Logging Macro with Level**  
    Build `log!(level, message)` where `level` is `info`, `warn`, or `error`, and it prints accordingly.

---

## 🔴 Advanced: Emulating Built-in Macros

1. **Reimplement `println!` (Basic)**  
    Create `my_println!(fmt, ...)` that works like `println!` for format strings and arguments.

2. **Clone Derive (Simplified)**  
    Write a macro `derive_clone!(StructName { field1: Type1, field2: Type2 })` that implements `Clone` manually for a struct with cloneable fields.

3. **Full `vec!` Clone**  
    Recreate `vec!` supporting both `vec![1, 2, 3]` and `vec![0; 10]` syntaxes.

4. **Custom `format!`**  
    Implement `my_format!(...)` that returns a `String` using `std::fmt`.

5. **Thread-Safe Static Initialization**  
    Create a macro `lazy_static!(NAME = value)` that safely initializes a static variable once (hint: use `std::sync::OnceLock`).

---

## ⚫ Expert: Procedural & Hygiene Challenges

1. **Attribute-like Macro (Fake `#[derive(Debug)]`)**  
    Simulate a derive macro that adds `Debug` implementation using only `macro_rules!` (for structs with named fields).

2. **Macro Generating Unit Tests**  
    Write `gen_tests!(fn_name, input, expected)` that generates a test function asserting `fn_name(input) == expected`.

3. **Hygienic Counter Macro**  
    Create a macro that expands to a unique static counter incremented on each call (use `$crate` and hygiene tricks).

4. **SQL-like DSL Macro**  
    Implement `query!(SELECT col FROM table WHERE cond)` that parses simple SQL-like syntax into Rust function calls.

5. **Error-Handling Wrapper**  
    Build `try_or!(expr, err_msg)` that returns `Err(err_msg.into())` if `expr` is `Err`, otherwise unwraps `Ok`.

---

# Theory Questions (Multiple Choice)

1. **What kind of macros does `macro_rules!` define?**  
    - [ ] A) Procedural macros  
    - [ ] B) Declarative macros  
    - [ ] C) Function-like macros only  
    - [ ] D) Both B and C  

2. **Which macro type runs at compile time and can inspect/modify AST?**  
    - [ ] A) `macro_rules!`  
    - [ ] B) Function-like macros  
    - [ ] C) Procedural macros  
    - [ ] D) All of the above  

3. **What is macro hygiene?**  
    - [ ] A) Preventing macro-generated code from conflicting with user variables  
    - [ ] B) Ensuring macros are safe from memory leaks  
    - [ ] C) Cleaning up unused macro definitions  
    - [ ] D) Formatting macro output  

4. **Can `macro_rules!` macros be recursive?**  
    - [ ] A) No, recursion is forbidden  
    - [ ] B) Yes, but only tail-recursion  
    - [ ] C) Yes, with no restrictions  
    - [ ] D) Only if marked `#[recursive]`  

5. **How do you match zero or more repetitions in `macro_rules!`?**  
    - [ ] A) `$(...)*`  
    - [ ] B) `$(...)+`  
    - [ ] C) `$(...)?`  
    - [ ] D) `$(...)..`  

6. **What does `concat!` do?**  
    - [ ] A) Concatenates strings at runtime  
    - [ ] B) Concatenates literals at compile time  
    - [ ] C) Joins vectors  
    - [ ] D) Merges macro outputs  

7. **Which built-in macro is used for compile-time panic?**  
    - [ ] A) `panic!`  
    - [ ] B) `compile_error!`  
    - [ ] C) `assert!`  
    - [ ] D) `unreachable!`  

8. **Procedural macros must be defined in:**  
    - [ ] A) The same crate  
    - [ ] B) A separate crate with `proc-macro = true`  
    - [ ] C) Only in `lib.rs`  
    - [ ] D) Any module marked `#[macro_use]`  

9. **What is the expansion site of a macro?**  
    - [ ] A) Where the macro is defined  
    - [ ] B) Where the macro is called  
    - [ ] C) Inside `Cargo.toml`  
    - [ ] D) In the preprocessor buffer  

10. **Can macros generate items (like functions or structs)?**  
     - [ ] A) No  
     - [ ] B) Only procedural macros can  
     - [ ] C) Yes, both declarative and procedural  
     - [ ] D) Only with `#[macro_export]`  

11. **What does `stringify!` do?**  
     - [ ] A) Converts an expression to a string literal at compile time  
     - [ ] B) Serializes a struct to JSON  
     - [ ] C) Prints to stdout  
     - [ ] D) Quotes a string for shell execution  

12. **Which syntax matches a single token tree?**  
     - [ ] A) `$x:ident`  
     - [ ] B) `$x:expr`  
     - [ ] C) `$x:tt`  
     - [ ] D) `$x:item`  

13. **How do you export a macro for use in other crates?**  
     - [ ] A) `pub macro my_macro! {...}`  
     - [ ] B) `#[macro_export]`  
     - [ ] C) `extern crate my_macro;`  
     - [ ] D) `use crate::my_macro;`  

14. **What is the main limitation of `macro_rules!` vs procedural macros?**  
     - [ ] A) Cannot generate new identifiers  
     - [ ] B) Cannot inspect types  
     - [ ] C) Cannot be recursive  
     - [ ] D) Slower compilation  

15. **Which fragment specifier matches a block `{ ... }`?**  
     - [ ] A) `block`  
     - [ ] B) `stmt`  
     - [ ] C) `expr`  
     - [ ] D) `item`  

16. **What does `env!` return?**  
     - [ ] A) Runtime environment variables  
     - [ ] B) Compile-time environment variable as a string literal  
     - [ ] C) Current working directory  
     - [ ] D) OS name  

17. **Can a macro expand to multiple statements in expression position?**  
     - [ ] A) Yes, always  
     - [ ] B) No, expressions must be single values  
     - [ ] C) Only inside blocks  
     - [ ] D) Only with `unsafe`  

18. **What is "TT munching"?**  
     - [ ] A) A technique to recursively consume token trees in `macro_rules!`  
     - [ ] B) A bug in old Rust versions  
     - [ ] C) Parsing JSON in macros  
     - [ ] D) Optimizing macro expansion speed  

19. **Which macro is used to include file contents at compile time?**  
     - [ ] A) `include_str!`  
     - [ ] B) `read_file!`  
     - [ ] C) `file!`  
     - [ ] D) `embed!`  

20. **Are macros Turing-complete?**  
     - [ ] A) No  
     - [ ] B) Only procedural macros  
     - [ ] C) Yes, `macro_rules!` is Turing-complete  
     - [ ] D) Only with nightly features  
