use leptos::prelude::*;
use stylers::style;

#[component]
pub fn PlantBox<'a>(name: String, scientific_name: &'a str) -> impl IntoView {
    let styler_class = style! { "PlantBox",
      section {
        text-align: center;
        color: blue;
        border: 1px solid lightgreen;
      }
    };

    view! { class = styler_class,
      <section>
        <h1>{name}</h1>
        <h2>{scientific_name.to_string()}</h2>
      </section>
    }
}
