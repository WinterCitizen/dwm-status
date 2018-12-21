use error::*;
use std::thread;
use std::time;

pub(crate) trait Runnable {
    fn run(&self) -> Result<()>;
}

pub(crate) struct Thread<R> {
    name: &'static str,
    runnable: R,
}

impl<R> Thread<R>
where
    R: Runnable + Send + 'static,
{
    pub(crate) fn new(name: &'static str, runnable: R) -> Self {
        Self { name, runnable }
    }

    pub(crate) fn run(self) -> Result<()> {
        thread::Builder::new()
            .name(String::from(self.name))
            .spawn(move || loop {
                let _ = self.runnable.run().show_error();
                sleep_secs(2);
            })
            .wrap_error("thread start", "failed to create thread")?;

        Ok(())
    }
}

pub(crate) fn sleep_secs(seconds: u64) {
    thread::sleep(time::Duration::from_secs(seconds));
}

pub(crate) fn sleep_prevent_spam() {
    thread::sleep(time::Duration::from_millis(100));
}