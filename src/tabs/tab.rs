use leptos::prelude::*;
use strum::Display;
use stylers::style;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Tab {
    About,
    Employment,
    Education,
    Projects,
}

impl Tab {
    pub fn all() -> &'static [Tab] {
        &[Tab::About, Tab::Employment, Tab::Education, Tab::Projects]
    }
}

#[component]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn TabComponent(tab: &'static Tab, active_tab: RwSignal<&'static Tab>) -> impl IntoView {
    let styler_class = style! { "TabComponent",
        .tab {
            padding: 4px 16px;
            cursor: pointer;
            color: #FFFFFF;
            background: #0000AA;
            border-right: 2px solid #FFFFFF;
        }

        .tab.active {
            background: #AAAAAA;
            color: #000000;
        }

        .tab:hover {
            background: #5555FF;
        }
    };

    view! {
        <div
            class=format!("tab {}", styler_class)
            class:active=move || active_tab.get() == tab
            on:click=move |_| active_tab.set(tab)
        >
            {tab.to_string()}
        </div>
    }
}
