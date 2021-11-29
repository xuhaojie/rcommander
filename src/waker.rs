use std::io::prelude::*;
use std::net::TcpStream;
use ssh2::Session;

pub struct Waker {
	user_name: String,
	user_password: String,
//	url: String,
	session: Session,
}

impl Waker {
	pub fn new(url: &str, user_name: String, user_password: String) -> Waker {
		let tcp = TcpStream::connect(url).unwrap();
		let mut session = Session::new().unwrap();
		session.set_tcp_stream(tcp);
		session.handshake().unwrap();
		Waker {
			user_name,
			user_password,
//			url: url.to_string(),
			session,

		}
	}

	fn login(self: &mut Self) -> Result<(),String> {

		self.session.userauth_password(&self.user_name, &self.user_password).unwrap();
		assert!(self.session.authenticated());
		Ok(())

	}

	fn execute_command(&self, cmd: &str) -> Result<(),String> {
		let mut channel = self.session.channel_session().unwrap();
		channel.exec(cmd).unwrap();
		let mut s = String::new();
		channel.read_to_string(&mut s).unwrap();
		println!("{}", s);
		match channel.wait_close(){
			Ok(())=>println!("channel closed, ext code = {}",channel.exit_status().unwrap()),
			Err(e) => println!("close channel failed. {}", e),
		}
		Ok(())
	}

	pub fn wake(&mut self, target: &str) -> Result<(),String> {


		self.login()?;

		let cmd = format!("ether-wake -i br0 -b {}", target);
		self.execute_command(cmd.as_str())
	}
}
