use super::super::def::*;


pub fn get_last_error() -> DWord {
  unsafe {
    GetLastError()
  }
}

pub fn get_module_handle(lp_module_name: LPCWStr) -> HInstance {
  unsafe {
    GetModuleHandleW(lp_module_name)
  }
}

#[link(name = "kernel32")]
extern "system" {
  fn GetLastError() -> DWord;
  fn GetModuleHandleW(lp_module_name: LPCWStr) -> HInstance;
}