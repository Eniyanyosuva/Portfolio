use leptos::prelude::*;
use stylers::style;

#[component]
pub fn About() -> impl IntoView {
    let styler_class = style! { "About",
        .about-section {
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
        }

        .about-links {
            display: flex;
            gap: 1rem;
            margin-bottom: 1.5rem;
        }

        .about-link-button {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.5rem 1rem;
            background-color: transparent;
            border-radius: 4px;
            color: inherit;
            text-decoration: none;
            font-size: 1.2vmin;
            transition: background-color 0.2s, border-color 0.2s;
            cursor: pointer;
        }

        .about-link-button:hover {
            background-color: rgba(120, 120, 120, 0.1);
        }

        .button-icon {
            width: 24px;
            height: 24px;
        }

        .about-content {
            font-size: 1.3vmin;
            line-height: 1.6;
            text-align: justify;
        }

        .about-content p {
            margin-bottom: 1rem;
        }

        .about-content p:last-child {
            margin-bottom: 0;
        }
    };

    view! { class = styler_class,
        <div class="about-section">
            <div class="about-links">
                <a href="https://github.com/wyatt-avilla" class="about-link-button" target="_blank" rel="noopener noreferrer">
                    <img src="/assets/github.svg" alt="GitHub" class="button-icon" />
                </a>
                <a href="https://www.linkedin.com/in/wyatt-avilla/" class="about-link-button" target="_blank" rel="noopener noreferrer">
                    <img src="/assets/linkedin.svg" alt="LinkedIn" class="button-icon" />
                </a>
            </div>

            <div class="about-content">
                <p>
                    "I'm a graduate student focused on systems programming and backend development."
                </p>
                <p>
                    "This website is written in Rust, compiled to JavaScript with a Nix flake, and automatically deployed to GitHub pages."
                </p>
            </div>
        </div>
    }
}
