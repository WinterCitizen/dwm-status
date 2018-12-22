use super::Data;
use super::FEATURE_NAME;
use error::*;
use feature;
use io;

const PATH_LOADAVG: &str = "/proc/loadavg";

pub(super) struct Updater {
    data: Data,
}

impl Updater {
    pub(super) fn new(data: Data) -> Self {
        Self { data }
    }
}

impl feature::Updatable for Updater {
    fn renderable(&self) -> Box<&dyn feature::Renderable> {
        Box::new(&self.data)
    }

    fn update(&mut self) -> Result<()> {
        let content = io::read_file(PATH_LOADAVG)
            .wrap_error(FEATURE_NAME, &format!("failed to read {}", PATH_LOADAVG))?;

        let mut iterator = content.split_whitespace();

        let one = convert_to_float(iterator.next())?;
        let five = convert_to_float(iterator.next())?;
        let fifteen = convert_to_float(iterator.next())?;

        self.data.update(one, five, fifteen);

        Ok(())
    }
}

fn convert_to_float(data: Option<&str>) -> Result<f32> {
    data.wrap_error(FEATURE_NAME, "no data found")?
        .parse()
        .wrap_error(FEATURE_NAME, "could not convert to float")
}