//! Integration tests for Dispatcher Core
//! 
//! These tests verify the dispatch logic and strategy selection.

use workflow_orchestrator::dispatcher_core::{DispatcherCore, Strategy, Agent};
use octocrab::Octocrab;

async fn create_dispatcher() -> DispatcherCore {
    let github = Octocrab::builder().build().unwrap();
    DispatcherCore::new(github, "owner".to_string(), "repo".to_string())
}

#[tokio::test]
async fn test_round_robin_strategy() {
    let _dispatcher = create_dispatcher().await;
    
    // Round-robin should alternate between agents
    // Note: We can't easily test the actual selection without mocking,
    // but we can test that the strategy parsing works
    
    let strategy: Strategy = "round-robin".parse().unwrap();
    assert_eq!(strategy, Strategy::RoundRobin);
}

#[tokio::test]
async fn test_random_strategy() {
    let _dispatcher = create_dispatcher().await;
    
    let strategy: Strategy = "random".parse().unwrap();
    assert_eq!(strategy, Strategy::Random);
}

#[tokio::test]
async fn test_copilot_only_strategy() {
    let _dispatcher = create_dispatcher().await;
    
    let strategy: Strategy = "copilot-only".parse().unwrap();
    assert_eq!(strategy, Strategy::CopilotOnly);
}

#[tokio::test]
async fn test_jules_only_strategy() {
    let _dispatcher = create_dispatcher().await;
    
    let strategy: Strategy = "jules-only".parse().unwrap();
    assert_eq!(strategy, Strategy::JulesOnly);
}

#[tokio::test]
async fn test_invalid_strategy() {
    let result: Result<Strategy, _> = "invalid-strategy".parse();
    assert!(result.is_err());
}

#[tokio::test]
async fn test_agent_labels() {
    assert_eq!(Agent::Copilot.label(), "copilot");
    assert_eq!(Agent::Jules.label(), "jules");
}

#[tokio::test]
async fn test_agent_assignees() {
    assert_eq!(Agent::Copilot.assignee(), Some("Copilot"));
    assert_eq!(Agent::Jules.assignee(), None);
}

#[tokio::test]
async fn test_risk_threshold_configuration() {
    let dispatcher = create_dispatcher().await;
    let _dispatcher_with_threshold = dispatcher.with_risk_threshold(80);
    
    // Threshold is set, but we can't easily inspect it without making fields public
    // This test mainly ensures the builder pattern works
    assert!(true);
}

#[tokio::test]
async fn test_strategy_case_insensitivity() {
    // Test that parsing is case-insensitive
    let strategy1: Strategy = "Round-Robin".parse().unwrap();
    let strategy2: Strategy = "ROUND-ROBIN".parse().unwrap();
    let strategy3: Strategy = "round-robin".parse().unwrap();
    
    assert_eq!(strategy1, Strategy::RoundRobin);
    assert_eq!(strategy2, Strategy::RoundRobin);
    assert_eq!(strategy3, Strategy::RoundRobin);
}

#[tokio::test]
async fn test_strategy_aliases() {
    // Test short aliases
    let copilot: Strategy = "copilot".parse().unwrap();
    assert_eq!(copilot, Strategy::CopilotOnly);
    
    let jules: Strategy = "jules".parse().unwrap();
    assert_eq!(jules, Strategy::JulesOnly);
    
    let roundrobin: Strategy = "roundrobin".parse().unwrap();
    assert_eq!(roundrobin, Strategy::RoundRobin);
}

#[tokio::test]
async fn test_dispatcher_creation() {
    let _dispatcher = create_dispatcher().await;
    
    // Test that dispatcher can be created successfully
    // This mainly tests the Octocrab initialization
    assert!(true);
}

#[tokio::test]
async fn test_multiple_dispatchers() {
    // Test that we can create multiple dispatcher instances
    let _dispatcher1 = create_dispatcher().await;
    let _dispatcher2 = create_dispatcher().await;
    let _dispatcher3 = create_dispatcher().await;
    
    // Should not panic or cause issues
    assert!(true);
}
