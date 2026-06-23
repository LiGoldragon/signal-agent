use schema_rust::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "signal-agent",
        "0.2.0",
        "SIGNAL_AGENT_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}
