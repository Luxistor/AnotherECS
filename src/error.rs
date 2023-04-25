#[derive(Clone, Copy, PartialEq, Eq, Debug)]

pub enum ECSError {
    UseAfterFree,
    ComponentNotFound,
    EntityNotFound,
}
