// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use crate::Result;

/// Native async automation service trait - replaces #\[async_trait\] `AutomationService`
pub trait NativeAsyncAutomationService<
    const MAX_WORKFLOWS: usize = 1000,
    const MAX_CONCURRENT_EXECUTIONS: usize = 100,
    const EXECUTION_TIMEOUT_SECS: u64 = 300,
    const MAX_WORKFLOW_STEPS: usize = 100,
>: Send + Sync
{
    /// Type alias for WorkflowDefinition
    type WorkflowDefinition: Clone + Send + Sync + 'static;
    /// Type alias for WorkflowExecution
    type WorkflowExecution: Clone + Send + Sync + 'static;
    /// Type alias for ExecutionResult
    type ExecutionResult: Clone + Send + Sync + 'static;
    /// Create workflow - native async, no Future boxing
    fn create_workflow(
        &self,
        definition: Self::WorkflowDefinition,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Execute workflow - direct async method
    fn execute_workflow(
        &self,
        workflow_id: &str,
        parameters: std::collections::HashMap<String, serde_json::Value>,
    ) -> impl std::future::Future<Output = Result<Self::WorkflowExecution>> + Send;

    /// Stop execution - native async
    fn stop_execution(
        &self,
        execution_id: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Get execution status - no Future boxing
    fn get_execution_status(
        &self,
        execution_id: &str,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// List executions - compile-time optimization
    fn list_executions(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Self::WorkflowExecution>>> + Send;

    /// Get execution result - direct async method
    fn get_execution_result(
        &self,
        execution_id: &str,
    ) -> impl std::future::Future<Output = Result<Self::ExecutionResult>> + Send;

    /// Compile-time constants
    /// Returns the maximum number of workflows.
    #[must_use]
    fn max_workflows() -> usize {
        MAX_WORKFLOWS
    }
    /// Returns the maximum number of concurrent executions.
    #[must_use]
    fn max_concurrent_executions() -> usize {
        MAX_CONCURRENT_EXECUTIONS
    }
    /// Returns the execution timeout in seconds.
    #[must_use]
    fn execution_timeout_seconds() -> u64 {
        EXECUTION_TIMEOUT_SECS
    }
    /// Returns the maximum number of workflow steps.
    #[must_use]
    fn max_workflow_steps() -> usize {
        MAX_WORKFLOW_STEPS
    }
}

/// Native async workflow service trait - replaces #\[async_trait\] `WorkflowService`
pub trait NativeAsyncWorkflowService<
    const MAX_WORKFLOWS: usize = 1000,
    const EXECUTION_TIMEOUT_SECS: u64 = 300,
>: Send + Sync
{
    /// Type alias for Workflow
    type Workflow: Clone + Send + Sync + 'static;
    /// Type alias for ExecutionContext
    type ExecutionContext: Clone + Send + Sync + 'static;
    /// Execute workflow - native async, no Future boxing
    fn execute(
        &self,
        workflow: Self::Workflow,
    ) -> impl std::future::Future<Output = Result<Self::ExecutionContext>> + Send;

    /// Get execution status - direct async method
    fn get_status(
        &self,
        execution_id: &str,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Compile-time constants
    /// Returns the maximum number of workflows.
    #[must_use]
    fn max_workflows() -> usize {
        MAX_WORKFLOWS
    }
    /// Returns the execution timeout in seconds.
    #[must_use]
    fn execution_timeout_seconds() -> u64 {
        EXECUTION_TIMEOUT_SECS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_service_constants() {
        struct MockWorkflow;
        impl NativeAsyncWorkflowService<50, 60> for MockWorkflow {
            type Workflow = String;
            type ExecutionContext = String;

            fn execute(
                &self,
                _workflow: Self::Workflow,
            ) -> impl std::future::Future<Output = Result<Self::ExecutionContext>> + Send
            {
                std::future::ready(Ok("ctx".to_string()))
            }
            fn get_status(
                &self,
                _execution_id: &str,
            ) -> impl std::future::Future<Output = Result<String>> + Send {
                std::future::ready(Ok("running".to_string()))
            }
        }
        assert_eq!(MockWorkflow::max_workflows(), 50);
        assert_eq!(MockWorkflow::execution_timeout_seconds(), 60);
    }

    struct MockAutomation;

    impl NativeAsyncAutomationService<20, 5, 120, 50> for MockAutomation {
        type WorkflowDefinition = String;
        type WorkflowExecution = String;
        type ExecutionResult = String;

        fn create_workflow(
            &self,
            _definition: Self::WorkflowDefinition,
        ) -> impl std::future::Future<Output = Result<String>> + Send {
            std::future::ready(Ok("wf".into()))
        }
        fn execute_workflow(
            &self,
            _workflow_id: &str,
            _parameters: std::collections::HashMap<String, serde_json::Value>,
        ) -> impl std::future::Future<Output = Result<Self::WorkflowExecution>> + Send {
            std::future::ready(Ok("ex".into()))
        }
        fn stop_execution(
            &self,
            _execution_id: &str,
        ) -> impl std::future::Future<Output = Result<()>> + Send {
            std::future::ready(Ok(()))
        }
        fn get_execution_status(
            &self,
            _execution_id: &str,
        ) -> impl std::future::Future<Output = Result<String>> + Send {
            std::future::ready(Ok("running".into()))
        }
        fn list_executions(
            &self,
        ) -> impl std::future::Future<Output = Result<Vec<Self::WorkflowExecution>>> + Send
        {
            std::future::ready(Ok(vec![]))
        }
        fn get_execution_result(
            &self,
            _execution_id: &str,
        ) -> impl std::future::Future<Output = Result<Self::ExecutionResult>> + Send {
            std::future::ready(Ok("done".into()))
        }
    }

    #[test]
    fn automation_service_constants() {
        assert_eq!(MockAutomation::max_workflows(), 20);
        assert_eq!(MockAutomation::max_concurrent_executions(), 5);
        assert_eq!(MockAutomation::execution_timeout_seconds(), 120);
        assert_eq!(MockAutomation::max_workflow_steps(), 50);
    }

    #[tokio::test]
    async fn automation_service_async_methods() {
        let a = MockAutomation;
        a.create_workflow("d".into()).await.expect("create");
        a.execute_workflow("w", std::collections::HashMap::new())
            .await
            .expect("exec");
        a.stop_execution("e").await.expect("stop");
        a.get_execution_status("e").await.expect("st");
        a.list_executions().await.expect("list");
        a.get_execution_result("e").await.expect("res");
    }
}
