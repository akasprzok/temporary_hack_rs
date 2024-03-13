use yew::UseReducerHandle;

pub struct RepoController {
    state: UseReducerHandle<RepoState>
}

impl RepoController {
    pub fn new(state: UseReducerHandle<RepoState>) -> RepoController {
        RepoController {State}
    }

    pub fn init_repos(&self) {
        let repos = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_repos = api::fetch_repos().await.unwrap();
            repos.dispatch(RepoAction::Set(fetched_repos))
        })
    }
}