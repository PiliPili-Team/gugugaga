#[derive(knuffel::Decode, Debug, PartialEq, Default)]
pub struct RegisterConf {
    #[knuffel(child, unwrap(argument), default)]
    pub address: String,
}
