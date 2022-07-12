////////////////////////////////////////////////////////////////////////////////

trait FakeFuture {
    type Output;
}

impl<I> FakeFuture for (I,) {
    type Output = ();
}

impl<I, F> FakeFuture for (I, F)
where
    F: FakeFuture,
{
    type Output = ();
}

trait Service<Request> {
    type Response;
    type Future: FakeFuture<Output = Self::Response>;
    fn i_am_a_service(&mut self) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct MiddleService<S>(S);

impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b (), Response = ()>,
    // 👋 comment this line to restore compile times to normal
    for<'b> <S as Service<&'b ()>>::Future: 'b,
{
    type Response = ();
    type Future = (&'a (), <S as Service<&'a ()>>::Future);
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct InnerService;

impl<'a> Service<&'a ()> for InnerService {
    type Response = ();
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
    let service = MiddleService(service);
    let service = MiddleService(service);
    let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);
    // let service = MiddleService(service);

    let mut service = service;
    service.i_am_a_service();

    let _a = service;
}
