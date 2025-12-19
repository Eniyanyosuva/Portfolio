use crate::header::Header;
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

        .content {
            flex: 1;
            padding: 16px;
            overflow-y: auto;
            background: #AAAAAA;
            color: #000000;
        }

        .footer {
            background: #0000AA;
            color: #FFFFFF;
            padding: 8px 16px;
            border-top: 2px solid #FFFFFF;
            font-size: 12px;
            display: flex;
            justify-content: space-between;
        }

        .warning {
            color: #FFFF00;
            margin-bottom: 12px;
        }

        .menu-item {
            padding: 4px 8px;
            margin: 2px 0;
            cursor: pointer;
        }

        .menu-item:hover {
            background: #5555FF;
            color: #FFFFFF;
        }
    };

    view! { class = styler_class,
        <div class="bios-container">
            <Header />

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

            <div class="content">
                {move || match active_tab.get() {
                    "Main" => view! {
                        <div>
                            <div class="warning">"WARNING: Setting wrong values in below sections"</div>
                            <div class="warning">"         may cause system to malfunction."</div>
                            <br/>
                            <div class="menu-item">"► System Information"</div>
                            <div class="menu-item">"► System Time"</div>
                            <div class="menu-item">"► System Date"</div>
                        </div>
                    }.into_any(),
                    "Advanced" => view! {
                        <div>
                            <div class="warning">"WARNING: Setting wrong values in below sections"</div>
                            <div class="warning">"         may cause system to malfunction."</div>
                            <br/>
                            <div class="menu-item">"► Boot Features"</div>
                            <div class="menu-item">"► Processor & Clock Options"</div>
                            <div class="menu-item">"► Advanced Chipset Control"</div>
                            <div class="menu-item">"► I/O Virtualization"</div>
                        </div>
                    }.into_any(),
                    "Security" => view! {
                        <div>
                            <div class="menu-item">"► Administrator Password"</div>
                            <div class="menu-item">"► User Password"</div>
                            <div class="menu-item">"► Secure Boot Configuration"</div>
                        </div>
                    }.into_any(),
                    "Boot" => view! {
                        <div>
                            <div class="menu-item">"► Boot Device Priority"</div>
                            <div class="menu-item">"► Hard Disk Drives"</div>
                            <div class="menu-item">"► CD/DVD Drives"</div>
                        </div>
                    }.into_any(),
                    "Exit" => view! {
                        <div>
                            <div class="menu-item">"► Exit Saving Changes"</div>
                            <div class="menu-item">"► Exit Discarding Changes"</div>
                            <div class="menu-item">"► Load Setup Defaults"</div>
                        </div>
                    }.into_any(),
                    _ => view! { <div>"Unknown tab"</div> }.into_any(),
                }}
            </div>

            <div class="footer">
                <span>"v02.61 (C)Copyright 1985-2006, American Megatrends, Inc."</span>
                <span>"↑↓: Select Item | Enter: Go to Sub Screen | F10: Save and Exit | ESC: Exit"</span>
            </div>
        </div>
    }
}
