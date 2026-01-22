use std::{
    fmt::Debug,
    pin::{Pin, pin},
    rc::Rc,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use tokio::time::sleep;

trait SayHi: Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {self:?}");
    }
}
impl<T: Debug> SayHi for T {}

//
trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}

impl<T: Default> MutMeSomehow for Box<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        **self.get_mut() = <T as Default>::default();
    }
}
impl<T: Default> MutMeSomehow for Rc<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        *self.get_mut() = Self::default();
    }
}
impl<T> MutMeSomehow for Vec<T> {
    fn mut_me_somehow(mut self: Pin<&mut Self>) {
        self.set(Self::new());
    }
}
impl MutMeSomehow for String {
    fn mut_me_somehow(mut self: Pin<&mut Self>) {
        self.push_str(" this task is weird");
    }
}
impl MutMeSomehow for &[u8] {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        *self.get_mut() = &[1, 2, 3];
    }
}

mod mut_me_blanket {
    use std::pin::Pin;

    pub trait MutMeSomehow {
        fn mut_me_somehow(self: Pin<&mut Self>);
    }

    impl<T: Default> MutMeSomehow for T {
        fn mut_me_somehow(mut self: Pin<&mut Self>) {
            self.set(Default::default());
        }
    }
}

//
struct MeasurableFuture<Fut> {
    inner_future: Fut,
    started_at: Option<Instant>,
}

impl<Fut> MeasurableFuture<Fut> {
    pub const fn new(fut: Fut) -> Self {
        Self {
            inner_future: fut,
            started_at: None,
        }
    }
}

impl<Fut: Future> Future for MeasurableFuture<Fut> {
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: Projecting the pin from Self to the inner_future field.
        // It is safe because:
        // * We never move out of inner_future;
        // * If Self is pinned, inner_future must also remain pinned;
        // * We can freely access started_at field as Option<Instant> is Unpin.
        let (inner_pinned, started_at) = unsafe {
            let this = self.get_unchecked_mut();
            (
                Pin::new_unchecked(&mut this.inner_future),
                &mut this.started_at,
            )
        };

        if started_at.is_none() {
            *started_at = Some(Instant::now());
        }

        match inner_pinned.poll(cx) {
            Poll::Ready(out) => {
                let elapsed = started_at.unwrap().elapsed();
                println!("Elapsed {}ns", elapsed.as_nanos());
                Poll::Ready(out)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[tokio::main]
async fn main() {
    let mut p = pin!(Box::new("box"));
    p.as_ref().say_hi();
    p.as_mut().mut_me_somehow();
    p.as_ref().say_hi();
    println!();

    let mut p = pin!(Rc::new("rc"));
    p.as_ref().say_hi();
    p.as_mut().mut_me_somehow();
    p.as_ref().say_hi();
    println!();

    let mut p = pin!(vec!["vec"]);
    p.as_ref().say_hi();
    p.as_mut().mut_me_somehow();
    p.as_ref().say_hi();
    println!();

    let mut p = pin!(String::from("For sure"));
    p.as_ref().say_hi();
    p.as_mut().mut_me_somehow();
    p.as_ref().say_hi();
    println!();

    let mut p = pin!([42].as_slice());
    p.as_ref().say_hi();
    p.as_mut().mut_me_somehow();
    p.as_ref().say_hi();
    println!();

    let mut p = pin!(true);
    p.as_ref().say_hi();
    mut_me_blanket::MutMeSomehow::mut_me_somehow(p.as_mut());
    p.as_ref().say_hi();
    println!();

    MeasurableFuture::new(sleep(Duration::from_micros(1))).await;
    MeasurableFuture::new(sleep(Duration::from_millis(1))).await;
    MeasurableFuture::new(sleep(Duration::from_secs(1))).await;
}
