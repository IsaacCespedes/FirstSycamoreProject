use sycamore::prelude::*;
use sycamore::suspense::Suspense;

mod random_quote;
use random_quote::quotes::get_random_quote;

#[component]
async fn RandomQuote<G: Html>(cx: Scope<'_>) -> View<G> {
    let quote = get_random_quote();
    view! { cx,
        p {
            span {
                (quote)
            }
        }
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div {
            Suspense(fallback=view! { cx, "Under maintenance..." }) {
                RandomQuote {}
            }
        }
    }
}

fn main() {
    sycamore::render(|cx| view! { cx, App {} });
}
