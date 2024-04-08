use std::sync::atomic::{AtomicUsize, Ordering};

pub trait AtomicCounter: Send + Sync {
  type PrimitiveType;

  fn get(&self) -> Self::PrimitiveType;
  fn increase(&self) -> Self::PrimitiveType;
  fn add(&self, count: Self::PrimitiveType) -> Self::PrimitiveType;
  fn reset(&self) -> Self::PrimitiveType;
  fn into_inner(self) -> Self::PrimitiveType;
}

#[derive(Default, Debug)]
pub struct ConsistentCounter(AtomicUsize);

impl ConsistentCounter {
  pub fn new(init_num: usize) -> ConsistentCounter {
    ConsistentCounter(AtomicUsize::new(init_num))
  }
}

impl AtomicCounter for ConsistentCounter {
  type PrimitiveType = usize;

  fn get(&self) -> Self::PrimitiveType {
    self.0.load(Ordering::SeqCst)
  }

  fn increase(&self) -> Self::PrimitiveType {
    self.add(1)
  }

  fn add(&self, count: Self::PrimitiveType) -> Self::PrimitiveType {
    self.0.fetch_add(count, Ordering::SeqCst)
  }

  fn reset(&self) -> Self::PrimitiveType {
    self.0.swap(0, Ordering::SeqCst)
  }

  fn into_inner(self) -> Self::PrimitiveType {
    self.0.into_inner()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::{fmt::Debug, ops::Deref, sync::Arc, thread};

  const NUM_THREADS: usize = 29;
  const NUM_ITERATIONS: usize = 7_000_000;

  fn test_simple<Counter>(counter: Counter)
  where
    Counter: AtomicCounter<PrimitiveType = usize>,
  {
    counter.reset();
    assert_eq!(0, counter.add(5));
    assert_eq!(5, counter.increase());
    assert_eq!(6, counter.get())
  }

  fn test_increase<Counter>(counter: Arc<Counter>)
  where
    Counter: AtomicCounter<PrimitiveType = usize> + Debug + 'static,
  {
    println!("[+] test_increase: Spawning {} thread, each with {}", NUM_THREADS, NUM_ITERATIONS);
    let mut join_handles = Vec::new();
    // 创建 NUM_THREADS 个线程，同时使用 increase 函数
    for _ in 0..NUM_THREADS {
      let counter_ref = counter.clone();
      join_handles.push(thread::spawn(move || {
        let counter: &Counter = counter_ref.deref();
        for _ in 0..NUM_ITERATIONS {
          counter.increase();
        }
      }));
    }
    // 等待线程完成
    for handle in join_handles {
      handle.join().unwrap();
    }
    let count = Arc::try_unwrap(counter).unwrap().into_inner();
    let excepted_num = NUM_ITERATIONS * NUM_THREADS;
    println!("[+] test_increase: get count {}, excepted num is {}", count, excepted_num);
    // 确定 count 正确
    assert_eq!(count, excepted_num)
  }

  fn test_reset<Counter>(counter: Arc<Counter>)
  where
    Counter: AtomicCounter<PrimitiveType = usize> + Debug + 'static,
  {
    println!("[+] test_reset: Spawning {} thread, each with {}", NUM_THREADS, NUM_ITERATIONS);
    let mut excepted_num = 0;
    for i in 0..NUM_THREADS {
      excepted_num += i * NUM_ITERATIONS;
    }

    let counter_ref = counter.clone();
    // 一直运行 reset 重置并获取已有的值
    let reset_handle = thread::spawn(move || {
      let mut total_count = 0;
      let counter = counter_ref.deref();
      while total_count < excepted_num {
        total_count += counter.reset();
      }

      total_count
    });

    let mut join_handles = Vec::new();
    // 创建 NUM_THREADS 个线程，同时使用 add 函数
    for to_add in 0..NUM_THREADS {
      let counter_ref = counter.clone();
      join_handles.push(thread::spawn(move || {
        let counter: &Counter = counter_ref.deref();
        for _ in 0..NUM_ITERATIONS {
          counter.add(to_add);
        }
      }));
    }

    for handle in join_handles {
      handle.join().unwrap();
    }
    let result = reset_handle.join().unwrap();
    // NUM_THREADS 个线程，NUM_ITERATIONS 次循环，每次循环都加上线程 id。
    println!("[+] test_increase: get count {}, excepted num is {}", result, excepted_num);
    assert_eq!(result, excepted_num)
  }

  #[test]
  fn it_works() {
    test_simple(ConsistentCounter::new(10));
    test_increase(Arc::new(ConsistentCounter::new(0)));
    test_reset(Arc::new(ConsistentCounter::new(0)));
  }
}
