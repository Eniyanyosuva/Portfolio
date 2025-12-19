use super::plantbox::PlantBox;
use leptos::prelude::*;
use stylers::style;

#[component]
pub fn App() -> impl IntoView {
    let styler_class = style! { "App",
      div {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        border: 1px solid pink;
        margin: 25px;
      }
    };

    view! { class = styler_class,
      <div>
        <PlantBox name="Plant Name 1".to_string() scientific_name=&"scientific name 1" />
        <PlantBox name="Plant Name 2".to_string() scientific_name=&"scientific name 2" />
      </div>
    }
}
