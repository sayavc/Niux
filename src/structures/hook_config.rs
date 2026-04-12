use knuffel;

#[derive(knuffel::Decode)]
pub struct HookConfig {
    #[knuffel(children(name = "actions"))]
    pub actions: Vec<Actions>,
}

#[derive(knuffel::Decode)]
pub struct Actions {
    #[knuffel(child, unwrap(argument))]
    pub action: String,
    #[knuffel(child, unwrap(argument))]
    pub run: String,
}
