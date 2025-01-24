use std::{marker::PhantomData, sync::Arc};

use crate::di::Container;

pub trait Provider<Dependency> {
    type Provided;

    fn provide(&self, c: &Container) -> Self::Provided;
}

pub trait ProviderExt<D>: Provider<D> {
    fn map<T, F>(self, f: F) -> MapProvider<Self, F, T>
    where
        F: Fn(Self::Provided) -> T,
        Self: Sized;

    fn filter<F>(self, f: F) -> FilterProvider<Self, F>
    where
        F: Fn(&Self::Provided) -> bool,
        Self: Sized;

    fn arced(self) -> ArcProvider<Self>
    where
        Self: Sized;

    fn boxed(self) -> BoxProvider<Self>
    where
        Self: Sized;
}

impl<P, D> ProviderExt<D> for P
where
    P: Provider<D>,
{
    fn map<T, F>(self, f: F) -> MapProvider<Self, F, T>
    where
        F: Fn(Self::Provided) -> T,
        Self: Sized,
    {
        MapProvider::new(self, f)
    }

    fn filter<F>(self, f: F) -> FilterProvider<Self, F>
    where
        F: Fn(&Self::Provided) -> bool,
        Self: Sized,
    {
        FilterProvider::new(self, f)
    }

    fn arced(self) -> ArcProvider<Self> {
        ArcProvider::new(self)
    }

    fn boxed(self) -> BoxProvider<Self> {
        BoxProvider::new(self)
    }
}

