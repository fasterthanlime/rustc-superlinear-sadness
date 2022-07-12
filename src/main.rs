////////////////////////////////////////////////////////////////////////////////

trait Service<Request> {
    type Response;
    type Future: FnOnce() -> Self::Response;

    fn call(&mut self, req: Request) -> Self::Future;
}

////////////////////////////////////////////////////////////////////////////////

struct SampleRequest;
struct SampleResponse;

struct BorrowedRequest<'a> {
    #[allow(dead_code)]
    req: &'a mut SampleRequest,
}

type BoxFut<'a, O> = Box<dyn FnOnce() -> O + 'a>;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct OuterService<S>(S);

impl<S> Service<SampleRequest> for OuterService<S>
where
    for<'b> S: Service<BorrowedRequest<'b>, Response = SampleResponse> + Clone + 'static,
    for<'b> <S as Service<BorrowedRequest<'b>>>::Future: 'b,
{
    type Response = SampleResponse;
    type Future = BoxFut<'static, Self::Response>;

    fn call(&mut self, mut req: SampleRequest) -> Self::Future {
        let mut inner = self.0.clone();
        std::mem::swap(&mut self.0, &mut inner);

        Box::new(move || {
            let breq = BorrowedRequest { req: &mut req };
            (inner.call(breq))()
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct MiddleService<S>(S);

impl<'a, S> Service<BorrowedRequest<'a>> for MiddleService<S>
where
    for<'b> S: Service<BorrowedRequest<'b>, Response = SampleResponse> + Clone + 'static,
    for<'b> <S as Service<BorrowedRequest<'b>>>::Future: 'b,
{
    type Response = SampleResponse;
    type Future = BoxFut<'a, Self::Response>;

    fn call(&mut self, req: BorrowedRequest<'a>) -> Self::Future {
        let mut inner = self.0.clone();
        std::mem::swap(&mut self.0, &mut inner);

        Box::new(move || (inner.call(req))())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct InnerService;

impl<'a> Service<BorrowedRequest<'a>> for InnerService {
    type Response = SampleResponse;
    type Future = BoxFut<'static, Self::Response>;

    fn call(&mut self, _req: BorrowedRequest<'a>) -> Self::Future {
        Box::new(move || SampleResponse)
    }
}

////////////////////////////////////////////////////////////////////////////////

fn make_http_service() -> Box<
    dyn Service<SampleRequest, Response = SampleResponse, Future = BoxFut<'static, SampleResponse>>,
> {
    let service = InnerService;

    // 👋 uncomment / add more lines here to witness compile times going bonkers
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);

    let service = OuterService(service);

    Box::new(service)
}

fn main() {
    let svc = make_http_service();
    let _ = svc;
}
