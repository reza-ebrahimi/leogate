use async_graphql::*;
use futures::prelude::*;
use std::time::Duration;

use std::pin::Pin;

use futures::lock::Mutex;
use std::sync::Arc;

use systemstat::{data, Platform, System};
//use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
struct SystemStat {
  user: f32,
  nice: f32,
  system: f32,
  interrupt: f32,
  idle: f32,
}

const INTERVAL: Duration = Duration::from_millis(2000);

struct TokioInterval {
  interval: Option<tokio::time::Interval>,
}

impl TokioInterval {
  pub fn new(millis: u64) -> Self {
    Self {
      interval: Some(tokio::time::interval(Duration::from_millis(millis))),
    }
  }
}

impl Stream for TokioInterval {
  type Item = Option<()>;
  fn poll_next(
    mut self: Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Option<Self::Item>> {
    println!("Sent!!!!!!!!!!!!!!!!");
    if self.interval.as_mut().unwrap().poll_tick(cx) == std::task::Poll::Pending {
      return std::task::Poll::Pending;
    }
    std::task::Poll::Ready(Some(Some(())))
  }
}

impl Drop for TokioInterval {
  fn drop(&mut self) {
    if let Some(interval) = self.interval.take() {
      drop(interval);
    }

    println!("DROP TokioInterval");
  }
}

#[derive(Clone, Default)]
pub struct SystemStatSubscription;

#[Subscription]
impl SystemStatSubscription {
  async fn sys_stat(&self, ctx: &Context<'_>) -> impl Stream<Item = Option<SystemStat>> {
    ctx
      .data_unchecked::<Arc<Mutex<tokio::runtime::Runtime>>>()
      .lock()
      .await
      .spawn(async {
        println!("TokioInterval called");

        let sys = System::new();
        let mut aggregate: Option<std::io::Result<data::DelayedMeasurement<data::CPULoad>>> = None;

        TokioInterval::new(1000).map(move |_| {
          println!("TokioInterval::map called");

          if aggregate.is_some() {
            let ret = match aggregate.as_ref().unwrap() {
              Ok(cpu) => {
                let cpu = cpu.done().unwrap();
                Some(
                  SystemStat {
                    user: cpu.user * 100.0,
                    nice: cpu.nice * 100.0,
                    system: cpu.system * 100.0,
                    interrupt: cpu.interrupt * 100.0,
                    idle: cpu.idle * 100.0,
                  }
                  .into(),
                )
              }
              Err(x) => {
                println!("\nCPU load: error: {}", x);
                None
              }
            };
            aggregate = Some(sys.cpu_load_aggregate());
            return ret;
          }
          aggregate = Some(sys.cpu_load_aggregate());
          None
        })
      })
      .await
      .unwrap()
  }

}
