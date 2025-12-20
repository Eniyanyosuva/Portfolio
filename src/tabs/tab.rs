use crate::tabs::{About, Education, Employment, Projects};
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

    pub fn content(self) -> impl IntoView {
        view! {
            {move || match self {
                Tab::About => About().into_any(),
                Tab::Employment => Employment().into_any(),
                Tab::Education => Education().into_any(),
                Tab::Projects => Projects().into_any(),
            }}
        }
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
            background: #787878;
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
