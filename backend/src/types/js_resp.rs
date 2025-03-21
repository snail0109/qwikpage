use serde::Serialize;
use std::fmt::Debug;

#[derive(Serialize)]
pub struct JSResp<D>
where
    D: Serialize,
{
    message: Option<String>,
    data: Option<D>,
	success: bool,
}

impl<D, E> From<Result<D, E>> for JSResp<D>
where
    D: Serialize,
    E: Debug,
{
    fn from(res: Result<D, E>) -> Self {
        match res {
            Ok(data) => JSResp {
                message: None,
                data: Some(data),
				success: true
            },
            Err(err) => JSResp {
                message: Some(format!("{:?}", err)),
                data: None,
				success: false
            },
        }
    }
}
