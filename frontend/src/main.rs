use std::rc::Rc;
use yew::prelude::*;

mod components;
mod controllers;
mod state;
mod temporary_hack_api;

use components::*;
use controllers::*;
use state::*;

#[function_component(App)]
fn app() -> Html {
    let repos = use_reducer(RepoState::default);
    let repos_controller = Rc::new(RepoController::new(repos.clone()));

    {
        let repos_controller = repos_controller.clone();
        use_effect_with((), move |_| {
            repos_controller.init_repos();
            || ()
        });
    }

    html! {
        <>
        <h1>{ "Repos" }</h1>
        <div>
           <RepoList repos={ repos.repos.clone() } />
        </div>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
