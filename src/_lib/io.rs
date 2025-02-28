use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process::{exit, Command};
use colored::{ColoredString, Colorize};

/// # 文件读取器
pub struct FileWrapper {
	works: bool,
	path: String,
	file: Option<io::BufReader<fs::File>>,
	line: FileLine,
}

impl FileWrapper {
	pub fn new(path: String) -> Self {
		let file = fs::File::open(&path);
		Self {
			works: file.is_ok(),
			path: (&path).to_string(),
			file: {
				Some(io::BufReader::new({
					match file {
						Ok(file) => file,
						Err(_) => return Self {
							works: false,
							path,
							file: None,
							line: FileLine::new(0, "".to_string()),
						},
					}
				}))
			},
			line: FileLine::new(0, "".to_string()),
		}
	}

	/// ## 对象正常
	pub fn works(&mut self) -> bool {
		self.works
	}

	/// ## 获取文件名称
	pub fn path(&self) -> String {
		self.path.clone()
	}

	/// ## 获取当前行数
	pub fn reading(&self) -> FileLine {
		self.line.clone()
	}

	/// ## 读取下一行
	pub fn next(&mut self) -> Option<FileLine> {
		if self.works {
			if let Some(line) = self.file.as_mut().unwrap().lines().next() {
				self.line = FileLine::new(self.line.number() + 1, line.unwrap());
				return Some(self.line.clone());
			}
			return None;
		} else {
			None
		}
	}

	/// ## 读取文件内容
	pub fn read_to_string(&mut self) -> io::Result<String> {
		fs::read_to_string(self.path())
	}
}

/// # 文件行
pub struct FileLine {
	number: u128,
	data: String,
}

impl FileLine {
	pub fn new(number: u128, data: String) -> Self {
		Self { number, data }
	}
	/// ## 获取行号
	pub fn number(&self) -> u128 {
		self.number
	}
	/// ## 获取行数据
	pub fn data(&self) -> String {
		self.data.clone()
	}
	/// ## 克隆
	pub fn clone(&self) -> Self {
		Self {
			number: self.number,
			data: self.data.clone(),
		}
	}
}

/// # 日志
pub struct Log {
	_type: ColoredString,
	_msg: String,
	_stop: Option<i32>,
}

pub enum LogType {
	Err,
	Info,
	Warn,
}

impl Log {
	/// ## 创建新的日志输出
	/// `_type` 错误的类型： e:错误 i:表示信息 w:表示警告 <br />
	/// `_msg` 消息正文 <br />
	/// `stop_key` 是退出程序的代码（如果错误是致命的） <br />如果为`0`则不退出程序，继续运行
	pub fn new(_type: LogType, _msg: &str, stop_key: i32) -> Self {
		Self {
			_type: match _type {
				LogType::Err => "错误".red(),
				LogType::Info => "信息".bold(),
				LogType::Warn => "警告".yellow(),
			},
			_msg: _msg.to_string(),
			_stop: if stop_key != 0 { Option::from(stop_key) } else { None },
		}
	}

	/// ## 打印日志
	/// 同一类型
	pub fn print(&self) {
		if !self._stop.is_none() {
			eprint!("{}: {}", self._type, self._msg);
			exit(self._stop.unwrap());
		} else {
			println!("{}: {}", self._type, self._msg);
		}
	}
}