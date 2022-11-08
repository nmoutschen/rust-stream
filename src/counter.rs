use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Stream;

pub struct Counter {
    limit: usize,
    current: usize,
}

impl Counter {
    pub fn new(limit: usize) -> Self {
        Self { limit, current: 0 }
    }
}

impl Stream for Counter {
    type Item = usize;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current >= self.limit {
            Poll::Ready(None)
        } else {
            let next = self.current + 1;
            self.as_mut().current += 1;
            Poll::Ready(Some(next))
        }
    }
}
