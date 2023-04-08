use astra_core::{Component, Div};

#[test]
fn test_counter() {
    let app = Div("hello world".to_string());
    let mut counter = 0;
    let mut id_provider = || {
        counter += 1;
        format!("c{counter}")
    };

    insta::assert_display_snapshot!(
        app.build(&mut id_provider),
        @"<div id=c1>hello world</div>"
    );
}
