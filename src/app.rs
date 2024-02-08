use std::time::Duration;

use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (app_stat, set_app_stat) = create_signal(0);
    // the top level resource works as expected
    let top_level_resource = create_resource(
        move || app_stat.get(),
        move |v| async move {
            #[cfg(feature = "ssr")]
            {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
            #[cfg(feature = "hydrate")]
            {
                gloo_timers::future::sleep(Duration::from_secs(2)).await;
            }
            v
        },
    );

    let (show_sub_page, set_show_sub_page) = create_signal(false);

    view! {
        <div>
            <h1>"Hello, World!"</h1>
            <Transition
                fallback=move || view! { <p>"Loading..."</p> }
            >
                <div>
                    {
                        move || {
                            top_level_resource.get().map(|v| {
                                view! {
                                    <p>"this works fine! there will be no fallback after initial load"{v}</p>
                                    <button
                                        on:click=move |_| {
                                            set_app_stat.update(|v| *v += 1);
                                        }
                                    >
                                        "Increment"
                                    </button>
                                }

                            })
                        }
                    }
                </div>
            </Transition>
        </div>
        <div>
                    <button
                        on:click=move |_| {
                            set_show_sub_page.update(|v| *v= !*v);
                        }
                    >
                        "Toggle Sub Page"
                    </button>
                    <div>
                        {
                            move || {
                                if show_sub_page.get() {
                                    view! {
                                        <SubPage />
                                    }.into_view()
                                } else {
                                    view! {
                                        <p>"Sub Page is not shown"</p>
                                    }.into_view()
                                }
                            }
                        }
                    </div>
        </div>
    }
}

#[component]
fn SubPage() -> impl IntoView {
    let (app_stat, set_app_stat) = create_signal(0);
    // the top level resource works as expected
    let sub_page_resource = create_resource(
        move || app_stat.get(),
        move |v| async move {
            #[cfg(feature = "ssr")]
            {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
            #[cfg(feature = "hydrate")]
            {
                gloo_timers::future::sleep(Duration::from_secs(2)).await;
            }
            v
        },
    );
    view! {
        <div>
            <h1>"subpage!"</h1>
            <Transition
                fallback=move || view! { <p>"Loading..."</p> }
            >
                <div>
                    {
                        move || {
                            sub_page_resource.get().map(|v| {
                                view! {
                                    <p>"bug here!, every time updated, it will show loading!"{v}</p>
                                    <button
                                        on:click=move |_| {
                                            set_app_stat.update(|v| *v += 1);
                                        }
                                    >
                                        "Increment"
                                    </button>
                                }

                            })
                        }
                    }
                </div>
            </Transition>
        </div>
    }
}
