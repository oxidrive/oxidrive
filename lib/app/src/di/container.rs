use std::{any::Any, collections::HashMap, fmt::Debug};

use super::{Module, Provider};

#[cfg(debug_assertions)]
type TypeKey = &'static str;

#[cfg(not(debug_assertions))]
type TypeKey = std::any::TypeId;

type TypeMap = HashMap<TypeKey, Box<dyn Any + Send + Sync>>;

#[cfg(debug_assertions)]
fn type_key<T: 'static>() -> TypeKey {
    std::any::type_name::<T>()
}

#[cfg(not(debug_assertions))]
fn type_key<T: 'static>() -> TypeKey {
    std::any::TypeId::of::<T>()
}

#[derive(Default)]
pub struct Container {
    data: TypeMap,
}

impl Debug for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Container")
            .field("data", &self.data.keys())
            .finish()
    }
}

impl Container {
    pub fn builder() -> ContainerBuilder {
        ContainerBuilder::default()
    }

    pub fn get_opt<T: 'static>(&self) -> Option<&T> {
        self.data
            .get(&type_key::<T>())
            .and_then(|t| t.downcast_ref())
    }

    pub fn get<T: 'static>(&self) -> &T {
        self.get_opt().unwrap_or_else(|| {
            panic!(
                "no instance of {} found in di::Container",
                std::any::type_name::<T>()
            )
        })
    }

    #[inline]
    fn bind<T: Send + Sync + 'static, D>(&mut self, provider: impl Provider<D, Provided = T>) {
        let t = provider.provide(self);
        self.add(t);
    }

    #[inline]
    fn bind_opt<T: Clone + Send + Sync + 'static, D>(
        &mut self,
        provider: impl Provider<D, Provided = Option<T>>,
    ) {
        let t = provider.provide(self);
        self.add(t.clone());

        if let Some(t) = t {
            self.add(t);
        }
    }

    #[inline]
    fn add<T: Send + Sync + 'static>(&mut self, data: T) {
        let key = type_key::<T>();
        if self.data.contains_key(&key) {
            panic!(
                "an instance of {} is already registered in di::Container",
                std::any::type_name::<T>()
            );
        }

        self.data
            .insert(key, Box::new(data) as Box<dyn Any + Send + Sync>);
    }
}

#[derive(Default)]
pub struct ContainerBuilder {
    ctx: Context,
}

impl ContainerBuilder {
    pub fn bind<T: Send + Sync + 'static, D>(
        mut self,
        provider: impl Provider<D, Provided = T> + 'static,
    ) -> Self {
        self.ctx.bind(provider);
        self
    }

    pub fn bind_opt<T: Clone + Send + Sync + 'static, D>(
        mut self,
        provider: impl Provider<D, Provided = Option<T>> + 'static,
    ) -> Self {
        self.ctx.bind_opt(provider);
        self
    }

    #[allow(clippy::should_implement_trait)] // that's really not the same thing as 1 + 1
    pub fn add<T: Send + Sync + 'static>(mut self, t: T) -> Self {
        self.ctx.add(t);
        self
    }

    pub fn mount(mut self, module: impl Module) -> Self {
        self.ctx.mount(module);
        self
    }

    pub fn init(self) -> Container {
        let mut container = Container::default();

        for init in self.ctx.inits {
            init(&mut container);
        }

        container
    }
}

#[derive(Default)]
pub struct Context {
    #[allow(clippy::type_complexity)] // it's not that complicated, come on
    inits: Vec<Box<dyn FnOnce(&mut Container)>>,
}

