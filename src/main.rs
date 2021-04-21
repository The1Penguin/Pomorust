extern crate clap;
use clap::{App};

fn main() {
    let args = App::new("pomorust")
	.version("0.1.0")
	.about("pomodoro timer written in rust")
	.author("ThePenguin")
    .args_from_usage("
        -w, --work=[TIME] 'Sets your work time in minutes'
        -r, --rest=[TIME] 'Sets your rest time in minutes'
        -l, --long=[TIME] 'Sets your long rest time in minutes'
       ")
	.get_matches();

}
