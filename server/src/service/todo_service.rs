use std::sync::Arc;

use todolist_core::centre::todo::TodoCentre;
use todolist_core::error::ConvertError;
use todolist_core::model::todo::Todo;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Result, Status, Streaming, async_trait};

use crate::api::v1::todo::{
    GetRequest, GetResponse, Item, RemoveItemRequest, RemoveItemResponse, SearchRequest,
    SearchResponse, TodayRequest, TodayResponse, UpsertItemRequest, UpsertItemResponse,
};
use crate::api::v1::todo_service_server::TodoService;
use crate::auth::AuthInfo;
use crate::convert::{
    centre_error_to_status, invalid_request_message, unexpected_internal_conversion_error,
};

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
        let mut auth_info = AuthInfo::try_from(&request)?;
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

    async fn remove_item(
        &self,
        _request: Request<RemoveItemRequest>,
    ) -> Result<Response<RemoveItemResponse>> {
        unimplemented!()
    }
}
