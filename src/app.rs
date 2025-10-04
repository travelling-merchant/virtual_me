use crate::db::me;
use leptos::prelude::*;
use leptos_meta::{Link, MetaTags, Stylesheet, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                 <Link rel="icon" href="/pictures/favicon.ico"/>
                <Stylesheet id="leptos" href="/style/root.css"/>
                <Stylesheet id="homeCSS" href="/style/home.css"/>

                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
         <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
#[component]
fn HomePage() -> impl IntoView {
    let user_data = Resource::new(|| (), |_| async { me::provide_user_data().await });

    /*    let message = move || match user_data.get() {
        Some(Ok(data)) => data.current_crisis,
        Some(Err(e)) => e.to_string(),
        None => "Loading....".to_string(),
    };*/
    view! {
             <h1>"Welcome to Leptos!"</h1>
             /*<Suspense fallback=move || view! { <p>"Loading..."</p> }>
         <p>{message}</p>
     </Suspense>*/
    /*<Suspense fallback=move || view! { <p>"Loading fields..."</p> }>
        {move || user_data.get().map(|fields| match fields {
            Ok(fields) => view! {
                <div>
                    {fields.into_iter()
                        .map(|field| view! {
                            <div>
                                <strong>{field.label}:</strong> {field.value}
                            </div>
                        })
                        .collect::<Vec<_>>()
                    }
                </div>
            }.into_view(),
            Err(e) => view! { <div>{"Error: "} {e.to_string()}</div> }.into_view(),
        })}
    </Suspense>*/
    <Suspense fallback=move || view! { <p>"Loading fields..."</p> }>
            // ErrorBoundary handles any server-side errors automatically
            <ErrorBoundary
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Failed to load user data!"</p>
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                }
            >
                // When the Resource resolves successfully
                {move || {
                    user_data.get().map(|fields_opt| {
                        // fields_opt is now Vec<Field>
                        let x = me::Field{
                           label:"No data".to_string(),
                          value:"gg go next".to_string(),
                        };
                        let fields:Vec<me::Field> = fields_opt.unwrap_or(vec![x]);

                        view! {
                            <div class="user-data">
                                {fields.iter()
                                    .map(|field| view! {
                                        <div class="user-field">
                                            <p>{field.label.clone()}: </p><p> {field.value.clone()}</p>
                                        </div>
                                    })
                                    .collect_view()
                                }
                            </div>
                        }
                    })
                }}
            </ErrorBoundary>
        </Suspense>
         }
}
