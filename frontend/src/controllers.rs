use yew::UseReducerHandle;

use crate::state::{RepoAction, RepoState};
use crate::temporary_hack_api;

pub struct RepoController {
    state: UseReducerHandle<RepoState>,
}

impl RepoController {
    pub fn new(state: UseReducerHandle<RepoState>) -> RepoController {
        RepoController { state }
    }

    pub fn init_repos(&self) {
        println!("Initializing repos");
        let repos = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_repos = temporary_hack_api::fetch_repos().await.unwrap();
            repos.dispatch(RepoAction::Set(fetched_repos))
        })
    }
}
