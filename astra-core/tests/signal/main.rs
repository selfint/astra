use std::ops::{Deref, DerefMut};

use serde::Serialize;

fn build_app<I, C>(root: C, id_provider: &mut I) -> String
where
    I: FnMut() -> String,
    C: Component<I>,
{
    let (style, html, script) = root.build(id_provider);

    format!(
        r###"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Document</title>
  </head>
  <body>
    <style>
    {}
    </style>

    {}

    <script>
    {}
    </script>
  </body>
</html>"###,
        *style.unwrap_or_else(|| Style("".to_string())),
        *html,
        *script.unwrap_or_else(|| Script("".to_string())),
    )
}

struct Style(String);

impl Deref for Style {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Style {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct Html(String);

impl Deref for Html {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Html {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct Script(String);

impl Deref for Script {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Script {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

type Element = (Option<Style>, Html, Option<Script>);
trait Component<I: FnMut() -> String> {
    fn build(self, id_provider: &mut I) -> Element;
}

impl<I, F> Component<I> for F
where
    I: FnMut() -> String,
    F: Fn(&mut I, &mut Script) -> String,
{
    fn build(self, id_provider: &mut I) -> Element {
        let mut script = Script(String::new());
        let html = (self)(id_provider, &mut script);

        (None, Html(html), Some(script))
    }
}

fn counter(id_provider: &mut impl FnMut() -> String, script: &mut Script) -> String {
    let (count, set_count) = create_signal(&0, id_provider, script);

    let button_id = (id_provider)();

    let button = format!(r#"<button id={button_id}>{count}</button>"#);

    let update_count = set_count(&1);

    script.push_str(&format!(
        r#"
{button_id}.addEventListener("click", () => {{
    {update_count}
}});
    "#,
    ));

    button
}

fn create_signal<T: Serialize>(
    value: &T,
    id_provider: &mut impl FnMut() -> String,
    script: &mut Script,
) -> (String, impl Fn(&T) -> String) {
    let initial_value = serde_json::to_string(value).expect("failed to serialize value");
    let var = (id_provider)();
    let var_view_id = format!("{}View", var);
    let set_fn_name = format!("set{}", var);

    script.push_str(&format!(
        r#"
let {var} = {initial_value};

function {set_fn_name}(newValue) {{
    {var} = newValue;
    {var_view_id}.innerHTML = {var};
}}
    "#
    ));

    (
        format!(r#"<span id={}>{}</span>"#, var_view_id, initial_value),
        move |new_value| {
            let new_value = serde_json::to_string(new_value).expect("failed to serialize value");

            format!("{set_fn_name}({new_value});")
        },
    )
}

#[test]
fn test_signal() {
    let mut count = 0;
    let app = build_app(counter, &mut || {
        count += 1;
        format!("count{count}")
    });

    insta::assert_display_snapshot!(app,
        @r###"
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="UTF-8" />
        <meta http-equiv="X-UA-Compatible" content="IE=edge" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Document</title>
      </head>
      <body>
        <style>
        
        </style>

        <button id=count2><span id=count1View>0</span></button>

        <script>
        
    let count1 = 0;

    function setcount1(newValue) {
        count1 = newValue;
        count1View.innerHTML = count1;
    }
        
    count2.addEventListener("click", () => {
        setcount1(1);
    });
        
        </script>
      </body>
    </html>
    "###
    );
}
