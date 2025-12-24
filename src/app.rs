use crate::footer::Footer;
use crate::header::Header;
use crate::tabs::{Tab, TabContent, Tabs};
use leptos::prelude::*;
use stylers::style;

#[component]
pub fn App() -> impl IntoView {
    let active_tab = RwSignal::new(&Tab::About);

    let styler_class = style! { "App",
        @font-face {
            font-family: "Perfect DOS VGA";
            src: url("./assets/Perfect_DOS_VGA_437.ttf") format("truetype");
            font-weight: normal;
            font-style: normal;
        }

        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        :deep(html) {
            font-size: 16px;
        }

        :deep(html, body) {
            margin: 0;
            padding: 0;
            overflow: hidden;
            width: 100%;
            height: 100%;
            background: #000;
        }

        .bios-container {
            font-family: "Perfect DOS VGA", monospace;
            font-size: clamp(0.875rem, 2vw, 1.125rem);
            width: 100vw;
            height: 100vh;
            background: #0000AA;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }

        .content-box {
            width: min(80vw, 80vh);
            height: min(80vw, 80vh);
            margin: auto;
            display: flex;
            flex-direction: column;
            overflow: hidden;
            box-shadow: 1.5rem 1.5rem 0 #000000;
        }

        @media (max-width: 768px) {
            :deep(html) {
                font-size: 14px;
            }

            .bios-container {
                font-size: clamp(0.75rem, 3vw, 1rem);
            }

            .content-box {
                width: min(95vw, 95vh);
                height: min(95vw, 95vh);
                box-shadow: 0.375rem 0.375rem 0 #000000;
            }
        }

        @media (max-width: 480px) {
            :deep(html) {
                font-size: 12px;
            }

            .content-box {
                width: 100vw;
                height: calc(100vh - 4rem);
                box-shadow: none;
            }
        }
    };

    view! { class = styler_class,
        <div class="bios-container">
            <Header />

            <div class="content-box">
                <Tabs active_tab />
                <TabContent active_tab />
            </div>

            <Footer />
        </div>
    }
}
