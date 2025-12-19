use crate::tabs::{Tab, TabComponent};
use leptos::prelude::*;
use stylers::style;

#[component]
pub fn Tabs(active_tab: RwSignal<&'static Tab>) -> impl IntoView {
    let styler_class = style! { "Tabs",
        .tabs {
            display: flex;
            background: #0000AA;
            border-bottom: 2px solid #FFFFFF;
        }
    };

    view! { class = styler_class,
      <div class="tabs">
          {Tab::all().iter().map(|tab| {
              view! {
                  <TabComponent tab=tab active_tab=active_tab />
              }
          }).collect_view()}
      </div>
    }
}
