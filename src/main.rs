////////////////////////////////////////////////////////////////////////////////

trait FakeFuture {
    type Output;
}

impl<I> FakeFuture for (I,) {
    type Output = Result<(), ()>;
}

impl<I, F> FakeFuture for (I, F)
where
    F: FakeFuture,
{
    type Output = Result<(), ()>;
}

trait Service<Request> {
    type Response;
    type Error;
    type Future: FakeFuture<Output = Result<Self::Response, Self::Error>>;
    fn i_am_a_service(&mut self) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct MiddleService<S>(S);

impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b (), Response = (), Error = ()>,
    // 👋 comment this line to restore compile times to normal
    // for<'b> <S as Service<&'b ()>>::Future: 'b,
    for<'b> <S as Service<&'b ()>>::Future: Clone,
{
    type Response = ();
    type Error = ();
    type Future = (&'a (), <S as Service<&'a ()>>::Future);
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct InnerService;

impl<'a> Service<&'a ()> for InnerService {
    type Response = ();
    type Error = ();
    type Future = (&'a (),);
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
    // let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);

    let mut service = service;
    service.i_am_a_service();

    let _a = service;
}
