#![feature(type_alias_impl_trait)]

use std::future::Future;
use tower::{util::BoxCloneService, Layer, Service, ServiceBuilder, ServiceExt};

////////////////////////////////////////////////////////////////////////////////

struct SampleRequest;
struct SampleResponse;
struct SampleError;

////////////////////////////////////////////////////////////////////////////////

struct SampleLayer;

impl<S> Layer<S> for SampleLayer
where
    S: Service<SampleRequest>,
{
    type Service = SampleService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SampleService { inner }
    }
}

#[derive(Clone)]
struct SampleService<S> {
    inner: S,
}

impl<S> Service<SampleRequest> for SampleService<S>
where
    S: Service<SampleRequest, Response = SampleResponse, Error = SampleError>
        + Clone
        + Send
        + 'static,
{
    type Response = SampleResponse;
    type Error = SampleError;
    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: SampleRequest) -> Self::Future {
        let mut inner = self.inner.clone();
        std::mem::swap(&mut self.inner, &mut inner);

        async move {
            let res = inner.call(req).await?;
            Ok(res)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct InnerService;

impl Service<SampleRequest> for InnerService {
    type Response = SampleResponse;
    type Error = SampleError;
    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: SampleRequest) -> Self::Future {
        async move {
            let _ = req;
            Ok(SampleResponse)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn make_http_service() -> BoxCloneService<SampleRequest, SampleResponse, SampleError> {
    ServiceBuilder::new()
        .layer(SampleLayer)
        .layer(SampleLayer)
        .layer(SampleLayer)
        .layer(SampleLayer)
        .layer(SampleLayer)
        .layer(SampleLayer)
        .layer(SampleLayer)
        .layer(SampleLayer)
        .layer(SampleLayer)
        .service(InnerService)
        .boxed_clone()
}

fn main() {
    let svc = make_http_service();
    let _ = svc;
}
