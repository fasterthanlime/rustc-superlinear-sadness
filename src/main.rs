#![feature(type_alias_impl_trait)]

use std::future::Future;
use tower::{util::BoxCloneService, Layer, Service, ServiceBuilder, ServiceExt};

////////////////////////////////////////////////////////////////////////////////

struct SampleRequest;
struct SampleResponse;
struct SampleError;

////////////////////////////////////////////////////////////////////////////////

struct OuterLayer;

struct BorrowedRequest<'a> {
    #[allow(dead_code)]
    req: &'a mut SampleRequest,
}

impl<S> Layer<S> for OuterLayer {
    type Service = OuterService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        OuterService { inner }
    }
}

#[derive(Clone)]
struct OuterService<S> {
    inner: S,
}

impl<S> Service<SampleRequest> for OuterService<S>
where
    for<'b> S: Service<BorrowedRequest<'b>, Response = SampleResponse, Error = SampleError>
        + Clone
        + Send
        + Sync
        + 'static,
    for<'b> <S as Service<BorrowedRequest<'b>>>::Future: Send + 'b,
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

    fn call(&mut self, mut req: SampleRequest) -> Self::Future {
        let mut inner = self.inner.clone();
        std::mem::swap(&mut self.inner, &mut inner);

        async move {
            let breq = BorrowedRequest { req: &mut req };

            let res = inner.call(breq).await?;
            Ok(res)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

struct MiddleLayer;

impl<S> Layer<S> for MiddleLayer {
    type Service = MiddleService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MiddleService { inner }
    }
}

#[derive(Clone)]
struct MiddleService<S> {
    inner: S,
}

impl<'a, S> Service<BorrowedRequest<'a>> for MiddleService<S>
where
    for<'b> S: Service<BorrowedRequest<'b>, Response = SampleResponse, Error = SampleError>
        + Clone
        + Send
        + 'static,
    for<'b> <S as Service<BorrowedRequest<'b>>>::Future: Send + 'b,
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

    fn call(&mut self, req: BorrowedRequest<'a>) -> Self::Future {
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

impl<'a> Service<BorrowedRequest<'a>> for InnerService {
    type Response = SampleResponse;
    type Error = SampleError;
    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: BorrowedRequest<'a>) -> Self::Future {
        async move {
            let _ = req;
            Ok(SampleResponse)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn make_http_service() -> BoxCloneService<SampleRequest, SampleResponse, SampleError> {
    // 👋 uncomment / add more `.layer` lines here to witness compile times
    // going bonkers.

    ServiceBuilder::new()
        .layer(OuterLayer)
        .layer(MiddleLayer)
        .layer(MiddleLayer)
        .layer(MiddleLayer)
        .layer(MiddleLayer)
        .layer(MiddleLayer)
        .layer(MiddleLayer)
        // .layer(MiddleLayer)
        // .layer(MiddleLayer)
        // .layer(MiddleLayer)
        .service(InnerService)
        .boxed_clone()
}

fn main() {
    let svc = make_http_service();
    let _ = svc;
}
