use yew::prelude::*;

use crate::models::Repo;

#[derive(Properties, PartialEq)]
pub struct RepoItemProps {
    pub repo: Repo,
}

#[function_component(RepoItem)]
pub fn repo(RepoItemProps { repo }: &RepoItemProps) -> Html {
    html! {
        <p key={repo.name.clone()}>{format!("{}: {}", repo.name, repo.language)}</p>
    }
}
