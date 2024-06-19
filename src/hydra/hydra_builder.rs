#[derive(Clone, Debug, PartialEq)]
pub struct HydraInstance {
    pub hydra_url: String,
    pub project: String,
    pub jobset: Option<String>,
    pub build: Option<u64>,
    pub eval: Option<u64>,
}

impl HydraInstance {
    pub fn builder() -> HydraInstanceBuilder {
        HydraInstanceBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct HydraInstanceBuilder {
    hydra_url: String,
    project: String,
    jobset: Option<String>,
    build: Option<u64>,
    eval: Option<u64>,
}

impl HydraInstanceBuilder {
    pub fn new() -> HydraInstanceBuilder {
        HydraInstanceBuilder {
            hydra_url: String::from("https://hydra.nixos.org"),
            project: String::from("nixos"),
            jobset: None,
            build: None,
            eval: None,
        }
    }

    pub fn hydra_url(mut self, hydra_url: String) -> HydraInstanceBuilder {
        self.hydra_url = hydra_url;
        self
    }

    pub fn project(mut self, project: String) -> HydraInstanceBuilder {
        self.project = project;
        self
    }

    pub fn jobset(mut self, jobset: String) -> HydraInstanceBuilder {
        self.jobset = Some(jobset);
        self
    }

    pub fn eval(mut self, eval: u64) -> HydraInstanceBuilder {
        self.eval = Some(eval);
        self
    }

    pub fn build_id(mut self, build: u64) -> HydraInstanceBuilder {
        self.build = Some(build);
        self
    }

    pub fn build(self) -> HydraInstance {
        HydraInstance {
            hydra_url: self.hydra_url,
            project: self.project,
            jobset: self.jobset,
            build: self.build,
            eval: self.eval,
        }
    }
}

#[test]
fn builder_test_default() {
    let nixpkgs = HydraInstance {
        hydra_url: String::from("https://hydra.nixos.org"),
        project: String::from("nixos"),
        jobset: None,
        build: None,
        eval: None,
    };
    let nixpkgs_from_builder = HydraInstanceBuilder::new().build();
    assert_eq!(nixpkgs, nixpkgs_from_builder);
}

#[test]
fn builder_test_jobset() {
    let nixpkgs = HydraInstance {
        hydra_url: String::from("https://hydra.nixos.org"),
        project: String::from("nixos"),
        jobset: Some(String::from("unstable-small")),
        build: None,
        eval: None,
    };
    let nixpkgs_from_builder = HydraInstanceBuilder::new()
        .jobset(String::from("unstable-small"))
        .build();
    assert_eq!(nixpkgs, nixpkgs_from_builder);
}

#[test]
fn builder_test_build() {
    let nixpkgs = HydraInstance {
        hydra_url: String::from("https://hydra.nixos.org"),
        project: String::from("nixos"),
        jobset: None,
        build: Some(2),
        eval: None,
    };
    let nixpkgs_from_builder = HydraInstanceBuilder::new().build_id(2).build();
    assert_eq!(nixpkgs, nixpkgs_from_builder);
}

#[test]
fn builder_test_eval() {
    let nixpkgs = HydraInstance {
        hydra_url: String::from("https://hydra.nixos.org"),
        project: String::from("nixos"),
        jobset: None,
        build: None,
        eval: Some(6),
    };
    let nixpkgs_from_builder = HydraInstanceBuilder::new().eval(6).build();
    assert_eq!(nixpkgs, nixpkgs_from_builder);
}

#[test]
fn builder_test_custom_url() {
    let nixpkgs = HydraInstance {
        hydra_url: String::from("https://hydra.alicehuston.xyz"),
        project: String::from("nixos"),
        jobset: None,
        build: None,
        eval: None,
    };
    let nixpkgs_from_builder = HydraInstanceBuilder::new()
        .hydra_url(String::from("https://hydra.alicehuston.xyz"))
        .build();
    assert_eq!(nixpkgs, nixpkgs_from_builder);
}

#[test]
fn builder_test_custom_project() {
    let nixpkgs = HydraInstance {
        hydra_url: String::from("https://hydra.nixos.org"),
        project: String::from("nixpkgs"),
        jobset: None,
        build: None,
        eval: None,
    };
    let nixpkgs_from_builder = HydraInstanceBuilder::new()
        .project(String::from("nixpkgs"))
        .build();
    assert_eq!(nixpkgs, nixpkgs_from_builder);
}