macro_rules! provider_fn (
    ($($param:ident),*) => {
        #[allow(non_snake_case, unused_mut)]
        impl<F, $($param,)* T> Provider<($($param,)*)> for F
        where
        F: Fn($($param),*) -> T,
        $($param: Sized + Clone + 'static,)*
        {
            type Provided = T;

            #[inline]
            fn provide(&self, c: &Container) -> Self::Provided {
                let mut missing = Vec::new();

                $(
              let $param = c.get_opt::<$param>().cloned().or_else(|| {
                  let debug_info = if cfg!(debug_assertions) {
                      std::borrow::Cow::Owned(format!(r#"

DEBUG: the available types are: {:?}

"#, c))
                  } else {
                      "".into()
                  };

                  missing.push(format!(
                      "* could not resolve dependency {0} of {1} (provided by {2}). Make sure the provider for {0} has been added to the Container before {1}{3}",
                      std::any::type_name::<$param>(),
                      std::any::type_name::<T>(),
                      std::any::type_name::<F>(),
                      debug_info,
                  ));
                  None
              });
                )*

                if !missing.is_empty() {
                    panic!("dependencies are missing:\n\n{}", missing.join("\n"));
                }

                (self)($($param.unwrap(),)*)
            }
        }
    }
);

macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!(T1);
        $name!(T1, T2);
        $name!(T1, T2, T3);
        $name!(T1, T2, T3, T4);
        $name!(T1, T2, T3, T4, T5);
        $name!(T1, T2, T3, T4, T5, T6);
        $name!(T1, T2, T3, T4, T5, T6, T7);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    };
}

impl<T, F> Provider<()> for F
where
    F: Fn() -> T,
{
    type Provided = T;

    fn provide(&self, _: &Container) -> Self::Provided {
        (self)()
    }
}

all_the_tuples!(provider_fn);

pub struct MapProvider<P, F, T> {
    inner: P,
    map: F,
    _to: PhantomData<T>,
}

impl<P, F, T> MapProvider<P, F, T> {
    fn new(inner: P, map: F) -> Self {
        MapProvider {
            inner,
            map,
            _to: PhantomData,
        }
    }
}

impl<D, T, P, F> Provider<D> for MapProvider<P, F, T>
where
    P: Provider<D>,
    F: Fn(P::Provided) -> T,
{
    type Provided = T;

    fn provide(&self, c: &Container) -> Self::Provided {
        let p = self.inner.provide(c);
        (self.map)(p)
    }
}

pub struct FilterProvider<P, F> {
    inner: P,
    filter: F,
}

impl<P, F> FilterProvider<P, F> {
    fn new(inner: P, filter: F) -> Self {
        FilterProvider { inner, filter }
    }
}

impl<D, P, F> Provider<D> for FilterProvider<P, F>
where
    P: Provider<D>,
    F: Fn(&P::Provided) -> bool,
{
    type Provided = Option<P::Provided>;

    fn provide(&self, c: &Container) -> Self::Provided {
        let p = self.inner.provide(c);
        (self.filter)(&p).then_some(p)
    }
}

pub struct ArcProvider<P>(P);

impl<P> ArcProvider<P> {
    fn new(provider: P) -> Self {
        Self(provider)
    }
}

impl<D, P> Provider<D> for ArcProvider<P>
where
    P: Provider<D>,
{
    type Provided = Arc<P::Provided>;

    fn provide(&self, c: &Container) -> Self::Provided {
        Arc::new(self.0.provide(c))
    }
}

pub struct BoxProvider<P>(P);

impl<P> BoxProvider<P> {
    fn new(provider: P) -> Self {
        Self(provider)
    }
}

impl<D, P> Provider<D> for BoxProvider<P>
where
    P: Provider<D>,
{
    type Provided = Box<P::Provided>;

    fn provide(&self, c: &Container) -> Self::Provided {
        Box::new(self.0.provide(c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_function_is_a_provider() {
        fn test_provider(answer: u8) -> String {
            format!("the answer is {answer}")
        }

        let c = Container::builder().add(42u8).init();

        let s = test_provider.provide(&c);
        assert_eq!(s, "the answer is 42");
    }

    #[test]
    fn a_constructor_is_a_provider() {
        struct Test {
            answer: u8,
        }

        impl Test {
            fn new(answer: u8) -> Self {
                Self { answer }
            }
        }

        let c = Container::builder().add(42u8).init();

        let Test { answer } = Test::new.provide(&c);
        assert_eq!(answer, 42);
    }

    #[test]
    fn a_closure_is_a_provider() {
        let test_provider = |answer: u8| format!("the answer is {answer}");

        let c = Container::builder().add(42u8).init();

        let s = test_provider.provide(&c);
        assert_eq!(s, "the answer is 42");
    }

    #[test]
    fn it_accepts_optional_args_if_they_are_defined() {
        let test_provider = |answer: Option<u8>| {
            format!(
                "the answer is {}",
                answer
                    .map(|answer| answer.to_string())
                    .unwrap_or_else(|| "missing".into())
            )
        };

        let c = Container::builder().bind_opt(|| None::<u8>).init();

        let s = test_provider.provide(&c);
        assert_eq!(s, "the answer is missing");
    }

    #[test]
    fn it_maps_a_provider() {
        fn test_provider() -> String {
            "hello".into()
        }

        struct Wrapper(String);

        let c = Container::default();

        let Wrapper(s) = test_provider.map(Wrapper).provide(&c);
        assert_eq!(s, "hello");
    }

    #[test]
    fn it_filters_a_provider() {
        fn test_provider() -> String {
            "hello".into()
        }

        let c = Container::default();

        let s = test_provider.filter(|_: &String| false).provide(&c);
        assert_eq!(s, None);
    }

    #[test]
    fn it_maps_a_provider_with_an_arc() {
        fn test_provider() -> String {
            "hello".into()
        }

        let c = Container::default();

        let s = test_provider.arced().provide(&c);
        assert_eq!(s, Arc::new("hello".into()));
    }

    #[test]
    fn it_maps_a_provider_with_a_box() {
        fn test_provider() -> String {
            "hello".into()
        }

        let c = Container::default();

        let s = test_provider.boxed().provide(&c);
        assert_eq!(s, Box::new("hello".into()));
    }
}
