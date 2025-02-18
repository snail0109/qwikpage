use serde::Serialize;
use std::fmt::Debug;

#[derive(Serialize)]
struct CmdError {
	message: String,
}

#[derive(Serialize)]
pub struct CmdOk<D> where	D: Serialize {
	pub data: D
}

#[derive(Serialize)]
pub struct CmdResponse<D> where	D: Serialize, {
	error: Option<CmdError>,
	result: Option<CmdOk<D>>,
}


impl<D, E> From<Result<D, E>> for CmdResponse<D> where D: Serialize, E: Debug {
	fn from(res: Result<D, E>) -> Self {
		match res {
			Ok(data) => CmdResponse {
				error: None,
				result: Some(CmdOk { data }),
			},
			Err(err) => CmdResponse {
				error: Some(CmdError {
					message: format!("{:?}", err)
				}),
				result: None,
			},
		}
	}
}
