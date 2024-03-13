use yew::prelude::*;

use common::model::repo::Repo;

#[derive(Properties, PartialEq)]
pub struct RepoItemProps {
    pub repo: Repo,
}

#[function_component(RepoItem)]
pub fn repo(RepoItemProps { repo }: &RepoItemProps) -> Html {
    let language = repo.language.clone().unwrap_or(String::from(""));

    html! {
        <p key={repo.name.clone()}>{format!("{}: {}", repo.name, language)}</p>
    }
}
