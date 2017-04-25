#[derive(Debug)]
pub struct SubField {
    data: u32,
}

#[derive(Debug)]
pub struct NextAspect {
    data: String,
}

#[derive(Debug)]
struct A {
    f: SubField,
    n: NextAspect,
}

#[derive(Debug)]
struct B {
    f: SubField,
    n: NextAspect,
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

impl<'a> From<&'a A> for &'a NextAspect {
    fn from(origin: &'a A) -> Self {
        &origin.n
    }
}

impl<'a> From<&'a B> for &'a NextAspect {
    fn from(origin: &'a B) -> Self {
        &origin.n
    }
}

pub trait SubTrait {
    fn sub_fn(self) -> u32;
}

pub trait NextAspectTrait<'a> {
    fn get_next(self) -> &'a str;
}


impl<'a, T> SubTrait for T
    where T: Into<&'a SubField>
{
    fn sub_fn(self) -> u32 {
        self.into().data
    }
}

impl<'a, T> NextAspectTrait<'a> for T
    where T: Into<&'a NextAspect>
{
    fn get_next(self) -> &'a str {
        &self.into().data
    }
}
fn main() {
    let a = A {
        f: SubField { data: 10 },
        n: NextAspect { data: "hello".to_owned() },
    };
    let b = B {
        f: SubField { data: 32 },
        n: NextAspect { data: "world".to_owned() },
    };

    let x = b.sub_fn();
    println!("x = {:?}", x);
    let y = a.get_next();
    println!("y = {:?}", y);

    println!("a.data = {}", a.sub_fn());
    println!("b.data = {}", b.sub_fn());
    println!("a = {:?}", a);
    println!("b = {:?}", b);
}
