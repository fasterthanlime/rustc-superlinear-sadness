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
    type ThirdType;

    type Future: FakeFuture<Output = Result<Self::Response, Self::Error>>;
    fn i_am_a_service(&mut self) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct MiddleService<S>(S);

#[cfg(assoc_type_0)]
impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b ()>,
{
    type Response = ();
    type Error = ();
    type ThirdType = ();
    type Future = (&'a (), <S as Service<&'a ()>>::Future);
}

#[cfg(assoc_type_1)]
impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b (), Response = ()>,
{
    type Response = ();
    type Error = ();
    type ThirdType = ();
    type Future = (&'a (), <S as Service<&'a ()>>::Future);
}

#[cfg(assoc_type_2)]
impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b (), Response = (), Error = ()>,
{
    type Response = ();
    type Error = ();
    type ThirdType = ();
    type Future = (&'a (), <S as Service<&'a ()>>::Future);
}

#[cfg(assoc_type_3)]
impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b (), Response = (), Error = (), ThirdType = ()>,
{
    type Response = ();
    type Error = ();
    type ThirdType = ();
    type Future = (&'a (), <S as Service<&'a ()>>::Future);
}

#[cfg(outlives)]
impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b ()>,
    for<'b> <S as Service<&'b ()>>::Future: 'b,
{
    type Response = ();
    type Error = ();
    type ThirdType = ();
    type Future = (&'a (), <S as Service<&'a ()>>::Future);
}

#[cfg(clone)]
impl<'a, S> Service<&'a ()> for MiddleService<S>
where
    for<'b> S: Service<&'b ()>,
    for<'b> <S as Service<&'b ()>>::Future: Clone,
{
    type Response = ();
    type Error = ();
    type ThirdType = ();
    type Future = (&'a (), <S as Service<&'a ()>>::Future);
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct InnerService;

impl<'a> Service<&'a ()> for InnerService {
    type Response = ();
    type Error = ();
    type ThirdType = ();
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

    #[cfg(any(more7))]
    let service = MiddleService(service);

    #[cfg(any(more7, more6))]
    let service = MiddleService(service);

    #[cfg(any(more7, more6, more5))]
    let service = MiddleService(service);

    #[cfg(any(more7, more6, more5, more4))]
    let service = MiddleService(service);

    #[cfg(any(more7, more6, more5, more4, more3))]
    let service = MiddleService(service);

    #[cfg(any(more7, more6, more5, more4, more3, more2))]
    let service = MiddleService(service);

    #[cfg(any(more7, more6, more5, more4, more3, more2, more1))]
    let service = MiddleService(service);

    let mut service = service;
    service.i_am_a_service();

    let _a = service;
}
