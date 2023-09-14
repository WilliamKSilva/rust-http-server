use std::thread::Builder;

pub struct ThreadPool {
  size: usize,
  running: Vec<Builder>,
  avaiable: Vec<Builder>
}

impl ThreadPool {
  pub fn new() -> ThreadPool {
    let mut threads: Vec<Builder> = Vec::new();

    for n in 0..2 {
      let thread = std::thread::Builder::new().name(format!("thread{:?}", n.to_string()));

      threads.push(thread);
    }

    let size = threads.len();

    return ThreadPool {
      avaiable: threads,
      running: Vec::new(),
      size
    }
  }

  pub fn size(&mut self) -> usize {
    return self.size
  }

  pub fn first_avaiable(&mut self) -> Option<&Builder> {
    return self.avaiable.first().clone();
  }

  pub fn running(&mut self) -> Option<&Builder> {
    return self.running.last();
  }
}