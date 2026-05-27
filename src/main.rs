use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use reqwest::Client;
use tokio::net::TcpListener;
mod github;
use crate::github::{get_user_repos, get_user_data};

struct TemplateUser {
    login: String,
    name: String,
    bio: String,
    avatar_url: String,
}

struct TemplateRepo {
    name: String,
    html_url: String,
    description: String,
    language: String,
    stargazers_count: u32,
}

struct Skill {
    name: String,
    icon: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct PortfolioTemplate {
    user: TemplateUser,
    repos: Vec<TemplateRepo>,
    skills: Vec<Skill>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(render_portfolio));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn render_portfolio() -> impl IntoResponse {
    let client = Client::new();
    let github_username = "enoocdev";

    let user_data = get_user_data(&client, github_username).await.unwrap();
    let mut repos_data = get_user_repos(&client, github_username).await;
    
    repos_data.retain(|repo| !repo.fork);
    repos_data.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
    repos_data.truncate(4);

    let template_user = TemplateUser {
        login: user_data.login.clone(),
        name: user_data.name.unwrap_or(user_data.login),
        bio: user_data.bio.unwrap_or("Desarrollador de software y experto en sistemas.".to_string()),
        avatar_url: user_data.avatar_url,
    };

    let mut template_repos = Vec::new();
    for repo in repos_data {
        template_repos.push(TemplateRepo {
            name: repo.name,
            html_url: repo.html_url,
            description: repo.description.unwrap_or("Sin descripcion.".to_string()),
            language: repo.language.unwrap_or("N/A".to_string()),
            stargazers_count: repo.stargazers_count,
        });
    }

    let skills_list = vec![
        ("C++", "logos:c-plusplus"),
        ("Rust", "logos:rust"),
        ("React", "logos:react"),
        ("Python", "logos:python"),
        ("Django", "logos:django-icon"),
        ("Axum", "mdi:axum"),
        ("Java", "logos:java"),
        ("Bash", "logos:bash-icon"),
        ("Linux OS", "logos:linux-tux"),
        ("Docker", "logos:docker-icon"),
    ];

    let template_skills: Vec<Skill> = skills_list.into_iter().map(|(n, i)| Skill {
        name: n.to_string(),
        icon: i.to_string(),
    }).collect();

    PortfolioTemplate {
        user: template_user,
        repos: template_repos,
        skills: template_skills,
    }
}
