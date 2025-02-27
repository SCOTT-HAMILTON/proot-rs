#[macro_use]
mod macros;
mod elf;
pub mod enter;
pub mod exit;
mod load_info;
mod loader;
mod shebang;

use crate::errors::Result;
use crate::kernel::execve::loader::LoaderFile;
use crate::kernel::exit::SyscallExitResult;
use crate::process::tracee::Tracee;

pub fn enter(tracee: &mut Tracee, loader: &dyn LoaderFile) -> Result<()> {
    enter::translate(tracee, loader)
}

pub fn exit(tracee: &mut Tracee) -> SyscallExitResult {
    exit::translate(tracee)
}
