use legion::{
        query::{LayoutFilter, FilterResult,DynamicFilter, EntityFilter, Fetch, Any, GroupMatcher}, 
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


// pub struct ExternalEntityFilter;

// impl EntityFilter for ExternalEntityFilter{
//     type Layout =  ExternalLayoutFilter;
//     type Dynamic = ExternalDynFilter;
//     fn layout_filter(&self) -> &Self::Layout {
//         println!("ExternalEntityFilter layout_filter()");
//         &ExternalLayoutFilter::default()
//     }
    
//     fn filters(&mut self) -> (&Self::Layout, &mut Self::Dynamic){
//         println!("ExternalEntityFilter filters()");
//         (&ExternalLayoutFilter::default(), &mut ExternalDynFilter::default())
//     }

// }

// impl Default for ExternalEntityFilter{
//     fn default() -> Self{
//         println!("ExternalEntityFilter default()");
//         ExternalEntityFilter{}
//     }
// }
