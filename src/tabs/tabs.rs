use leptos::prelude::*;
use stylers::style;

#[component]
pub fn Tabs(active_tab: RwSignal<&'static str>) -> impl IntoView {
    let styler_class = style! { "Tabs",
      .tabs {
          display: flex;
          background: #0000AA;
          border-bottom: 2px solid #FFFFFF;
      }

      .tab {
          padding: 4px 16px;
          cursor: pointer;
          color: #FFFFFF;
          background: #0000AA;
          border-right: 2px solid #FFFFFF;
      }

      .active {
          background: #AAAAAA;
          color: #000000;
      }

      .tab:hover {
          background: #5555FF;
      }
    };

    view! { class = styler_class,
      <div class="tabs">
          <div
              class="tab"
              class:active=move || active_tab.get() == "Main"
              on:click=move |_| active_tab.set("Main")
          >
              "Main"
          </div>
          <div
              class="tab"
              class:active=move || active_tab.get() == "Advanced"
              on:click=move |_| active_tab.set("Advanced")
          >
              "Advanced"
          </div>
          <div
              class="tab"
              class:active=move || active_tab.get() == "Security"
              on:click=move |_| active_tab.set("Security")
          >
              "Security"
          </div>
          <div
              class="tab"
              class:active=move || active_tab.get() == "Boot"
              on:click=move |_| active_tab.set("Boot")
          >
              "Boot"
          </div>
          <div
              class="tab"
              class:active=move || active_tab.get() == "Exit"
              on:click=move |_| active_tab.set("Exit")
          >
              "Exit"
          </div>
      </div>
    }
}
