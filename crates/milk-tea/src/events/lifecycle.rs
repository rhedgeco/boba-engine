use boba_core::Event;

pub struct MilkTeaStart(());
impl Event for MilkTeaStart {
    type Data<'a> = ();
}

pub struct MilkTeaUpdate(());
impl Event for MilkTeaUpdate {
    type Data<'a> = ();
}

pub struct MilkTeaExit(());
impl Event for MilkTeaExit {
    type Data<'a> = ();
}
