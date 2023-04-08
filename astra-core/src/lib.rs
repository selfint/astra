pub trait Component {
    fn build(&self, id_provider: &mut impl FnMut() -> String) -> String;
}
