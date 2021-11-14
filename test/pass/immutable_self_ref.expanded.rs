//PROC: Start
trait Test {
    fn test(&self) {
    }
}

impl<T: Test + ?Sized> Test for Box<T> {
    fn test(&self) {
        self.as_ref().test()
    }
}
//PROC: End

fn main() {}
