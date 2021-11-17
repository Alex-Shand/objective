struct A;
struct B;
struct C;
struct D;

#[objective::objective]
//PROC: Start
trait Test {
    fn test(&mut self, _a: A, _b: B, _c: C) -> D {
        D
    }
    fn test2(&mut self, _a: A, _b: B, _c: C) -> D;
}
//PROC: End

fn main() {}
