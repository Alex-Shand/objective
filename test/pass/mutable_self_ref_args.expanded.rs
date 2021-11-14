struct A;
struct B;
struct C;
struct D;

//PROC: Start
trait Test {
    fn test(&mut self, _a: A, _b: B, _c: C) -> D {
        D
    }
}

impl<T: Test + ?Sized> Test for Box<T> {
    fn test(&mut self, _a: A, _b: B, _c: C) -> D {
        self.as_mut().test(_a, _b, _c)
    }
}
//PROC: End

fn main() {}
