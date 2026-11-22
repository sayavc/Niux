use knus::Decode;

#[derive(Decode)]
pub struct HookConfig {
    #[knus(children(name = "actions"))]
    pub actions: Vec<Actions>,
}

#[derive(Decode)]
pub struct Actions {
    #[knus(child, unwrap(argument))]
    pub action: String,
    #[knus(child, unwrap(argument))]
    pub run: String,
}
