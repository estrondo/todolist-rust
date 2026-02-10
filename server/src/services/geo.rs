use crate::api::v1::{
    geo::{
        RemoveLocationRequest, RemoveLocationResponse, SearchItemAroundRequest,
        SearchItemAroundResponse, UpsertLocationRequest, UpsertLocationResponse,
    },
    geo_service_server::GeoService,
};
use log;
use todolist_core::{manager::geo::GeoManager, persistence::GeoPersistence};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{self, Request, Response, Status, Streaming};

#[derive(Debug)]
pub struct GeoServiceImpl<M> {
    manager: M,
}

impl<P> GeoServiceImpl<P>
where
    P: GeoManager,
{
    pub fn new(manager: P) -> Self {
        Self { manager }
    }
}

#[tonic::async_trait]
impl<M> GeoService for GeoServiceImpl<M>
where
    M: GeoManager + Sync + Send + 'static,
{
    type SearchItemAroundStream = ReceiverStream<Result<SearchItemAroundResponse, Status>>;

    async fn upsert_location(
        &self,
        _request: Request<Streaming<UpsertLocationRequest>>,
    ) -> std::result::Result<Response<UpsertLocationResponse>, Status> {
        log::info!("Upserting a location {:?}", _request.get_ref());

        unimplemented!()
    }

    async fn search_item_around(
        &self,
        _request: Request<Streaming<SearchItemAroundRequest>>,
    ) -> std::result::Result<Response<Self::SearchItemAroundStream>, Status> {
        unimplemented!()
    }
    async fn remove_location(
        &self,
        _request: Request<RemoveLocationRequest>,
    ) -> std::result::Result<Response<RemoveLocationResponse>, Status> {
        unimplemented!()
    }
}
