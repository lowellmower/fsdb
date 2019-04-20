## FSDB (File System Database)
FSDB is a lightweight, simple, easy to use database which uses the filesystem
of the machine for persistence. The ORM is designed to give the user the basic
operations necessary for storing and retrieving data.

### What FSDB is NOT...
FSDB is not... 
- Table driven
- Meant for use with Structured Query Language (SQL)

### What FSDB is...
FSDB is...
- Small
- Easy to use
- Transactional
- Object-relational
- Replayable

### Project Goals
- An intuitive ORM
- Easily create objects
- Easily relate objects
- Can be audited for historical state
- Optionally type unsafe (blobs)
- Fast
- Small

### Design and Notes on Implementation
You'll notice some odd function naming through out this crate and I'd like to
offer some explanation. This crate makes significant use procedural macros. If
you're familiar, procedural macros are unhygienic as noted in the [reference doc](https://doc.rust-lang.org/reference/procedural-macros.html#procedural-macro-hygiene)

```
Procedural macros are unhygienic. This means they behave as if the output token
stream was simply written inline to the code it's next to. This means that it's
affected by external items and also affects external imports.
```

In an effort to not pollute a user's space, all public functions necessary to
implement the crates behavior will be structured in such a way as to prepend
the name of the wrapping function with an `_` and version (`v1_0_0`) where the
digits follow traditional semantic versioning concepts; `Ma.Mi.P`. The user
exposed public functions are prepended with the crate name `fsdb`. Example:
```rust
// proc macro function
#[proc_macro_derive(Saveable)]
pub fn _v0_0_1_saveable(input: TokenStream) -> TokenStream {
    // user space function
    fn fsdb_save(arg: Arg) {
    }
}
```
