use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use chrono::{DateTime, Utc};
use common_config::RestateConfig;
use reqwest::Client;
use restate_sdk::errors::HandlerError;
use restate_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct WorkflowState(Arc<RwLock<HashMap<Uuid, WorkflowStatus>>>);

impl WorkflowState {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }

    pub async fn record(&self, status: WorkflowStatus) {
        self.0.write().await.insert(status.workflow_id, status);
    }

    pub async fn get(&self, workflow_id: Uuid) -> Option<WorkflowStatus> {
        self.0.read().await.get(&workflow_id).cloned()
    }
}

#[derive(Clone)]
pub struct WorkflowService {
    state: WorkflowState,
}

impl WorkflowService {
    pub fn new(state: WorkflowState) -> Self {
        Self { state }
    }
}

#[restate_sdk::workflow]
pub trait ProvisioningWorkflow {
    async fn run(request: Json<WorkflowRequest>) -> Result<Json<WorkflowStatus>, HandlerError>;

    #[shared]
    async fn status(
        request: Json<WorkflowStatusRequest>,
    ) -> Result<Json<Option<WorkflowStatus>>, HandlerError>;
}

impl ProvisioningWorkflow for WorkflowService {
    async fn run(
        &self,
        ctx: WorkflowContext<'_>,
        request: Json<WorkflowRequest>,
    ) -> Result<Json<WorkflowStatus>, HandlerError> {
        let workflow_id = Uuid::new_v4();
        let mut status = WorkflowStatus::new(workflow_id, WorkflowPhase::Received, request.0);
        self.state.record(status.clone()).await;

        ctx.sleep(Duration::from_millis(100)).await?;
        status = status.advance(WorkflowPhase::Scheduled);
        self.state.record(status.clone()).await;

        ctx.sleep(Duration::from_millis(100)).await?;
        status = status.advance(WorkflowPhase::Completed);
        self.state.record(status.clone()).await;

        Ok(Json(status))
    }

    async fn status(
        &self,
        _ctx: SharedWorkflowContext<'_>,
        request: Json<WorkflowStatusRequest>,
    ) -> Result<Json<Option<WorkflowStatus>>, HandlerError> {
        let query = request.0;
        Ok(Json(self.state.get(query.workflow_id).await))
    }
}

pub async fn start_workflow_server(
    config: &RestateConfig,
    service: WorkflowService,
) -> anyhow::Result<()> {
    let endpoint = Endpoint::builder().bind(service.serve()).build();
    info!(bind = %config.workflow_bind_address, "starting Restate workflow server");
    HttpServer::new(endpoint)
        .listen_and_serve(config.workflow_bind_address.parse()?)
        .await;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRequest {
    pub tenant_id: String,
    pub action: String,
    #[serde(default)]
    pub payload: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStatusRequest {
    pub workflow_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStatus {
    pub workflow_id: Uuid,
    pub phase: WorkflowPhase,
    pub updated_at: DateTime<Utc>,
    pub request: WorkflowRequest,
}

impl WorkflowStatus {
    pub fn new(workflow_id: Uuid, phase: WorkflowPhase, request: WorkflowRequest) -> Self {
        Self {
            workflow_id,
            phase,
            updated_at: Utc::now(),
            request,
        }
    }

    pub fn advance(mut self, phase: WorkflowPhase) -> Self {
        self.phase = phase;
        self.updated_at = Utc::now();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowPhase {
    Received,
    Scheduled,
    Completed,
}

#[derive(Clone)]
pub struct WorkflowClient {
    http: Client,
    base_url: String,
    namespace: String,
}

impl WorkflowClient {
    pub fn new(config: &RestateConfig) -> Self {
        Self {
            http: Client::new(),
            base_url: config.ingress_url.trim_end_matches('/').to_string(),
            namespace: config.namespace.clone(),
        }
    }

    pub async fn trigger(
        &self,
        request: &WorkflowRequest,
    ) -> Result<WorkflowStatus, WorkflowError> {
        self.post("ProvisioningWorkflow", "run", request).await
    }

    pub async fn query(&self, workflow_id: Uuid) -> Result<Option<WorkflowStatus>, WorkflowError> {
        let url = format!("{}/ProvisioningWorkflow/status", self.base_url);
        let response = self
            .http
            .post(url)
            .header("x-restate-namespace", &self.namespace)
            .json(&WorkflowStatusRequest { workflow_id })
            .send()
            .await
            .context("failed to query workflow")?;

        if response.status().is_success() {
            let status = response.json::<Option<WorkflowStatus>>().await?;
            Ok(status)
        } else if response.status() == reqwest::StatusCode::NOT_FOUND {
            Ok(None)
        } else {
            Err(WorkflowError::Remote(response.status()))
        }
    }

    async fn post<T>(
        &self,
        service: &str,
        handler: &str,
        payload: &T,
    ) -> Result<WorkflowStatus, WorkflowError>
    where
        T: Serialize + ?Sized,
    {
        let url = format!("{}/{service}/{handler}", self.base_url);
        let response = self
            .http
            .post(url)
            .header("x-restate-namespace", &self.namespace)
            .json(payload)
            .send()
            .await
            .context("failed to call Restate ingress")?;

        if response.status().is_success() {
            Ok(response.json::<WorkflowStatus>().await?)
        } else {
            Err(WorkflowError::Remote(response.status()))
        }
    }
}

#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error(transparent)]
    Transport(#[from] anyhow::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("restate returned {0}")]
    Remote(reqwest::StatusCode),
}

impl From<reqwest::Error> for WorkflowError {
    fn from(err: reqwest::Error) -> Self {
        WorkflowError::Transport(err.into())
    }
}
