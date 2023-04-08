use astra_core::Component;

struct Div(String);

impl Component for Div {
    fn build(&self, id_provider: &mut impl FnMut() -> String) -> String {
        format!("<div id={}>{}</div>", id_provider(), self.0)
    }
}

#[test]
fn test_counter() {
    let app = Div("hello world".to_string());
    let mut counter = 0;
    let mut id_provider = || {
        counter += 1;
        format!("c{counter})")
    };

    insta::assert_display_snapshot!(
        app.build(&mut id_provider),
        @"<div id=c1>hello world</div>"
    );
}
