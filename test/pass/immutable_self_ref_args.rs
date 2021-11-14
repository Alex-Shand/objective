struct A;
struct B;
struct C;
struct D;

#[objective::objective]
//PROC: Start
trait Test {
    fn test(&self, _a: A, _b: B, _c: C) -> D {
        D
    }
}
//PROC: End

fn main() {}
