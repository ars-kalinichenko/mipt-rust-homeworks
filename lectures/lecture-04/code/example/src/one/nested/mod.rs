pub mod nested2;

impl crate::one::MyTrait for nested2::Foo {}
impl nested2::Foo {}

#[allow(unused)]
pub(self) enum Count {
    Example(nested2::Foo),
}
