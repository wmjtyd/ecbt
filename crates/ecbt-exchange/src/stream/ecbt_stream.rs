use super::shared::Result;
use super::CallbackHandle;
use super::ExchangeStream;
use super::Subscriptions;
use crate::model::websocket::Subscription;
use crate::model::websocket::WebSocketResponse;

use futures::stream::BoxStream;

pub struct EcbtStream<E: ExchangeStream> {
    pub websocket: E,
}

impl<E: ExchangeStream> EcbtStream<E> {
    pub async fn instantiate(params: E::InitParams) -> Result<Self> {
        let websocket = E::new(params).await?;
        Ok(Self { websocket })
    }

    pub async fn create_stream_specific(
        &self,
        subscriptions: Subscriptions<E::Subscription>,
    ) -> Result<BoxStream<'static, Result<E::Response>>> {
        self.websocket.create_stream_specific(subscriptions).await
    }

    pub async fn subscribe<
        F: Fn(&Result<WebSocketResponse<E::Response>>) + Sync + Send + 'static,
    >(
        &self,
        subscription: Subscription,
        callback: F,
    ) -> Result<CallbackHandle> {
        self.websocket.subscribe(subscription, callback).await
    }

    pub async fn create_stream<S: Into<E::Subscription> + Clone + Send + Sync>(
        &self,
        subscriptions: &[S],
    ) -> Result<BoxStream<'static, Result<WebSocketResponse<E::Response>>>> {
        self.websocket.create_stream(subscriptions).await
    }

    pub async fn disconnect(&self) {
        self.websocket.disconnect().await
    }
}
