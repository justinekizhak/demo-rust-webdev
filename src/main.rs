use leptos::*;

fn main() {
    leptos::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class="bg-red-500"
        >
            "Click me: "
            {count}
        </button>
        <div class="text-red-500 text-3xl font-bold"> Hello world!</div>
    }
}
