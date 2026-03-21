use windows::core::Result;

trait D3DApp {}

extern "system" fn wndproc<S: D3DApp>() {}

fn main() -> Result<()> {
    unsafe {}

    Ok(())
}
