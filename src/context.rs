use {
  crate::{
    config::Config,
    instance::Instance,
    instance_state::InstanceState,
    package_json::{FormatMismatch, FormatMismatchVariant},
    packages::Packages,
    semver_group::SemverGroup,
    version_group::VersionGroup,
  },
  std::{cell::RefCell, collections::HashMap, rc::Rc},
};

#[derive(Debug)]
pub struct Context {
  /// All default configuration with user config applied
  pub config: Config,
  /// All formatting issues in package.json files
  pub formatting_mismatches_by_variant: RefCell<HashMap<FormatMismatchVariant, Vec<Rc<FormatMismatch>>>>,
  /// Every instance in the project
  pub instances: Vec<Rc<Instance>>,
  /// Every package.json in the project
  pub packages: Packages,
  /// All semver groups
  pub semver_groups: Vec<SemverGroup>,
  /// All version groups, their dependencies, and their instances
  pub version_groups: Vec<VersionGroup>,
}

impl Context {
  pub fn create(config: Config, packages: Packages) -> Self {
    let mut instances = vec![];
    let semver_groups = config.rcfile.get_semver_groups();
    let version_groups = config.rcfile.get_version_groups(&packages);

    packages.get_all_instances(&config, |instance| {
      let instance = Rc::new(instance);
      instances.push(Rc::clone(&instance));
      if let Some(semver_group) = semver_groups.iter().find(|semver_group| semver_group.selector.can_add(&instance)) {
        instance.set_semver_group(semver_group);
      }
      if let Some(version_group) = version_groups
        .iter()
        .find(|version_group| version_group.selector.can_add(&instance))
      {
        version_group.add_instance(instance);
      }
    });

    Self {
      config,
      formatting_mismatches_by_variant: RefCell::new(HashMap::new()),
      instances,
      packages,
      semver_groups,
      version_groups,
    }
  }

  /// Quit with the correct exit code based on the validity of each instance
  pub fn exit_program(&self) -> ! {
    if self.config.cli.options.inspect_mismatches {
      for instance in self.instances.iter() {
        match *instance.state.borrow() {
          InstanceState::Valid(_) => continue,
          _ => std::process::exit(1),
        }
      }
    }
    if self.config.cli.options.inspect_formatting {
      for package in self.packages.all.iter() {
        if !package.borrow().formatting_mismatches.borrow().is_empty() {
          std::process::exit(1);
        }
      }
    }
    std::process::exit(0);
  }
}
