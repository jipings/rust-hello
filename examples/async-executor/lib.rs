pub mod lazy {
    use std::{
        cell::UnsafeCell,
        mem::{
            self,
            MaybeUninit,
        },
        sync::Once
    };

    pub struct Lazy<T> {
        once: Once,
        cell: UnsafeCell<MaybeUninit<T>>,
    }

    impl<T> Lazy<T> {
        pub const fn new() -> Self {
            Self {
                once: Once::new(),
                cell: UnsafeCell::new(MaybeUninit::uninit()),
            }
        }
        fn is_initialized(&self) -> bool {
            self.once.is_completed()
        }
        pub fn get_or_init(&self, func: fn() -> T) -> &T {
            self.once.call_once(|| {
                (unsafe {&mut *self.cell.get() }).write(func());
            });
            unsafe { &(*self.cell.get()).assume_init_ref() }
        }
    }

    impl<T> Drop for Lazy<T> {
        fn drop(&mut self) {
            if self.is_initialized() {
                let old = mem::replace(unsafe { &mut *self.cell.get() }, MaybeUninit::uninit());
                drop(unsafe { old.assume_init() });
            }
        } 
    }

    unsafe impl<T: Send> Send for Lazy<T> {}
    unsafe impl<T: Send + Sync> Sync for Lazy<T> {}
}

pub mod runtime {
    use std::{collections::LinkedList, future::Future, pin::Pin, sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
            Mutex,
        }, task::{
            Context,
            Poll,
            Wake,
            Waker,
        }};
            
    pub(crate) struct Runtime {
        queue: Queue,
        spawner: Spawner,
        tasks: AtomicUsize,
    }

    impl Runtime {
        fn start() {
            std::thread::spawn(|| loop {
                let task = match Runtime::get().queue.lock().unwrap().pop_front() {
                    Some(task) => task,
                    None => continue,
                };
                if task.will_block() {
                    while let Poll::Pending = task.poll() {}
                } else {
                    if let Poll::Pending = task.poll() {
                        task.wake();
                    }
                }
            });
        }
        pub(crate) fn get() -> &'static Runtime {
            RUNTIME.get_or_init(setup_runtime)
        }
        pub(crate) fn spawner() -> Spawner {
            Runtime::get().spawner.clone()
        }
    }

    fn setup_runtime() -> Runtime {
        Runtime::start();
        let queue = Arc::new(Mutex::new(LinkedList::new()));
        Runtime {
            spawner: Spawner {
                queue: queue.clone(),
            },
            queue,
            tasks: AtomicUsize::new(0),
        }
    }

    static RUNTIME: crate::lazy::Lazy<Runtime> = crate::lazy::Lazy::new();

    type Queue = Arc<Mutex<LinkedList<Arc<Task>>>>;

    #[derive(Clone)]
    pub(crate) struct Spawner {
        queue: Queue,
    }

    impl Spawner {
        fn spawn(self, future: impl Future<Output = ()> + Send + Sync + 'static) {
            self.inner_spawn(Task::new(false, future));
        }
        fn spawn_blocking(self, future: impl Future<Output = ()> + Send + Sync + 'static) {
            self.inner_spawn_blocking(Task::new(true, future));
        }
        fn inner_spawn(self, task: Arc<Task>) {
            self.queue.lock().unwrap().push_back(task);
        }
        fn inner_spawn_blocking(self, task: Arc<Task>) {
            self.queue.lock().unwrap().push_front(task);
        }
    }

    pub fn spawn(future: impl Future<Output = ()> + Send + Sync +'static) {
        Runtime::spawner().spawn(future);
    }

    pub fn block_on(future: impl Future<Output = ()> + Send + Sync + 'static) {
        Runtime::spawner().spawn_blocking(future);
    }

    pub fn wait() {
        let runtime = Runtime::get();
        while runtime.tasks.load(Ordering::Relaxed) > 0 {}
    }

    struct Task {
        funture: Mutex<Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>>,
        block: bool,
    }

    impl Task {
        fn new(block: bool, future: impl Future<Output = ()> + Send + Sync +'static) -> Arc<Self> {
            Runtime::get().tasks.fetch_add(1, Ordering::Relaxed);
            Arc::new(Task {
                funture: Mutex::new(Box::pin(future)),
                block,
            })
        }

        fn waker(self: &Arc<Self>) -> Waker {
            self.clone().into()
        }

        fn poll(self: &Arc<Self>) -> Poll<()> {
            let waker = self.waker();
            let mut ctx = Context::from_waker(&waker);
            self.funture.lock().unwrap().as_mut().poll(&mut ctx)
        }

        fn will_block(&self) -> bool {
            self.block
        }
    }

    impl Drop for Task {
        fn drop(&mut self) {
            Runtime::get().tasks.fetch_sub(1, Ordering::Relaxed);
        }
    }

    impl Wake for Task {
        fn wake(self: Arc<Self>) {
            if self.will_block() {
                Runtime::spawner().inner_spawn_blocking(self);
            } else {
                Runtime::spawner().inner_spawn(self);
            }
        }
    }
}

pub mod futures {
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
        time::SystemTime,
    };

    pub struct Sleep {
        now: SystemTime,
        ms: u128,
    }

    impl Sleep {
        pub fn new(ms: u128) -> Self {
            Self {
                now: SystemTime::now(),
                ms,
            }
        }
    }

    impl Future for Sleep {
        type Output = ();

        fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<Self::Output> {
            if self.now.elapsed().unwrap().as_millis() >= self.ms {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }
}

#[test]
    fn library_test() {
        use crate::{futures::Sleep, runtime};
        use rand::Rng;
        use std::time::SystemTime;

        runtime::block_on(async {
            const SECOND: u128 = 1000;
            println!("Begin Asynchronous Execution");
            let mut rng = rand::thread_rng();

            let time = || {
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            };

            for i in 0..5 {
                let random = rng.gen_range(1..10);
                let random2 = rng.gen_range(1..10);

                runtime::spawn(async move {
                    println!("Spawned Fn #{:02}: Inner {}", i, time());
                    Sleep::new(SECOND * random).await;

                    runtime::spawn(async move {
                        Sleep::new(SECOND * random2).await;
                        println!("Spawned Fn #{:02}: Inner {}", i, time());
                    });
                    println!("Spawned Fn #{:02}: Ended {}", i, time());
                });
            }
            runtime::block_on(async {
                Sleep::new(11000).await;
                println!("Blocking Function Polled To Completion");
            });
        });

        runtime::wait();
        println!("End of Asynchronous Execution");
    }
