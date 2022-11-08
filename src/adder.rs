use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{ready, Stream};

#[pin_project]
pub struct Adder<S, I> {
    #[pin]
    stream: S,
    prev: Option<I>,
}

impl<S, I> Adder<S, I> {
    pub fn new(stream: S) -> Self {
        Self { stream, prev: None }
    }
}

impl<S, I> Stream for Adder<S, I>
where
    S: Stream<Item = I>,
    I: std::ops::Add<I> + Default + Clone,
{
    type Item = I::Output;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        Poll::Ready(loop {
            let prev = this.prev.clone();
            let next = ready!(this.stream.as_mut().poll_next(cx));

            match prev {
                Some(prev) => {
                    *this.prev = None;
                    break Some(prev + next.unwrap_or_default());
                }
                None => match next {
                    Some(next) => {
                        *this.prev = Some(next);
                    }
                    None => break None,
                },
            }
        })
    }
}
