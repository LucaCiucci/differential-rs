
pub trait DiffIndex {
    fn into_orders(self) -> impl Iterator<Item = usize> + Clone;
}

impl<I, U: Usize> DiffIndex for I
where
    I: IntoIterator<Item = U>,
    I::IntoIter: Clone,
{
    fn into_orders(self) -> impl Iterator<Item = usize> + Clone {
        let it: <I as IntoIterator>::IntoIter = self.into_iter();
        it.map(|x: U| x.value())
    }
}

trait Usize {
    fn value(&self) -> usize;
}

impl Usize for usize {
    fn value(&self) -> usize {
        *self
    }
}

impl Usize for &usize {
    fn value(&self) -> usize {
        **self
    }
}

/*
~\Documents\GitHub\differential-rs> cargo run --example b --release                                                                                                                                  03/07/24 23:00:30 PM
   Compiling differential v0.1.0 (C:\Users\lucac\Documents\GitHub\differential-rs)
error[E0275]: overflow evaluating the requirement `{closure@all::check<usize, {closure@differential::utils::offset_of_impl<Map<Map<Map<Map<..., ...>, ...>, ...>, ...>>::{closure#0}}>::{closure#0}}: Copy`
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`b`)
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^
note: required because it's used within this closure
  --> C:\Users\lucac\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\map.rs:92:5
   |
92 |     move |acc, elt| g(acc, f(elt))
   |     ^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0275`.
error: could not compile `differential` (example "b") due to 1 previous error
*/