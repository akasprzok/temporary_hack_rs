use super::RepoItem;
use crate::models::Repo;

#[derive(Properties, PartialEq)]
pub struct RepoListProps {
    pub repos: Vec<Repos>,
}

#[function_component(RepoList)]
pub fn repo_list(
    RepoListProps {
        repos
    }: &RepoListProps,
) -> Html {
    let repos = repos
        .iter()
        .map(|repo| html!(<RepoItem repo={repo.clone} />))
        .collect();

    html!(
        <ul id="repo-list">
            {repos}
        </ul>
    )
}