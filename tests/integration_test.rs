//! Integration tests for SigmaOS-arch-elixir.

use sigmaos_arch_elixir_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
