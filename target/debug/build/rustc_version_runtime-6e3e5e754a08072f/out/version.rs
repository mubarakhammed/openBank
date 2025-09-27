
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 87,
                        patch: 0,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "x86_64-apple-darwin".to_owned(),
                    short_version_string: "rustc 1.87.0 (17067e9ac 2025-05-09)".to_owned(),
                    commit_hash: Some("17067e9ac6d7ecb70e50f92c1944e545188d2359".to_owned()),
                    commit_date: Some("2025-05-09".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            