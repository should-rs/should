pub trait ShouldResult<T: ?Sized> {
    fn should_be_ok(&self);
    fn should_be_err(&self);
}