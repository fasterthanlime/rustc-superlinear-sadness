////////////////////////////////////////////////////////////////////////////////

use std::marker::PhantomData;

trait FakeFuture {
    type Output;
}

struct BaseFF<I, O> {
    _phantom: PhantomData<(I, O)>,
}

impl<I, O> FakeFuture for BaseFF<I, O> {
    type Output = O;
}

struct NestedFF<I, O> {
    _phantom: PhantomData<(I, O)>,
}

impl<I, O> FakeFuture for NestedFF<I, O>
where
    O: FakeFuture,
{
    type Output = O::Output;
}

trait Service<Request> {
    type Future: FakeFuture<Output = SampleResponse>;
    fn call(&mut self) -> Self::Future;
}

////////////////////////////////////////////////////////////////////////////////

struct SampleRequest;
struct SampleResponse;

struct BorrowedRequest<'a> {
    #[allow(dead_code)]
    req: &'a mut SampleRequest,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct OuterService<S>(S);

impl<S> Service<SampleRequest> for OuterService<S>
where
    for<'b> S: Service<BorrowedRequest<'b>> + Clone + 'static,
    for<'b> <S as Service<BorrowedRequest<'b>>>::Future: 'b,
{
    type Future = NestedFF<SampleRequest, <S as Service<BorrowedRequest<'static>>>::Future>;

    fn call(&mut self) -> Self::Future {
        NestedFF {
            _phantom: Default::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct MiddleService<S>(S);

impl<'a, S> Service<BorrowedRequest<'a>> for MiddleService<S>
where
    for<'b> S: Service<BorrowedRequest<'b>> + Clone + 'static,
    for<'b> <S as Service<BorrowedRequest<'b>>>::Future: 'b,
{
    type Future = NestedFF<BorrowedRequest<'a>, <S as Service<BorrowedRequest<'a>>>::Future>;

    fn call(&mut self) -> Self::Future {
        NestedFF {
            _phantom: Default::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct InnerService;

impl<'a> Service<BorrowedRequest<'a>> for InnerService {
    type Future = BaseFF<BorrowedRequest<'a>, SampleResponse>;

    fn call(&mut self) -> Self::Future {
        BaseFF {
            _phantom: Default::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn main() {
    let service = InnerService;

    // 👋 uncomment / add more lines here to witness compile times going bonkers
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);

    let mut service = service;
    service.call();

    let _ = service;
}
