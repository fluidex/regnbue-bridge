fn main() {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["dingir-exchange/proto/exchange/matchengine.proto"],
            &["dingir-exchange/proto/exchange", "dingir-exchange/proto/third_party/googleapis"],
        )
        .unwrap()
}
