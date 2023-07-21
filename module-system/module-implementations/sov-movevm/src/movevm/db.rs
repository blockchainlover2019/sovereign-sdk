use std::convert::Infallible;

use sov_state::WorkingSet;
// move crates
use move_core_types::{
  account_address::AccountAddress, identifier::Identifier,
  language_storage::{ModuleId, StructTag},
  resolver::{ModuleResolver, MoveResolver, ResourceResolver},
};

pub(crate) struct MovevmDB<'a, C: sov_modules_api::Context> {
    pub(crate) modules: sov_state::StateMap<ModuleId, Vec<u8>>,
    pub(crate) resources: sov_state::StateMap<ResourceKey, Vec<u8>>,
    pub(crate) working_set: &'a mut WorkingSet<C::Storage>,
}

impl<'a, C: sov_modules_api::Context> MovevmDB<'a, C> {
    pub(crate) fn new(
        modules: sov_state::StateMap<ModuleId, Vec<u8>>,
        resources: sov_state::StateMap<ResourceKey, Vec<u8>>,
        working_set: &'a mut WorkingSet<C::Storage>,
    ) -> Self {
        Self {
            modules,
            resources
            working_set,
        }
    }
}

fn get_resource_key(addr: &AccountAddress, tag: &StructTag) -> ResourceKey {
  let addr = AccountAddress::new(addr.to_vec());
  ResourceKey::new(addr, *tag)
}

impl MoveResolver for MovevmDB {
  type Error = ();

  fn get_module(&self, module_id: &ModuleId) -> Result<Option<Vec<u8>>, Self::Error> {
      Ok(self.modules.get(module_id).cloned(), &self.working_set);
  }

  fn get_resource(
    &self,
    address: &AccountAddress,
    tag: &StructTag,
  ) -> Result<Option<Vec<u8>>, Self::Error> {
    let resource_key = get_resource_key(address, tag);
    Ok(self.resources.get(resource_key).cloned(), &self.working_set)
  }
}