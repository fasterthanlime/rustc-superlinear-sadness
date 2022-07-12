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

impl<I, F> FakeFuture for (I, F)
where
    F: FakeFuture,
{
    type Output = ();
}

trait Service<Request> {
    type Future: FakeFuture<Output = ()>;
    fn i_am_a_service(&mut self) {}
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
    type Future = (Borrowed<'a>, <S as Service<Borrowed<'a>>>::Future);
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct InnerService;

impl<'a> Service<Borrowed<'a>> for InnerService {
    type Future = BaseFF<Borrowed<'a>, ()>;
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
    let service = MiddleService(service);
    // let service = MiddleService(service);

    let mut service = service;
    service.i_am_a_service();

    let _ = service;
}
