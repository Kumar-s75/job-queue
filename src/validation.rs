use crate::error::AppError;
fn identifier(value:&str,label:&str,max:usize)->Result<(),AppError>{if value.is_empty()||value.len()>max||!value.chars().all(|c|c.is_ascii_alphanumeric()||matches!(c,'-'|'_'|'.')){Err(AppError::BadRequest(format!("invalid {label}")))}else{Ok(())}}
pub fn queue(value:&str)->Result<(),AppError>{identifier(value,"queue",128)}
pub fn worker(value:&str)->Result<(),AppError>{identifier(value,"worker_id",128)}
