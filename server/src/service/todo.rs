use std::pin::Pin;

use tokio_stream::Stream;
use tonic::{Request, Response, Result, Streaming, async_trait};

use crate::api::v1::todo::{
    GetRequest, GetResponse, RemoveItemRequest, RemoveItemResponse, SearchRequest, SearchResponse,
    TodayRequest, TodayResponse, UpsertItemRequest, UpsertItemResponse,
};
use crate::api::v1::todo_service_server::TodoService;

pub struct DefaultTodoService {}

impl DefaultTodoService {
    pub fn new() -> Self {
        Self {  }
    }
}

#[async_trait]
impl TodoService for DefaultTodoService {
    type GetStream = Pin<Box<dyn Stream<Item = Result<GetResponse>> + Send + Sync>>;

    async fn today(&self, request: Request<TodayRequest>) -> Result<Response<TodayResponse>> {
        unimplemented!()
    }

    async fn search(&self, request: Request<SearchRequest>) -> Result<Response<SearchResponse>> {
        unimplemented!()
    }

    async fn get(
        &self,
        request: Request<Streaming<GetRequest>>,
    ) -> Result<Response<Self::GetStream>> {
        unimplemented!()
    }

    async fn upsert_item(
        &self,
        request: Request<UpsertItemRequest>,
    ) -> Result<Response<UpsertItemResponse>> {
        unimplemented!()
    }

    async fn remove_item(
        &self,
        request: Request<RemoveItemRequest>,
    ) -> Result<Response<RemoveItemResponse>> {
        unimplemented!()
    }
}
