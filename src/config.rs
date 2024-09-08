use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "INFLUX_URL")]
    pub influx_url: String,
    #[envconfig(from = "INFLUX_DB")]
    pub influx_db: String,
}
