use anyhow::Result;
use crate::ipc::Request;
use super::transport;
use super::output::{resolve, print_value};

pub fn cmd_whoami(json: bool) -> Result<()> {
    let resp = transport::send(Request::Whoami)?;
    print_value(&resp.data, &resolve(json))
}
