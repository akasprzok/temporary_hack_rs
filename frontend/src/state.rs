use yew::Reducible;

use crate::models::Repo;

pub enum RepoAction {
    Set(Vec<Repo>),
}

pub struct RepoState {
    pub repos: Vec<Repo>,
}

impl Default for RepoState {
    fn default() -> Self {
        Self { repos: vec![] }
    }
}

impl Reducible for RepoState {
    type Action = RepoAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_repos = match action {
            RepoAction::Set(repos) => repos,
        };

        Self { repos: next_repos }.into()
    }
}