impl Context {
    pub fn bind<T: Send + Sync + 'static, D>(
        &mut self,
        provider: impl Provider<D, Provided = T> + 'static,
    ) -> &mut Self {
        self.inits.push(Box::new(move |container| {
            container.bind(provider);
        }));

        self
    }

    pub fn bind_opt<T: Clone + Send + Sync + 'static, D>(
        &mut self,
        provider: impl Provider<D, Provided = Option<T>> + 'static,
    ) -> &mut Self {
        self.inits.push(Box::new(move |container| {
            container.bind_opt(provider);
        }));

        self
    }

    pub fn add<T: Send + Sync + 'static>(&mut self, t: T) -> &mut Self {
        self.inits.push(Box::new(move |container| {
            container.add(t);
        }));

        self
    }

    pub fn mount(&mut self, module: impl Module) -> &mut Self {
        Box::new(module).mount(self);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_panics_if_the_value_is_not_in_the_container() {
        let c = Container::default();

        c.get::<String>();
    }

    #[test]
    fn it_returns_none_if_the_value_is_not_in_the_container() {
        let c = Container::default();

        assert_eq!(c.get_opt::<String>(), None);
    }

    #[test]
    fn it_adds_a_value_to_the_container() {
        let c = Container::builder().add(String::from("hello")).init();

        assert_eq!(c.get::<String>(), "hello");
    }

    #[test]
    fn it_binds_a_provider_to_the_container() {
        let c = Container::builder().bind(|| String::from("hello")).init();

        assert_eq!(c.get::<String>(), "hello");
    }

    #[test]
    fn it_resolves_a_provider_dependency() {
        let c = Container::builder()
            .add(42u8)
            .bind(|answer: u8| format!("the answer is {answer}"))
            .init();

        assert_eq!(c.get::<String>(), "the answer is 42");
    }

    #[test]
    #[should_panic]
    fn it_panics_if_it_cannot_resolve_a_provider_dependency() {
        let c = Container::builder()
            .bind(|answer: u8| format!("the answer is {answer}"))
            .init();

        c.get::<String>();
    }

    #[test]
    fn it_binds_an_optional_provider() {
        let c = Container::builder()
            .bind_opt(|| Some(String::from("hello")))
            .init();

        assert_eq!(c.get::<String>(), "hello");
        assert_eq!(c.get_opt::<String>().map(|s| s.as_str()), Some("hello"));
        assert_eq!(c.get::<Option<String>>().as_deref(), Some("hello"));

        assert_eq!(c.get_opt::<u8>(), None);
    }

    #[test]
    #[should_panic]
    fn it_panics_an_optional_provider_was_not_bound() {
        let c = Container::default();

        c.get::<Option<String>>();
    }

    #[test]
    fn it_mounts_a_module() {
        struct TestModule;

        impl Module for TestModule {
            fn mount(self: Box<Self>, c: &mut Context) {
                c.add(42u8);
                c.bind(|answer: u8| format!("the answer is {answer}"));
            }
        }

        let c = Container::builder().mount(TestModule).init();
        assert_eq!(c.get::<String>(), "the answer is 42");
    }

    #[test]
    fn it_mounts_multiple_modules() {
        struct FirstModule;

        impl Module for FirstModule {
            fn mount(self: Box<Self>, c: &mut Context) {
                c.add(42u8);
            }
        }

        struct SecondModule;

        impl Module for SecondModule {
            fn mount(self: Box<Self>, c: &mut Context) {
                c.bind(|answer: u8| format!("the answer is {answer}"));
            }
        }

        let c = Container::builder()
            .mount(FirstModule)
            .mount(SecondModule)
            .init();
        assert_eq!(c.get::<String>(), "the answer is 42");
    }

    #[test]
    #[should_panic]
    fn it_panics_if_dependent_modules_are_missing() {
        struct TestModule;

        impl Module for TestModule {
            fn mount(self: Box<Self>, c: &mut Context) {
                c.bind(|answer: u8| format!("the answer is {answer}"));
            }
        }

        let c = Container::builder().mount(TestModule).init();
        c.get::<String>();
    }

    #[test]
    #[should_panic]
    fn it_panics_if_dependent_modules_are_mounted_in_the_wrong_order() {
        struct FirstModule;

        impl Module for FirstModule {
            fn mount(self: Box<Self>, c: &mut Context) {
                c.add(42u8);
            }
        }

        struct SecondModule;

        impl Module for SecondModule {
            fn mount(self: Box<Self>, c: &mut Context) {
                c.bind(|answer: u8| format!("the answer is {answer}"));
            }
        }

        Container::builder()
            .mount(SecondModule)
            .mount(FirstModule)
            .init();

        eprintln!("the container should have failed to build, but instead succeeded");
    }
}
