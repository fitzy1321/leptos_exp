use leptos::*;

#[component]
fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="container">
                <div class="navbar-brand">
                    <a class="navbar-item"><strong>My Leptos Site</strong></a>
                </div>
            </div>
        </nav>
    }
}

#[component]
pub fn SimpleCounter(initial_value: i32) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = create_signal(initial_value);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    // create user interfaces with the declarative `view!` macro
    view! {
        <div>
            <button on:click=clear>Clear</button>
            <button on:click=decrement>-1</button>
            // text nodes can be quoted or unquoted
            <span>"Value: " {value} "!"</span>
            <button on:click=increment>+1</button>
        </div>
    }
}

// Easy to use with Trunk (trunkrs.dev) or with a simple wasm-bindgen setup
pub fn main() {
    mount_to_body(|| {
        view! {
            <Navbar />
            <SimpleCounter initial_value=3 />
        }
    })
}
