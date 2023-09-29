use crate::component::query::ComponentQuerySlice;

pub trait WorldQuery {
    const COMPONENTS: ComponentQuerySlice<'static>;
}
