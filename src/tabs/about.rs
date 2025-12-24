use leptos::prelude::*;
use stylers::style;

#[component]
pub fn About() -> impl IntoView {
    let styler_class = style! { "About",
        .about-section {
            max-width: 50rem;
            margin: 0 auto;
            padding: 1.5rem;
        }

        .about-links {
            display: flex;
            gap: 0.5rem;
            margin-bottom: 1.5rem;
            flex-wrap: wrap;
        }

        .about-link-button {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.5rem;
            background-color: transparent;
            border: 0.125rem solid #000000;
            box-shadow: 0.25rem 0.25rem 0 #000000;
            color: inherit;
            text-decoration: none;
            font-size: 1rem;
            transition: all 0.1s;
            cursor: pointer;
            position: relative;
        }

        .about-link-button:hover {
            background-color: rgba(120, 120, 120, 0.1);
            transform: translate(0.125rem, 0.125rem);
            box-shadow: 0.125rem 0.125rem 0 #000000;
        }

        .about-link-button:active {
            transform: translate(0.25rem, 0.25rem);
            box-shadow: none;
        }

        .button-icon {
            width: 1.5rem;
            height: 1.5rem;
        }

        .about-content {
            font-size: 1rem;
            line-height: 1.6;
            text-align: left;
        }

        .about-content p {
            margin-bottom: 1rem;
        }

        .about-content p:last-child {
            margin-bottom: 0;
        }

        @media (max-width: 768px) {
            .about-section {
                padding: 1rem;
            }

            .about-link-button {
                padding: 0.4rem 0.8rem;
                font-size: 0.9rem;
            }

            .button-icon {
                width: 1.25rem;
                height: 1.25rem;
            }

            .about-content {
                font-size: 0.9rem;
                text-align: left;
            }
        }

        @media (max-width: 480px) {
            .about-section {
                padding: 0.75rem;
            }

            .about-links {
                gap: 0.4rem;
                margin-bottom: 1rem;
            }

            .about-link-button {
                padding: 0.35rem 0.7rem;
                font-size: 0.85rem;
            }

            .about-content {
                font-size: 0.85rem;
                line-height: 1.5;
            }
        }
    };

    view! { class = styler_class,
        <div class="about-section">
            <div class="about-links">
                <a href="https://github.com/wyatt-avilla" class="about-link-button" target="_blank" rel="noopener noreferrer">
                    <img src="./assets/github.svg" alt="GitHub" class="button-icon" />
                </a>
                <a href="https://www.linkedin.com/in/wyatt-avilla/" class="about-link-button" target="_blank" rel="noopener noreferrer">
                    <img src="./assets/linkedin.svg" alt="LinkedIn" class="button-icon" />
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
