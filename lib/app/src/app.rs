use std::{borrow::Cow, future::Future, sync::Arc};

use crate::boot::Hooks;
use crate::di::{Container, ContainerBuilder, Module, Provider};

use eyre::WrapErr;

pub struct App {
    pub name: Cow<'static, str>,
    container: ContainerBuilder,
    hooks: Vec<Box<dyn Hooks>>,
}

impl App {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            container: ContainerBuilder::default(),
            hooks: Vec::new(),
        }
    }

    pub fn bind<T: Send + Sync + 'static, D>(
        mut self,
        provider: impl Provider<D, Provided = T> + 'static,
    ) -> Self {
        self.container = self.container.bind(provider);
        self
    }

    #[allow(clippy::should_implement_trait)] // that's really not the same thing as 1 + 1
    pub fn add<T: Send + Sync + 'static>(mut self, t: T) -> Self {
        self.container = self.container.add(t);
        self
    }

    pub fn mount(mut self, module: impl Module) -> Self {
        self.container = self.container.mount(module);
        self
    }

    pub fn mount_and_hook(self, module: impl Module + Hooks + Clone) -> Self {
        self.mount(module.clone()).hook(module)
    }

    pub fn hook(mut self, hooks: impl Hooks) -> Self {
        self.hooks.push(Box::new(hooks));
        self
    }

    pub fn init(self) -> Container {
        self.container.init()
    }

    pub async fn run<F, Fut>(mut self, run: F)
    where
        F: FnOnce(Arc<Container>) -> Fut,
        Fut: Future<Output = eyre::Result<()>>,
    {
        tracing::debug!(app = %self.name, "building dependency injection container");
        let container = Arc::new(self.container.init());

        tracing::info!(app = %self.name, "app starting up");

        tracing::debug!(app = %self.name, "running before_start hooks");
        run_hooks(self.hooks.iter_mut(), |h| h.before_start(&container)).await;

        tracing::debug!(app = %self.name, "running main function");
        let res = tokio::select! {
            res = run(container.clone()) => res,
            res = tokio::signal::ctrl_c() => res.wrap_err("failed to listen for SIGINT event"),
        }
        .inspect(|_| {
            tracing::info!(app = %self.name, "app shutting down");
        })
        .inspect_err(|err| {
            let stack = err
                .chain()
                .enumerate()
                .map(|(i, e)| format!("{i}: {e:?}"))
                .collect::<Vec<_>>()
                .join("\n");

            tracing::error!(
                app = %self.name,
                error.message = %err,
                error.kind = "AppError",
                error.stack = ?stack,
                "{} exited with error",
                self.name,
            );
        });

        let code = res.map(|_| 0).unwrap_or(1);

        tracing::debug!(app = %self.name, "running on_shutdown hooks");
        run_hooks(self.hooks.iter_mut().rev(), |h| h.on_shutdown(&container)).await;

        std::process::exit(code);
    }
}

async fn run_hooks<'iter, 'hook, I, F, Fut>(hooks: I, hook: F)
where
    'iter: 'hook,
    I: Iterator<Item = &'iter mut Box<dyn Hooks>>,
    F: Fn(&'hook mut dyn Hooks) -> Fut,
    Fut: Future<Output = eyre::Result<()>>,
{
    futures::future::try_join_all(hooks.map(|h| hook(h.as_mut())))
        .await
        .unwrap();
}

#[macro_export]
macro_rules! app {
    () => {
        $crate::App::new(env!("CARGO_PKG_NAME"))
    };

    ($name:expr) => {
        $crate::App::new($name)
    };
}

#[macro_export]
macro_rules! provides {
    ($module:ty, [$($ty:ty),*]) => {
        #[cfg(debug_assertions)]
        impl $module {
            pub fn di_ensure(container: &$crate::di::Container) {
                $(
                container.get::<$ty>();
                )*
            }
        }
    };

    ($module:ty, $($ty:ty),*) => { $crate::provides!($module, [$($ty),*]); };

    ($module:ty) => { $crate::provides!($module, []); };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! assert {
    ($container:expr, [$($module:ty),*]) => {
        $(
         <$module>::di_ensure(&$container);
        )*
    };

    ($container:expr, $($module:ty),*) => { $crate::assert!($container, [$($module), *]); };
}
