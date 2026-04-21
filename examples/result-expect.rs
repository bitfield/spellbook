fn main() {
    try_deploy_solar_panels().expect("mission terminated");
}

fn try_deploy_solar_panels() -> Result<(), &'static str> {
    Err("panels got stuck")
}
