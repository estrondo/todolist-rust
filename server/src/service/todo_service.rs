use std::fmt::Debug;
use std::sync::Arc;

use todolist_core::centre::todo::TodoCentre;
use todolist_core::model::todo::Todo;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Result, Status, Streaming, async_trait};
use tracing::instrument;

use crate::api::v1::todo::{
    GetRequest, GetResponse, Item, RemoveItemRequest, RemoveItemResponse, SearchRequest,
    SearchResponse, TodayRequest, TodayResponse, UpsertItemRequest, UpsertItemResponse,
};
use crate::api::v1::todo_service_server::TodoService;

use crate::auth::AuthInfo;
use crate::convert::{
    centre_error_to_status, invalid_request_message, unexpected_internal_conversion_error,
};

#[derive(Debug)]
pub struct DefaultTodoService {
    manager: Arc<dyn TodoCentre>,
}

impl DefaultTodoService {
    pub fn new(manager: Arc<dyn TodoCentre>) -> Self {
        Self { manager }
    }
}

#[async_trait]
impl TodoService for DefaultTodoService {
    type GetStream = ReceiverStream<Result<GetResponse>>;

    #[instrument(name = "grpc-default-todo-service.today", skip_all)]
    async fn today(&self, _request: Request<TodayRequest>) -> Result<Response<TodayResponse>> {
        unimplemented!()
    }

    #[instrument(name = "grpc-default-todo-service.search", skip_all)]
    async fn search(&self, _request: Request<SearchRequest>) -> Result<Response<SearchResponse>> {
        unimplemented!()
    }

    #[instrument(name = "grpc-default-todo-service.get", skip_all)]
    async fn get(
        &self,
        _request: Request<Streaming<GetRequest>>,
    ) -> Result<Response<Self::GetStream>> {
        unimplemented!()
    }

    #[instrument(name = "grpc-default-todo-service.upsert-item", skip_all)]
    async fn upsert_item(
        &self,
        request: Request<UpsertItemRequest>,
    ) -> Result<Response<UpsertItemResponse>> {
        let auth_info = AuthInfo::try_from(&request)?;
        let message = request
            .into_inner()
            .item
            .ok_or(Status::invalid_argument("Without todo item"))?;

        let todo: Todo = message
            .try_into()
            .inspect_err(|e| log::warn!("Unable to read the todo item: {}", e))
            .map_err(invalid_request_message)?;

        let user_id = auth_info.user_id().map_err(Status::from)?;

        let item: Item = self
            .manager
            .upsert(&todo, &user_id)
            .await
            .inspect_err(|e| log::error!("Unable to update a Todo: {}", e))
            .map_err(centre_error_to_status)?
            .try_into()
            .map_err(unexpected_internal_conversion_error)?;

        Result::Ok(Response::new(UpsertItemResponse {
            item: Option::Some(item),
        }))
    }

    #[instrument(name = "grpc-default-todo-service.remove-item", skip_all)]
    async fn remove_item(
        &self,
        _request: Request<RemoveItemRequest>,
    ) -> Result<Response<RemoveItemResponse>> {
        unimplemented!()
    }
}
