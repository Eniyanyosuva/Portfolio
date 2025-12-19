use leptos::prelude::*;
use stylers::style;

#[component]
pub fn Header() -> impl IntoView {
    let styler_class = style! { "Header",
        .header {
            background: #0000AA;
            color: #FFFFFF;
            padding: 8px;
            text-align: center;
            font-weight: bold;
            border-bottom: 2px solid #FFFFFF;
        }
    };

    view! { class = styler_class,
      <div class="header">
        "WYATT AVILLA"
      </div>
    }
}
