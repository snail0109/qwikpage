use serde::Serialize;
use std::fmt::Debug;

#[derive(Serialize)]
pub struct CmdResponse<D>
where
    D: Serialize,
{
    message: Option<String>,
    data: Option<D>,
	success: bool,
}

impl<D, E> From<Result<D, E>> for CmdResponse<D>
where
    D: Serialize,
    E: Debug,
{
    fn from(res: Result<D, E>) -> Self {
        match res {
            Ok(data) => CmdResponse {
                message: None,
                data: Some(data),
				success: true
            },
            Err(err) => CmdResponse {
                message: Some(format!("{:?}", err)),
                data: None,
				success: false
            },
        }
    }
}
