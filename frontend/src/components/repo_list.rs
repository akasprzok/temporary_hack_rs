use common::model::repo::Repo;
use yew::prelude::*;

use super::RepoItem;

#[derive(Properties, PartialEq)]
pub struct RepoListProps {
    pub repos: Vec<Repo>,
}

#[function_component(RepoList)]
pub fn repo_list(RepoListProps { repos }: &RepoListProps) -> Html {
    let repos: Html = repos
        .iter()
        .map(|repo| html!(<RepoItem repo={repo.clone()} />))
        .collect();

    html!(
        <ul id="repo-list">
            {repos}
        </ul>
    )
}
