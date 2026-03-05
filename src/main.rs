use nu_plugin_audio::Sound;

fn main() {
    let _ = env_logger::try_init();
    nu_plugin::serve_plugin(&Sound {}, nu_plugin::MsgPackSerializer {})
}
