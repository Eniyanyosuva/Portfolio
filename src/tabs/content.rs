use leptos::prelude::*;
use stylers::style;

#[component]
pub fn TabContent(active_tab: RwSignal<&'static str>) -> impl IntoView {
    let styler_class = style! { "TabContent",
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

        .content {
            flex: 1;
            padding: 16px;
            overflow-y: auto;
            background: #AAAAAA;
            color: #000000;
        }
    };

    view! { class = styler_class,
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
    }
}
