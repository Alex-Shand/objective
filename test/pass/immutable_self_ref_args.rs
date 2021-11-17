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

    fn test2(&self, _a: A, _b: B, _c: C) -> D;
}
//PROC: End

fn main() {}
