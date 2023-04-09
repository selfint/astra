use std::ops::{Deref, DerefMut};

use serde::Serialize;

fn build_app<C: Component>(root: C) -> String {
    let mut count = 0;
    let (style, html, script) = root.build(|| {
        count += 1;
        format!("count{}", count)
    });

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
trait Component {
    fn build(self, id_provider: impl FnMut() -> String) -> Element;
}

impl<F: Fn(&mut Script) -> String> Component for F {
    fn build(self, id_provider: impl FnMut() -> String) -> Element {
        let mut script = Script(String::new());
        let html = (self)(&mut script);

        (None, Html(html), Some(script))
    }
}

fn counter(script: &mut Script) -> String {
    r#"<button>0</button>"#.to_string()
}

fn create_signal<T: Serialize>(id: &str, value: T) -> (String, String) {
    todo!()
}

#[test]
fn test_signal() {
    let app = build_app(counter);

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

        <button id="counter">0</button>

        <script>
        
        </script>
      </body>
    </html>
    "###
    );
}
