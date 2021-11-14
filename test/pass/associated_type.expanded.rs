//PROC: Start
trait Test {
    type Something;
}

impl<T: Test + ?Sized> Test for Box<T> {
    type Something = T::Something;
}
//PROC: End

fn main() {}
