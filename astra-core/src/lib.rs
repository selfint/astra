pub trait Component {
    fn build(self, id_provider: &mut impl FnMut() -> String) -> String;
}

impl Component for String {
    fn build(self, _id_provider: &mut impl FnMut() -> String) -> String {
        self
    }
}

#[macro_export]
macro_rules! derive_tag_component {
    (pub $tag:tt) => {
        pub struct $tag<C>(pub C);

        impl<C: Component> Component for $tag<C> {
            fn build(self, id_provider: &mut impl FnMut() -> String) -> String {
                format!(
                    "<{} id={}>{}</{}>",
                    stringify!($tag).to_lowercase(),
                    id_provider(),
                    self.0.build(id_provider),
                    stringify!($tag).to_lowercase()
                )
            }
        }
    };
    ($tag:tt) => {
        struct $tag<C>(pub C);

        impl<C: Component> Component for $tag<C> {
            fn build(self, id_provider: &mut impl FnMut() -> String) -> String {
                format!(
                    "<{} id={}>{}</{}>",
                    stringify!($tag).to_lowercase(),
                    id_provider(),
                    self.0.build(id_provider),
                    stringify!($tag).to_lowercase()
                )
            }
        }
    };
}

derive_tag_component!(pub Div);
