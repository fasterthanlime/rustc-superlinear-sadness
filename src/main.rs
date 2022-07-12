use std::{future::Future, pin::Pin};

////////////////////////////////////////////////////////////////////////////////

trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn call(&mut self, req: Request) -> Self::Future;
}

////////////////////////////////////////////////////////////////////////////////

struct SampleRequest;
struct SampleResponse;
struct SampleError;

struct BorrowedRequest<'a> {
    #[allow(dead_code)]
    req: &'a mut SampleRequest,
}

type BoxFut<'a, O> = Pin<Box<dyn Future<Output = O> + 'a>>;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct OuterService<S>(S);

impl<S> Service<SampleRequest> for OuterService<S>
where
    for<'b> S: Service<BorrowedRequest<'b>, Response = SampleResponse, Error = SampleError>
        + Clone
        + 'static,
    for<'b> <S as Service<BorrowedRequest<'b>>>::Future: 'b,
{
    type Response = SampleResponse;
    type Error = SampleError;
    type Future = BoxFut<'static, Result<Self::Response, Self::Error>>;

    fn call(&mut self, mut req: SampleRequest) -> Self::Future {
        let mut inner = self.0.clone();
        std::mem::swap(&mut self.0, &mut inner);

        Box::pin(async move {
            let breq = BorrowedRequest { req: &mut req };

            let res = inner.call(breq).await?;
            Ok(res)
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct MiddleService<S>(S);

impl<'a, S> Service<BorrowedRequest<'a>> for MiddleService<S>
where
    for<'b> S: Service<BorrowedRequest<'b>, Response = SampleResponse, Error = SampleError>
        + Clone
        + 'static,
    for<'b> <S as Service<BorrowedRequest<'b>>>::Future: 'b,
{
    type Response = SampleResponse;
    type Error = SampleError;
    type Future = BoxFut<'a, Result<Self::Response, Self::Error>>;

    fn call(&mut self, req: BorrowedRequest<'a>) -> Self::Future {
        let mut inner = self.0.clone();
        std::mem::swap(&mut self.0, &mut inner);

        Box::pin(async move {
            let res = inner.call(req).await?;
            Ok(res)
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct InnerService;

impl<'a> Service<BorrowedRequest<'a>> for InnerService {
    type Response = SampleResponse;
    type Error = SampleError;
    type Future = BoxFut<'static, Result<Self::Response, Self::Error>>;

    fn call(&mut self, req: BorrowedRequest<'a>) -> Self::Future {
        Box::pin(async move {
            let _ = req;
            Ok(SampleResponse)
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

fn make_http_service() -> Box<
    dyn Service<
        SampleRequest,
        Response = SampleResponse,
        Error = SampleError,
        Future = BoxFut<'static, Result<SampleResponse, SampleError>>,
    >,
> {
    let service = InnerService;

    // 👋 uncomment / add more `.layer` lines here to witness compile times
    // going bonkers.
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    // let service = MiddleService(service);

    let service = OuterService(service);

    Box::new(service)
}

fn main() {
    let svc = make_http_service();
    let _ = svc;
}
