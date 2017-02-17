//#![feature(alloc_system)]

use std::process::Command;
use std::path::Path;
use std::env;

fn help() {
    println!("/*\\");
    println!("|*| Синхронизация бекапа корневой файловой системы");
    println!("|*|");
    println!("|*| Использование:");
    println!("|*| root-backup [directory] [-h|--help]");
    println!("\\*/");
}

fn backup(dir: &str) {
    // Киррилические символы в переменной dir неправильные,
    // вида: \#320\#237\#320\#260\#320\#274\#321\#217\#321\#202\#321\#214,
	// а без киррилицы работает нормально
	println!();
	let mut line = String::new();
	line += "sudo rsync -vaHAXS / --exclude={\"/dev/\",\"/proc/\",\"/mnt/\",\"/media/\",\"/run/\",\"/sys/\",\"tmp/*\",\"temp/*\",\"cache/*\",\".cache/*\",\"log\",\"history\",\"lost+found/*\"} ";
	line += dir;
	line += "/ --delete --delete-excluded";
	let mut command = Command::new("sh").arg("-c").arg(line)
		.spawn().expect("Ошибка выполнения!");
	let ecode = command.wait().expect("Ошибка выполнения команды bash!");
	assert!(ecode.success());
	println!();
}

fn main() {
    let mut execute = true;
    let mut dir = String::new();
    for argument in env::args() {
        match argument.as_ref() {
            "-h" | "--help" => {
                execute = false;
                help();
            },
            _ => {
                dir = argument;
            }
        }
    }
    if execute {
        if Path::new(dir.as_str()).is_dir() { backup(dir.as_str()); }
        else { help(); }
    }
}
