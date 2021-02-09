use std::future::Future;
use std::task::{Context,Poll};
use std::pin::Pin;
use pin_project::pin_project;
use std::time;

// future that implements a timeout that prints to stderr
// if the polling took too long

#[pin_project]
pub struct PollMaxDuration<F> {
    #[pin]
    inner: F
}

impl<F: Future> Future for PollMaxDuration<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<<Self as Future>::Output> {
        eprintln!("poll()");

        // INFO: replaced by pin-project
        //
        // let future = unsafe {
        //     self.map_unchecked_mut(|f| {
        //         &mut f.inner
        //     })
        // };
        //
        // future.poll(ctx)

        let now = time::Instant::now();

        let this = self.project();
        let res = this.inner.poll(ctx);

        let elapsed = now.elapsed();

        if elapsed > time::Duration::from_millis(10) {
            eprintln!("WARNING: future polled for {:?} ms", elapsed);
        }

        res
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
