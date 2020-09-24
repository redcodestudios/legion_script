use legion::{
        query::{LayoutFilter, FilterResult,DynamicFilter, EntityFilter, Fetch}, 
        world::WorldId
    };
use legion::storage::ComponentTypeId;
pub struct ExternalLayoutFilter;

impl LayoutFilter for ExternalLayoutFilter {
    fn matches_layout(&self, components: &[ComponentTypeId]) -> FilterResult {
        println!("matches_layout - start");
        let result = FilterResult::Match(components.is_empty());
        println!("matches_layout - end");
        result
    }
}

struct ExternalDynFilter;

impl DynamicFilter for ExternalDynFilter{
    fn prepare(&mut self, world_id: WorldId) {
        self.filter.prepare(world_id);
    }
    fn matches_archetype<T:Fetch>(mut self, fetch: &T)->FilterResult{
        println!("matches_archetype - start");
        let result = FilterResult::Match();
        println!("matches_archetype - end");
        result
    }
}


pub struct ExternalEntityFilter;

impl EntityFilter for ExternalEntityFilter{
    type Layout =  ExternalLayoutFilter;
    type Dynamic = ExternalDynFilter;
    fn layout_filter(&self) -> &Self::Layout {
        println!("ExternalEntityFilter layout_filter()");
        self.Layout
    }
    
    fn filters(&mut self) -> (&Self::Layout, &mut Self::Dynamic){
        println!("ExternalEntityFilter filters()");
        (self.Layout,self.Dynamic)
    }

}

impl Default for ExternalEntityFilter{
    fn default() -> Self{
        println!("ExternalEntityFilter default()");
        ExternalEntityFilter{}
    }
}