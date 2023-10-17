use postgres::Client;

pub struct Tasker {
    client: Client,
    tasks: Vec<Box<dyn Task>>,
}

pub trait Task {
    fn exec(&mut self, client: &mut Client);
}

impl Tasker {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            tasks: Vec::new(),
        }
    }

    pub fn append_task(&mut self, task: Box<dyn Task>) {
        self.tasks.push(task.into());
    }

    pub fn run(mut self) {
        for mut task in self.tasks {
            task.exec(&mut self.client);
        }
    }
}
