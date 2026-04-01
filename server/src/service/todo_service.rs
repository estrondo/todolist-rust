use std::sync::Arc;

use todolist_core::error::{ConvertError, ManagerError};
use todolist_core::manager::TodoManager;
use todolist_core::model::Todo;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Result, Status, Streaming, async_trait};

use crate::api::v1::todo::{
    GetRequest, GetResponse, Item, RemoveItemRequest, RemoveItemResponse, SearchRequest,
    SearchResponse, TodayRequest, TodayResponse, UpsertItemRequest, UpsertItemResponse,
};
use crate::api::v1::todo_service_server::TodoService;
use crate::convert::{
    invalid_request_message, manager_error_to_status, unexpected_internal_conversion_error,
};

pub struct DefaultTodoService {
    manager: Arc<dyn TodoManager>,
}

impl DefaultTodoService {
    pub fn new(manager: Arc<dyn TodoManager>) -> Self {
        Self { manager }
    }
}

#[async_trait]
impl TodoService for DefaultTodoService {
    type GetStream = ReceiverStream<Result<GetResponse>>;

    async fn today(&self, _request: Request<TodayRequest>) -> Result<Response<TodayResponse>> {
        unimplemented!()
    }

    async fn search(&self, _request: Request<SearchRequest>) -> Result<Response<SearchResponse>> {
        unimplemented!()
    }

    async fn get(
        &self,
        _request: Request<Streaming<GetRequest>>,
    ) -> Result<Response<Self::GetStream>> {
        unimplemented!()
    }

    async fn upsert_item(
        &self,
        request: Request<UpsertItemRequest>,
    ) -> Result<Response<UpsertItemResponse>> {
        //let _auth_info = extract_auth_info(&request)?;
        let message = request
            .into_inner()
            .item
            .ok_or(Status::invalid_argument("No item to add."))?;

        let todo: Todo = message
            .try_into()
            .inspect_err(|e: &ConvertError| {
                log::warn!("Unable to read the message: {}", e.message())
            })
            .map_err(invalid_request_message)?;

        let item: Item = self
            .manager
            .upsert(&todo)
            .await
            .inspect_err(|e: &ManagerError| log::error!("Unable to upsert a Todo: {}", e.message()))
            .map_err(manager_error_to_status)?
            .try_into()
            .map_err(unexpected_internal_conversion_error)?;

        Result::Ok(Response::new(UpsertItemResponse {
            item: Option::Some(item),
        }))
    }

    async fn remove_item(
        &self,
        _request: Request<RemoveItemRequest>,
    ) -> Result<Response<RemoveItemResponse>> {
        unimplemented!()
    }
}
