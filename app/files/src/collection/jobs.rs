use std::{future::Future, sync::Arc};

use futures::StreamExt;
use oxidrive_pubsub::Publisher;
use oxidrive_workers::{
    Dispatch, Process, Worker,
    queue::{Enqueue, JobQueue},
};

pub use refresh_collection::*;
pub use refresh_collections::*;

use crate::file::FileEvent;

use super::CollectionEvent;

mod refresh_collection;
mod refresh_collections;

pub(crate) struct JobsModule;

impl app::Module for JobsModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(RefreshCollectionWorker::new);
        c.bind(
            |queue: Arc<dyn JobQueue>,
             enqueue: Arc<dyn Enqueue>,
             process: RefreshCollectionWorker| { Worker::new(queue, enqueue, process) },
        );

        c.bind(RefreshCollectionsWorker::new);
        c.bind(
            |queue: Arc<dyn JobQueue>,
             enqueue: Arc<dyn Enqueue>,
             process: RefreshCollectionsWorker| {
                Worker::new(queue, enqueue, process)
            },
        );
    }
}

#[app::async_trait]
impl app::Hooks for JobsModule {
    async fn after_start(
        &mut self,
        ctx: app::context::Context,
        c: &app::di::Container,
    ) -> app::eyre::Result<()> {
        start_event_listener::<RefreshCollectionsWorker, FileEvent, _, _>(
            ctx.clone(),
            c,
            |dispatcher, event| async move {
                match event {
                    FileEvent::Changed(file) | FileEvent::Deleted(file) => {
                        if let Err(err) = dispatcher
                            .dispatch(RefreshCollections {
                                owner_id: file.owner_id,
                            })
                            .await
                        {
                            tracing::error!(
                                error = %err,
                                account_id = %file.owner_id,
                                file_id = %file.id,
                                "failed to queue RefreshCollections job",
                            );
                        }
                    }
                }
            },
        );

        start_event_listener::<RefreshCollectionWorker, CollectionEvent, _, _>(
            ctx,
            c,
            |dispatcher, event| async move {
                match event {
                    CollectionEvent::Changed(collection) => {
                        if let Err(err) = dispatcher
                            .dispatch(RefreshCollection {
                                collection_id: collection.id,
                            })
                            .await
                        {
                            tracing::error!(
                                error = %err,
                                account_id = %collection.owner_id,
                                collection_id = %collection.id,
                                "failed to queue RefreshCollection job",
                            );
                        }
                    }
                }
            },
        );

        Ok(())
    }
}

fn start_event_listener<W, E, F, Fut>(
    ctx: app::context::Context,
    c: &app::di::Container,
    mut handler: F,
) where
    W: Process,
    W::Job: Send,
    E: Clone + Send + 'static,
    F: FnMut(Dispatch<W::Job>, E) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send,
{
    let worker = c.get::<Worker<W>>();
    let publisher = c.get::<Publisher<E>>();
    let dispatcher = worker.dispatcher();
    let mut subscriber = publisher.subscribe();

    worker.clone().start(ctx.clone());

    let run = async move {
        while let Some(event) = subscriber.next().await {
            handler(dispatcher.clone(), event).await
        }
    };

    tokio::spawn(async move {
        tokio::select! {
            _ = run => {},
            _ = ctx.cancelled() => {},
        };
    });
}
