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
    type Future: FakeFuture<Output = ()>;
    fn call(&mut self) -> Self::Future;
}

////////////////////////////////////////////////////////////////////////////////

struct Borrowed<'a>(&'a mut ());

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct MiddleService<S>(S);

impl<'a, S> Service<Borrowed<'a>> for MiddleService<S>
where
    for<'b> S: Service<Borrowed<'b>> + Clone + 'static,
    for<'b> <S as Service<Borrowed<'b>>>::Future: 'b,
{
    type Future = NestedFF<Borrowed<'a>, <S as Service<Borrowed<'a>>>::Future>;

    fn call(&mut self) -> Self::Future {
        NestedFF {
            _phantom: Default::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct InnerService;

impl<'a> Service<Borrowed<'a>> for InnerService {
    type Future = BaseFF<Borrowed<'a>, ()>;

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
    let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);

    let mut service = service;
    service.call();

    let _ = service;
}
