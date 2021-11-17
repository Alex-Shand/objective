//PROC: Start
trait Test {
    fn test(&mut self) {
    }
    fn test2(&mut self);
}

impl<T: Test + ?Sized> Test for Box<T> {
    fn test(&mut self) {
        self.as_mut().test()
    }
    fn test2(&mut self) {
        self.as_mut().test2()
    }
}
//PROC: End

fn main() {}
