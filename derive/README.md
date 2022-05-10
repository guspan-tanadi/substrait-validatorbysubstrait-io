Procedural macro crate for substrait-validator
==============================================

This crate defines some `#[derive]` macros for
[substrait-validator](https://crates.io/crates/substrait-validator),
specifically for the types generated by `prost-build`. This is needed because
`prost-build` on its own doesn't generate any introspection-like information
for the protobuf structures, such as message type names as strings, which we
want to be able to use in our parse tree.