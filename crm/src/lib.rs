mod abi;
mod config;
pub mod pb;

pub use config::AppConfig;

use anyhow::Result;
use crm_metadata::pb::metadata_client::MetadataClient;
use crm_send::pb::notification_client::NotificationClient;
use pb::{
  crm_server::{Crm, CrmServer},
  RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest, WelcomeResponse,
};
use tonic::{async_trait, transport::Channel, Request, Response, Status};
use user_stat::pb::user_stats_client::UserStatsClient;

pub struct CrmService {
  config: AppConfig,
  metadata: MetadataClient<Channel>,
  user_stats: UserStatsClient<Channel>,
  notification: NotificationClient<Channel>,
}

#[async_trait]
impl Crm for CrmService {
  async fn welcome(
    &self,
    request: Request<WelcomeRequest>,
  ) -> Result<Response<WelcomeResponse>, Status> {
    self.welcome(request.into_inner()).await
  }

  async fn remind(
    &self,
    _request: Request<RemindRequest>,
  ) -> Result<Response<RemindResponse>, Status> {
    todo!()
  }

  async fn recall(
    &self,
    _request: Request<RecallRequest>,
  ) -> Result<Response<RecallResponse>, Status> {
    todo!()
  }
}

impl CrmService {
  pub async fn try_new(config: AppConfig) -> Result<Self> {
    let metadata = MetadataClient::connect(config.server.metadata.clone()).await?;
    let user_stats = UserStatsClient::connect(config.server.user_stats.clone()).await?;
    let notification = NotificationClient::connect(config.server.notification.clone()).await?;
    Ok(Self {
      config,
      metadata,
      user_stats,
      notification,
    })
  }

  pub fn into_server(self) -> CrmServer<Self> {
    CrmServer::new(self)
  }
}
