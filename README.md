# OpenPolicyAgent Bundle proxy

This program acts as a proxy between OpenPolicyAgent (OPA) and an arbitrary HTTP(S) Webserver which provides JSON or YAML files.
These files can be combined into a single or mutiple bundles, as [defined by OPA](https://www.openpolicyagent.org/docs/latest/management/#bundles).

## Usage
In this early alpha version only one bundle with one resource is supported. This can be configured via environment variables or a `.env` file. An example can be found [here](.env).
* `OBP_RESOURCE_URL` the URL where the resource will be fetched
* `OBP_RESOURCE_PATH` the path where the resource will be placed, for example `my/custom/path` can be accessed in OPA via `data.my.custom.path`
* `OBP_RESOURCE_TYPE` the file type of the fetched resource, can be `json` or `yaml`

## Roadmap
A lot of features are still missing. This roadmap contains all key elements, which shall be implemented.
1. Add a `.manifest` file
2. Create multiple named bundles
3. Create bundles with multiple files
4. Make the download async
5. Support etags
6. Add signing via the `opa` command

## Licenses
This project is published under the [MIT License](LICENSE.md).
The direct dependencies are published under the following licenses:

* `rocket` [MIT License](https://github.com/SergioBenitez/Rocket/blob/master/LICENSE-MIT) or [Apache 2.0 License](https://github.com/SergioBenitez/Rocket/blob/master/LICENSE-APACHE)
* `rocket` [MIT License](https://github.com/sd2k/rocket_prometheus/blob/master/LICENSE-MIT) or [Apache 2.0 License](https://github.com/sd2k/rocket_prometheus/blob/master/LICENSE-APACHE)
* `reqwest` [MIT License](https://github.com/seanmonstar/reqwest/blob/master/LICENSE-MIT) or [Apache 2.0 License](https://github.com/seanmonstar/reqwest/blob/master/LICENSE-APACHE)
* `flate2` [MIT License](https://github.com/rust-lang/flate2-rs/blob/master/LICENSE-MIT) or [Apache 2.0 License](https://github.com/rust-lang/flate2-rs/blob/master/LICENSE-APACHE)
* `tar` [MIT License](https://github.com/alexcrichton/tar-rs/blob/master/LICENSE-MIT) or [Apache 2.0 License](https://github.com/alexcrichton/tar-rs/blob/master/LICENSE-APACHE)
* `dotenv` [MIT License](https://github.com/dotenv-rs/dotenv/blob/master/LICENSE.md)
* `serde` [MIT License](https://github.com/serde-rs/serde/blob/master/LICENSE-MIT) or [Apache 2.0 License](https://github.com/serde-rs/serde/blob/master/LICENSE-APACHE)
* `serde_json` [MIT License](https://github.com/serde-rs/json/blob/master/LICENSE-MIT) or [Apache 2.0 License](https://github.com/serde-rs/json/blob/master/LICENSE-APACHE)
