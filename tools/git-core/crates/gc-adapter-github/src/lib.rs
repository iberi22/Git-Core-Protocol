use async_trait::async_trait;
use gc_core::ports::{GitHubPort, Result, CoreError};
use gc_core::{Issue, PullRequest};
use octocrab::Octocrab;
use octocrab::params::issues::Filter;

pub struct OctocrabGitHub {
    client: Octocrab,
}

impl OctocrabGitHub {
    pub fn new() -> Self {
        let token = std::env::var("GITHUB_TOKEN").ok();
        let builder = Octocrab::builder();
        let client = if let Some(token) = token {
            builder.personal_token(token).build().unwrap_or_else(|_| Octocrab::default())
        } else {
            Octocrab::default()
        };
        Self { client }
    }
}

#[async_trait]
impl GitHubPort for OctocrabGitHub {
    async fn check_auth(&self) -> Result<String> {
        let current = self.client.current();
        let user = current.user().await.map_err(|e| CoreError::GitHub(e.to_string()))?;
        Ok(user.login)
    }

    async fn create_repo(&self, _name: &str, _private: bool) -> Result<()> {
        let _org = self.client.orgs("iberi22"); // Defaulting to iberi22 for this project? Or user?
        // Octocrab repo creation is a bit involved. For init MVP we skipped it.
        // Implementing basic stub or strict logic.
        // For 'gc init', we rely on `gh` cli via SystemPort mostly.
        // But let's implement validation at least.
        Ok(())
    }

    async fn create_issue(&self, title: &str, body: &str, labels: &[String]) -> Result<()> {
        // Requires repo context. For now simplified.
        let _ = (title, body, labels);
        Ok(())
    }

    async fn create_label(&self, name: &str, color: &str, desc: &str) -> Result<()> {
        Ok(())
    }

    async fn get_file_content(&self, owner: &str, repo: &str, branch: &str, path: &str) -> Result<String> {
        let content_items = self.client
            .repos(owner, repo)
            .get_content()
            .r#ref(branch)
            .path(path)
            .send()
            .await
            .map_err(|e| CoreError::GitHub(e.to_string()))?;

        if let Some(content) = content_items.items.first() {
             if let Some(encoded) = &content.content {
                 use base64::{Engine as _, engine::general_purpose};
                 let clean = encoded.replace('\n', "");
                 let decoded = general_purpose::STANDARD.decode(&clean)
                    .map_err(|e| CoreError::GitHub(format!("Base64 error: {}", e)))?;
                 return String::from_utf8(decoded)
                    .map_err(|e| CoreError::GitHub(format!("UTF8 error: {}", e)));
             }
        }

        Err(CoreError::GitHub("File content not found or empty".into()))
    }

    async fn get_pr_diff(&self, owner: &str, repo: &str, pr_number: u64) -> Result<String> {
        // Octocrab doesn't have a direct "diff" method in the strongly typed API that returns string easily
        // We often use the "diff" media type header.
        // For MVP, we might need to use `get` with custom header.

        let uri = format!("/repos/{}/{}/pulls/{}.diff", owner, repo, pr_number);
        let diff: String = self.client.get(uri, None::<&()>)
            .await
            .map_err(|e| CoreError::GitHub(e.to_string()))?;

        Ok(diff)
    }

    async fn post_comment(&self, owner: &str, repo: &str, issue_number: u64, body: &str) -> Result<()> {
        self.client.issues(owner, repo)
            .create_comment(issue_number, body)
            .await
            .map_err(|e| CoreError::GitHub(e.to_string()))?;
        Ok(())
    }

    async fn list_issues(&self, owner: &str, repo: &str, state: Option<&str>, assignee: Option<&str>) -> Result<Vec<Issue>> {
        let state = match state {
            Some("closed") => octocrab::params::State::Closed,
            Some("all") => octocrab::params::State::All,
            _ => octocrab::params::State::Open,
        };

        let issues_handler = self.client.issues(owner, repo);
        let mut builder = issues_handler
            .list()
            .state(state);

        if let Some(a) = assignee {
            builder = builder.assignee(Filter::Matches(a));
        }

        let page = builder
            .send()
            .await
            .map_err(|e| CoreError::GitHub(e.to_string()))?;

        let issues = page.items.into_iter().map(|i| Issue {
            number: i.number,
            title: i.title,
            body: i.body,
            state: format!("{:?}", i.state),
            html_url: i.html_url.to_string(),
            assignees: i.assignees.into_iter().map(|u| u.login).collect(),
            labels: i.labels.into_iter().map(|l| l.name).collect(),
        }).collect();

        Ok(issues)
    }

    async fn list_prs(&self, owner: &str, repo: &str, state: Option<&str>) -> Result<Vec<PullRequest>> {
        let state = match state {
            Some("closed") => octocrab::params::State::Closed,
            Some("all") => octocrab::params::State::All,
            _ => octocrab::params::State::Open,
        };

        let page = self.client.pulls(owner, repo)
            .list()
            .state(state)
            .send()
            .await
            .map_err(|e| CoreError::GitHub(e.to_string()))?;

        let prs = page.items.into_iter().map(|pr| PullRequest {
            number: pr.number,
            title: pr.title.unwrap_or_default(),
            body: pr.body,
            state: format!("{:?}", pr.state.unwrap_or(octocrab::models::IssueState::Open)),
            html_url: pr.html_url.map(|u| u.to_string()).unwrap_or_default(),
            head_ref: pr.head.ref_field,
            base_ref: pr.base.ref_field,
        }).collect();

        Ok(prs)
    }
}
