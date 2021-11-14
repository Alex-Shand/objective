//PROC: Start
trait Test {
}

impl<T: Test + ?Sized> Test for Box<T> {
}
//PROC: End

fn main() {}
