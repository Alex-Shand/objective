//PROC: Start
trait Test {
    fn test(&self) {
    }
    fn test2(&self);
}

impl<T: Test + ?Sized> Test for Box<T> {
    fn test(&self) {
        self.as_ref().test()
    }

    fn test2(&self) {
        self.as_ref().test2()
    }
}
//PROC: End

fn main() {}
