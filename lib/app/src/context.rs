use tokio_util::sync::CancellationToken;

#[derive(Clone)]
pub struct Context {
    token: CancellationToken,
}

impl Context {
    pub fn root() -> Self {
        Self {
            token: CancellationToken::new(),
        }
    }

    pub fn cancel(&self) {
        self.token.cancel();
    }

    pub async fn cancelled(&self) {
        self.token.cancelled().await;
    }
}
