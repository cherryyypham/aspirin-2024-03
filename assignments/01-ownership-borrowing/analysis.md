# Analysis
## basics.rs - fn find_in_string
### Lifetime Error
Having done the homework 2 weeks late, after we have learned the concept of lifetimes, I understand how the function would have failed if I did not define a lifetime annotation that ties the input lifetimes to the lifetime of the output reference. The function initiation for this would look like:

```
fn find_in_string (string: &str, substr: &str) -> Option<&str>
```

The above would receive an error when ran, and the compiler will notify you that "this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `string` or `substr`". As opposed to the correct declaration.

```
fn find_in_string <'a> (string: &'a str, substr: &'a str) -> Option<&'a str>
```

In this version of declaration, by employing the lifetime annotation `'a`, you're telling the Rust compiler that both the `string` and `substr` arguments share the same lifetime with each other and with the returned reference. This guarantees that the slice being returned cannot outlive either the `string` or `substr`, which are both essential to infer the output.


### Ownership-related Warning
*Side Notes: I know this is not the part that was asked, but the compiler also ran with a warning for the following line in `test_find_in_string()`:*

```
drop(&word);
```

The error reads "warning: calls to `std::mem::drop` with a reference instead of an owned value does nothing". This means that you're passing a reference to drop instead of the actual owned value. Since references don't own the value, calling drop on a reference does nothing—Rust will still automatically drop the value later when it goes out of scope. The code would be more efficient if this line is removed.

## doubly_linked_list.rs
The doubly linked list is not possible to implement in Rust. The implementation fails because each node references both the *next* and *previous* nodes. This creates a mutual ownership scenario, where each node "owns" its adjacent nodes. This creates a problem because Rust's borrowing rules don't allow bi-directional traversal. Specifically, Rust disallows multiple mutable references to the same data simultaneously, which complicates the management of node references. 