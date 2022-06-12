//! A collection of attribute macros to reduce boilerplate in the
//! [solders](https://github.com/kevinheavey/solders) project.
//!
//! These macros make some very specific assumptions about the structs
//! they're applied to, so they're unlikely to be useful for other projects.
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ImplItem, ItemImpl};

/// Add a `__hash__` to the impl using the `PyHash` trait.
///
/// # Example
///
/// ```rust
/// use solders_macros::pyhash;
///
/// #[derive(Debug)]
/// struct Foo(u8);
///
/// #[pyhash]
/// impl Foo {
///   pub fn pyhash(&self) -> u64 {  // Fake implementation in place of `PyHash`.
///      self.0.into()
///   }
/// }
///
/// let foo = Foo(3);
/// assert_eq!(3, foo.__hash__());
///
/// ```
#[proc_macro_attribute]
pub fn pyhash(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let to_add = quote! {pub fn __hash__(&self) -> u64 {self.pyhash()}};
    ast.items.push(ImplItem::Verbatim(to_add));
    TokenStream::from(ast.to_token_stream())
}

/// Add a `__richcmp__` to the impl using the `RichcmpFull` trait.
///
/// # Example
///
/// ```rust
/// use solders_macros::richcmp_full;
/// use pyo3::prelude::*;
/// use pyo3::pyclass::CompareOp;
///
///
/// #[derive(Debug)]
/// struct Foo(u8);
///
/// #[richcmp_full]
/// impl Foo {
///   pub fn richcmp(&self, other: &Self, op: CompareOp) -> bool {  // Fake implementation in place of `RichcmpFull`.
///      true
///   }
/// }
///
/// let foo = Foo(3);
/// assert_eq!(true, foo.__richcmp__(&foo, CompareOp::Eq));
///
/// ```
#[proc_macro_attribute]
pub fn richcmp_full(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let to_add = quote! {pub fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {self.richcmp(other, op)}};
    ast.items.push(ImplItem::Verbatim(to_add));
    TokenStream::from(ast.to_token_stream())
}

/// Add a `__richcmp__` to the impl using the `RichcmpEqualityOnly` trait.
#[proc_macro_attribute]
pub fn richcmp_eq_only(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let to_add = quote! {pub fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> pyo3::prelude::PyResult<bool> {self.richcmp(other, op)}};
    ast.items.push(ImplItem::Verbatim(to_add));
    TokenStream::from(ast.to_token_stream())
}

/// Add a `__richcmp__` to the impl using the `RichcmpSigner` trait.
#[proc_macro_attribute]
pub fn richcmp_signer(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let to_add = quote! {pub fn __richcmp__(&self, other: crate::Signer, op: pyo3::basic::CompareOp) -> pyo3::prelude::PyResult<bool> {self.richcmp(other, op)}};
    ast.items.push(ImplItem::Verbatim(to_add));
    TokenStream::from(ast.to_token_stream())
}

/// Add `__bytes__`, `__str__`, `__repr__` and `__reduce__` using the `CommonMethods` trait.
#[proc_macro_attribute]
pub fn common_magic_methods(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemImpl);
    let methods = &[
        ImplItem::Verbatim(
            quote! {pub fn __bytes__<'a>(&self, py: pyo3::prelude::Python<'a>) -> &'a pyo3::types::PyBytes  {self.pybytes(py)}},
        ),
        ImplItem::Verbatim(quote! { pub fn __str__(&self) -> String {self.pystr()} }),
        ImplItem::Verbatim(quote! { pub fn __repr__(&self) -> String {self.pyrepr()} }),
        ImplItem::Verbatim(
            quote! { pub fn __reduce__(&self) -> pyo3::prelude::PyResult<(pyo3::prelude::PyObject, pyo3::prelude::PyObject)> {self.pyreduce()} },
        ),
    ];
    ast.items.extend_from_slice(methods);
    TokenStream::from(ast.to_token_stream())
}
