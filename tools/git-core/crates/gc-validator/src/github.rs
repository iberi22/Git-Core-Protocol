//! GitHub API client with parallel execution support

use anyhow::{Result, Context};
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Semaphore;
use futures::future::join_all;
use tracing::{info, debug, warn};

/// GitHub API client with rate limiting and parallel execution
pub struct GitHubClient {
    client: Client,
    repo: String,
    owner: String,
    semaphore: Arc<Semaphore>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WorkflowRun {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub run_attempt: u32,
    pub workflow_id: u64,
    pub head_branch: String,
    pub head_sha: String,
}

#[derive(Debug, Deserialize)]
pub struct WorkflowRunsResponse {
    pub total_count: u32,
    pub workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Job {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub steps: Option<Vec<Step>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Step {
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub number: u32,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JobsResponse {
    pub total_count: u32,
    pub jobs: Vec<Job>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Workflow {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct WorkflowsResponse {
    pub total_count: u32,
    pub workflows: Vec<Workflow>,
}

#[derive(Debug, Serialize)]
pub struct CreatePRRequest {
    pub title: String,
    pub body: String,
    pub head: String,
    pub base: String,
    pub draft: bool,
}

#[derive(Debug, Deserialize)]
pub struct PRResponse {
    pub number: u64,
    pub html_url: String,
}

#[derive(Debug, Serialize)]
pub struct CreateIssueCommentRequest {
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct WorkflowAnalysis {
    pub run: WorkflowRun,
    pub jobs: Vec<Job>,
    pub logs: Option<String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub duration_seconds: Option<i64>,
}

impl GitHubClient {
    pub fn new(token: &str, repo: &str, max_parallel: usize) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(
            "X-GitHub-Api-Version",
            header::HeaderValue::from_static("2022-11-28"),
        );
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("workflow-orchestrator/1.0"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        let parts: Vec<&str> = repo.split('/').collect();
        let (owner, repo_name) = if parts.len() == 2 {
            (parts[0].to_string(), parts[1].to_string())
        } else {
            panic!("Invalid repo format. Expected owner/repo");
        };

        Self {
            client,
            repo: repo_name,
            owner,
            semaphore: Arc::new(Semaphore::new(max_parallel)),
        }
    }

    fn api_url(&self, path: &str) -> String {
        format!("https://api.github.com/repos/{}/{}{}", self.owner, self.repo, path)
    }

    /// Get all workflow runs with parallel job fetching
    pub async fn get_workflow_runs(&self, per_page: u32) -> Result<Vec<WorkflowRun>> {
        let url = self.api_url(&format!("/actions/runs?per_page={}", per_page));

        let response: WorkflowRunsResponse = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        info!("📊 Found {} workflow runs", response.total_count);
        Ok(response.workflow_runs)
    }

    /// Get jobs for a workflow run
    pub async fn get_jobs(&self, run_id: u64) -> Result<Vec<Job>> {
        let _permit = self.semaphore.acquire().await?;

        let url = self.api_url(&format!("/actions/runs/{}/jobs", run_id));

        let response: JobsResponse = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.jobs)
    }

    /// Get all workflows
    pub async fn get_workflows(&self) -> Result<Vec<Workflow>> {
        let url = self.api_url("/actions/workflows");

        let response: WorkflowsResponse = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.workflows)
    }

    /// Get logs for a job (returns URL, actual download is separate)
    pub async fn get_job_logs(&self, job_id: u64) -> Result<String> {
        let _permit = self.semaphore.acquire().await?;

        let url = self.api_url(&format!("/actions/jobs/{}/logs", job_id));

        let response = self.client
            .get(&url)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Ok(String::new())
        }
    }

    /// Analyze multiple workflow runs in parallel
    pub async fn analyze_runs_parallel(&self, runs: Vec<WorkflowRun>) -> Result<Vec<WorkflowAnalysis>> {
        info!("🔄 Analyzing {} runs in parallel...", runs.len());

        let futures: Vec<_> = runs.into_iter().map(|run| {
            let client = self.clone_minimal();
            async move {
                client.analyze_single_run(run).await
            }
        }).collect();

        let results = join_all(futures).await;

        let analyses: Vec<WorkflowAnalysis> = results
            .into_iter()
            .filter_map(|r| r.ok())
            .collect();

        info!("✅ Analyzed {} runs successfully", analyses.len());
        Ok(analyses)
    }

    /// Analyze a single workflow run
    async fn analyze_single_run(&self, run: WorkflowRun) -> Result<WorkflowAnalysis> {
        debug!("Analyzing run #{}: {}", run.id, run.name);

        let jobs = self.get_jobs(run.id).await.unwrap_or_default();

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Collect errors and warnings from jobs
        for job in &jobs {
            if job.conclusion.as_deref() == Some("failure") {
                errors.push(format!("Job '{}' failed", job.name));

                if let Some(steps) = &job.steps {
                    for step in steps {
                        if step.conclusion.as_deref() == Some("failure") {
                            errors.push(format!("  - Step '{}' failed", step.name));
                        }
                    }
                }
            }

            if job.conclusion.as_deref() == Some("cancelled") {
                warnings.push(format!("Job '{}' was cancelled", job.name));
            }
        }

        // Calculate duration
        let duration_seconds = if let (Some(start), Some(end)) = (
            jobs.first().and_then(|j| j.started_at.as_ref()),
            jobs.last().and_then(|j| j.completed_at.as_ref()),
        ) {
            chrono::DateTime::parse_from_rfc3339(end)
                .ok()
                .and_then(|e| chrono::DateTime::parse_from_rfc3339(start).ok().map(|s| (e - s).num_seconds()))
        } else {
            None
        };

        Ok(WorkflowAnalysis {
            run,
            jobs,
            logs: None, // Fetch on demand to save API calls
            errors,
            warnings,
            duration_seconds,
        })
    }

    /// Create a PR with validation results
    pub async fn create_pr(&self, title: &str, body: &str, branch: &str) -> Result<PRResponse> {
        let url = self.api_url("/pulls");

        let request = CreatePRRequest {
            title: title.to_string(),
            body: body.to_string(),
            head: branch.to_string(),
            base: "main".to_string(),
            draft: false,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Add comment to PR
    pub async fn add_pr_comment(&self, pr_number: u64, body: &str) -> Result<()> {
        let url = self.api_url(&format!("/issues/{}/comments", pr_number));

        let request = CreateIssueCommentRequest {
            body: body.to_string(),
        };

        self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        Ok(())
    }

    /// Create a branch
    pub async fn create_branch(&self, branch_name: &str, from_sha: &str) -> Result<()> {
        let url = format!("https://api.github.com/repos/{}/{}/git/refs", self.owner, self.repo);

        let body = serde_json::json!({
            "ref": format!("refs/heads/{}", branch_name),
            "sha": from_sha
        });

        self.client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        Ok(())
    }

    /// Get default branch SHA
    pub async fn get_default_branch_sha(&self) -> Result<String> {
        let url = self.api_url("/git/refs/heads/main");

        #[derive(Deserialize)]
        struct RefResponse {
            object: RefObject,
        }

        #[derive(Deserialize)]
        struct RefObject {
            sha: String,
        }

        let response: RefResponse = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.object.sha)
    }

    fn clone_minimal(&self) -> Self {
        Self {
            client: self.client.clone(),
            repo: self.repo.clone(),
            owner: self.owner.clone(),
            semaphore: self.semaphore.clone(),
        }
    }
}
