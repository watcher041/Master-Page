
use yew::prelude::*;
use yew_router::prelude::*;

// 上から順にURLが含まれているか確認するため。/を上にすると全てHomeになる
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/post/:id")]
    Post { id: String },
}

// URLごとに出力するコンポーネントを選択する。
fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { 
            <h1>{ "Home" }</h1> 
        },
        Route::Post { id } => html! {
            <p>{format!("You are looking at Post {}", id)}</p>
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch
                <Route> render={Switch::render(switch)} 
            />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}