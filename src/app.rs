use crate::footer::Footer;
use crate::header::Header;
use crate::tabs::{TabContent, Tabs};
use leptos::prelude::*;
use stylers::style;

#[component]
#[allow(clippy::too_many_lines)]
pub fn App() -> impl IntoView {
    let active_tab = RwSignal::new("Main");

    let styler_class = style! { "App",
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            background: #000;
            font-family: "Courier New", monospace;
            color: #fff;
        }

        .bios-container {
            width: 100vw;
            height: 100vh;
            background: #0000AA;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }
    };

    view! { class = styler_class,
        <div class="bios-container">
            <Header />

            <Tabs active_tab />

            <TabContent active_tab />

            <Footer />
        </div>
    }
}
