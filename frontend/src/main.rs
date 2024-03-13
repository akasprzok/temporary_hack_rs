use std::rc::Rc;
use yew::prelude::*;

mod components;
mod controllers;
mod models;
mod state;

use components::*;
use controllers::*;
use state::*;

#[function_component(App)]
fn app() -> Html {
    let repos = use_reducer(RepoState::default);
    let repos_controller = Rc::new(RepoController::new(repos.clone()));

    {
        let repos_controller = repos_controller.clone();
        use_effect_with(
            move |_| {
                repos_controller.init_repos();
                || ()
            },
            (),
        );
    }

    html! {
        <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
           <RepoList repos={ repos } />
        </div>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
