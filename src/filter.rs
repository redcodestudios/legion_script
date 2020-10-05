use legion::{
        query::{LayoutFilter, FilterResult,DynamicFilter, Fetch, Any, GroupMatcher}, 
        world::WorldId
    };
use legion::storage::ComponentTypeId;

#[derive(Default)]
pub struct ExternalLayoutFilter;

impl LayoutFilter for ExternalLayoutFilter {
    fn matches_layout(&self, components: &[ComponentTypeId]) -> FilterResult {
        println!("matches_layout - start");
        let result = FilterResult::Match(components.is_empty());
        println!("matches_layout - end");
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

impl DynamicFilter for ExternalDynFilter{
    fn prepare(&mut self, world_id: WorldId) {
        self.filter.prepare(world_id);
    }
    fn matches_archetype<T:Fetch>(&mut self, fetch: &T) -> FilterResult{
        println!("matches_archetype - start");
        let result = FilterResult::Match(true);
        println!("matches_archetype - end");
        result
    }
}
