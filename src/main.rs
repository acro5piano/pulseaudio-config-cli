use libpulse_binding::{
    context::{Context, FlagSet, State},
    mainloop::threaded::Mainloop,
};

fn main() {
    pretty_env_logger::init();
    let mut mainloop = Mainloop::new().unwrap();
    let mut context = Context::new(&mainloop, "pulseaudio-config-cli").unwrap();
    mainloop.start().unwrap();
    context.connect(None, FlagSet::NOFLAGS, None).unwrap();

    // wait for context to be ready
    loop {
        match context.get_state() {
            State::Ready => {
                break;
            }
            State::Failed | State::Terminated => {
                panic!("Unable to connect PulseAudio");
            }
            _ => {
                mainloop.wait();
            }
        }
    }
    log::debug!("[PAInterface] PAContext ready");

    // Not run...
    context.device_manager().read(|devices| {
        dbg!(&devices);
    });

    // dbg!(&context);
    println!("Hello, world!");
}
