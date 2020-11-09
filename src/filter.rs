use legion::storage::ComponentTypeId;
use legion::{
    query::{Any, DynamicFilter, Fetch, FilterResult, GroupMatcher, LayoutFilter},
    world::WorldId,
};

#[derive(Default)]
pub struct ExternalLayoutFilter;
use log::*;

impl LayoutFilter for ExternalLayoutFilter {
    fn matches_layout(&self, components: &[ComponentTypeId]) -> FilterResult {
        trace!("matches_layout - start");
        let result = FilterResult::Match(components.is_empty());
        trace!("matches_layout - end");
        result
    }
}

impl GroupMatcher for ExternalLayoutFilter {
    fn can_match_group() -> bool {
        true
    }

    fn group_components() -> Vec<ComponentTypeId> {
        Vec::new()
    }
}

#[derive(Default)]
struct ExternalDynFilter {
    pub filter: Any,
}

impl DynamicFilter for ExternalDynFilter {
    fn prepare(&mut self, world_id: WorldId) {
        self.filter.prepare(world_id);
    }
    fn matches_archetype<T: Fetch>(&mut self, _fetch: &T) -> FilterResult {
        trace!("matches_archetype - start");
        let result = FilterResult::Match(true);
        trace!("matches_archetype - end");
        result
    }
}
