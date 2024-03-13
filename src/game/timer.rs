use std::time::Duration;
use std::{cell::RefCell, time::Instant};

/// Contains a timer for keeping track of if an event is finished.
#[derive(Debug)]
pub struct Timer {
  timer: RefCell<Option<Instant>>,
  duration: Duration,
}

impl Timer {
  pub fn new(duration: Duration) -> Self {
    let timer = RefCell::new(None);

    Self { timer, duration }
  }

  /// Starts the timer.
  ///
  /// If the timer is already running, this resets it. If you want to be more clear that you're restarting,
  /// use the [`restart()`][Timer::restart] method.
  pub fn start(&self) {
    *self.timer.borrow_mut() = Some(Instant::now())
  }

  /// This method just calls [`start()`][Timer::start] internally.
  ///
  /// It's mostly meant for clarity if you're trying to restart a timer and want to be clear about that.
  pub fn restart(&self) {
    self.start()
  }

  /// Stops the timer if it has finished, and returns the status.
  pub fn is_finished(&self) -> bool {
    if self.timer.borrow().is_none() {
      return false;
    }

    let timer_finished = self.timer.borrow().unwrap().elapsed() >= self.duration;

    if timer_finished {
      *self.timer.borrow_mut() = None;
    }

    timer_finished
  }

  /// Returns true if the timer is currently running.
  pub fn running(&self) -> bool {
    self.timer.borrow().is_some()
  }
}
