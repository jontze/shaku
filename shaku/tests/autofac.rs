//! Examples based on AutoFac 'getting started' example
//! (http://autofac.readthedocs.io/en/latest/getting-started/index.html)

use std::sync::Arc;

use shaku::{Component, Interface};

// IOutput & ConsoleOutput implementation
// ---------------------------------------------------------------------
trait IOutput: Interface {
    fn write(&self, content: String);
    fn get_date(&self, content: String) -> String;
}

#[derive(Component)]
#[shaku(interface = IOutput)]
struct ConsoleOutput {
    prefix: String,
    other_param: usize,
}

impl IOutput for ConsoleOutput {
    fn write(&self, content: String) {
        println!(
            "[Outputting to the console] {} #{} {}",
            self.prefix, self.other_param, content
        );
    }

    fn get_date(&self, content: String) -> String {
        format!("{}#{} {}", self.prefix, self.other_param, content)
    }
}

// IDateWriter & TodayWriter implementation
// ---------------------------------------------------------------------
trait IDateWriter: Interface {
    fn write_date(&self);
    fn get_date(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = IDateWriter)]
struct TodayWriter {
    #[shaku(inject)]
    output: Arc<dyn IOutput>,
    today: String,
}

impl IDateWriter for TodayWriter {
    fn write_date(&self) {
        let mut content = "Today is ".to_string();
        content.push_str(self.today.as_str());
        self.output.write(content);
    }

    fn get_date(&self) -> String {
        let mut content = "Today is ".to_string();
        content.push_str(self.today.as_str());
        self.output.get_date(content)
    }
}

#[test]
fn main_test() {
    // Create your builder.
    let mut builder = shaku::ContainerBuilder::new();

    builder
        .register_type::<ConsoleOutput>()
        .with_named_parameter("prefix", "PREFIX > ".to_string())
        .with_typed_parameter::<usize>(117 as usize);
    builder
        .register_type::<TodayWriter>()
        .with_typed_parameter::<String>("June 19".to_string());
    let container = builder.build().unwrap();

    // The WriteDate method is where we'll make use
    // of our dependency injection. We'll define that
    // in a bit.

    let writer = container.resolve::<dyn IDateWriter>().unwrap();
    writer.write_date();
    let date = writer.get_date();
    assert_eq!(date, "PREFIX > #117 Today is June 19");
}
