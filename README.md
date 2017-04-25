## rust aspect programming paradigm

```rust
#[derive(Debug)]
pub struct SubField {
    data: u32,
}

#[derive(Debug)]
struct A {
    f: SubField,
}

#[derive(Debug)]
struct B {
    f: SubField,
}

impl<'a> From<&'a A> for &'a SubField {
    fn from(origin: &'a A) -> Self {
        &origin.f
    }
}

impl<'a> From<&'a B> for &'a SubField {
    fn from(origin: &'a B) -> Self {
        &origin.f
    }
}

pub trait SubTrait {
    fn sub_fn(self) -> u32;
}

impl<'a, T> SubTrait for T
    where T: Into<&'a SubField>
{
    fn sub_fn(self) -> u32 {
        self.into().data
    }
}


fn main() {
    let a = A { f: SubField { data: 10 } };
    let b = B { f: SubField { data: 32 } };
    let x = b.sub_fn();
    println!("x = {:?}", x);
    println!("a.data = {}", a.sub_fn());
    println!("b.data = {}", b.sub_fn());
    println!("a = {:?}", a);
    println!("b = {:?}", b);
}

```
