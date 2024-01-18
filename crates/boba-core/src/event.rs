pub trait Event: 'static {
    type Data<'a>;
}
