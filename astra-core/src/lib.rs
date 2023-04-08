pub trait Component {
    fn build(self, id_provider: &mut impl FnMut() -> String) -> String;
}

impl Component for String {
    fn build(self, _id_provider: &mut impl FnMut() -> String) -> String {
        self
    }
}
