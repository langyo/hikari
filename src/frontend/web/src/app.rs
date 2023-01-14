use std::collections::HashMap;

use stylist::css;
use stylist::manager::StyleManager;
use stylist::yew::{styled_component, Global, ManagerProvider};
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::pages::home::Home;
use crate::pages::page_not_found::PageNotFound;

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub manager: StyleManager,
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn App() -> Html {
    let style_manager = (*use_memo(|_| StyleManager::new().unwrap(), ())).to_owned();

    html! {
        <ManagerProvider  manager={style_manager}>
            <BrowserRouter>
                <Content />
            </BrowserRouter>
        </ManagerProvider>
    }
}

#[function_component]
pub fn ServerApp(props: &AppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <ManagerProvider manager={props.manager.clone()}>
            <Router history={history}>
                <Content />
            </Router>
        </ManagerProvider>
    }
}

#[styled_component]
pub fn Content() -> Html {
    html! {
        <>
            <Global css={css!(r#"
                html, body {
                    margin: 0;
                    padding: 0;
                }
            "#)} />

            <Nav />

            <main class={css!("background: #6cf;")}>
                <Switch<Route> render={switch} />
            </main>
        </>
    }
}

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
