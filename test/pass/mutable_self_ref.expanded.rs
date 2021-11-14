//PROC: Start
trait Test {
    fn test(&mut self) {
    }
}

impl<T: Test + ?Sized> Test for Box<T> {
    fn test(&mut self) {
        self.as_mut().test()
    }
}
//PROC: End

fn main() {}
