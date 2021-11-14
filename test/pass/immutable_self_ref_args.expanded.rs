struct A;
struct B;
struct C;
struct D;

//PROC: Start
trait Test {
    fn test(&self, _a: A, _b: B, _c: C) -> D {
        D
    }
}

impl<T: Test + ?Sized> Test for Box<T> {
    fn test(&self, _a: A, _b: B, _c: C) -> D {
        self.as_ref().test(_a, _b, _c)
    }
}
//PROC: End

fn main() {}
