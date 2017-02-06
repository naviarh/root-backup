//#![feature(alloc_system)]

use std::process::Command;
use std::path::Path;
use std::io::Write;


fn main()
{
	println!();
	println!();
	println!("_________________");
	println!();
	println!("Синхронизация бэкапа корневой файловой системы");
	println!();
	println!();
	print!("Введи путь к папке бэкапа (/run/media/Disk/arch-backup?): ");
	std::io::stdout().flush().expect("Ошибка вопроса!");
	let mut input:String = String::new();
	std::io::stdin().read_line(&mut input).ok().expect("Ошибка ввода!");
	if input.len() == 1
	{
		input = String::from("/run/media/Disk/arch-backup");
		println!("{}",input);
	}
	let dir = input.trim_right();
	if Path::new(dir).is_dir()
	{
		// Киррилические символы в переменной dir неправильные,
		// вида: \#320\#237\#320\#260\#320\#274\#321\#217\#321\#202\#321\#214,
		// а без киррилицы работает нормально
		println!();
		println!("Бэкап синхронизируется...");
		println!();
		println!();
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
		println!();
		println!("_________________");
		println!();
		println!("Синхронизация выполнена");
		println!();
		println!();
	}
	else
	{
		println!();
		println!("Такой папки не существует!");
		println!();
	}
}
