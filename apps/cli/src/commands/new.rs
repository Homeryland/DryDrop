use usage_rs::{Args, Run};

#[derive(Args)]
pub struct New {
    #[usage(default = "playground")]
    project_name: String,
}

impl Run for New {
    type Output = ();
    fn run(self) {
        println!("hello, {}", self.project_name);
    }
}
