use gloo::utils::document;
use yew::{function_component, html, use_state, Callback, Html};

#[function_component(ThemeToggle)]
fn theme_toggle() -> Html {
    let theme = use_state(|| "light".to_string());

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let new_theme = if *theme == "light" { "dark" } else { "light" }.to_string();
            theme.set(new_theme.clone());
            if let Some(body) = document().body() {
                body.set_class_name(&new_theme);
            }
        })
    };

    html! {
        <button
            class="toggle-btn"
            onclick={toggle_theme}
            title={if *theme == "light" { "Switch to Dark Mode" } else { "Switch to Light Mode" }}
        >
            { if *theme == "light" { "🌙" } else { "☀️" } }
        </button>
    }
}

#[function_component(Content)]
fn content() -> Html {
    html! {
        <div class="content">
            <h1>{ "Hello World!" }</h1>
            <img
                src="https://rustacean.net/assets/rustacean-flat-happy.svg"
                alt="Ferris the Rust Crab"
                class="rotating-crab"
            />
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    // Inject the favicon link into the head
    document().head().map(|head| {
        let link = document().create_element("link").unwrap();
        link.set_attribute("rel", "icon").unwrap();
        link.set_attribute(
            "href",
            "https://rustacean.net/assets/rustacean-flat-happy.svg",
        )
        .unwrap();
        head.append_child(&link).unwrap();
    });

    html! {
        <>
            <div class="toggle-container">
                <ThemeToggle />
            </div>
            <Content />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
