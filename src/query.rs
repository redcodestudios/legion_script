// use legion::{
//     query::{View, DefaultFilter, QueryResult, IntoIndexableIter, Fetch},
//     storage::{ComponentTypeId, Components, Component, Archetype},
//     world::{ComponentAccess, Permissions}
// };
// use crate::filter::{ExternalEntityFilter};

// struct ExternalView {
//     components: Vec<ComponentTypeId>,
// }

// impl ExternalView {}

// impl DefaultFilter for ExternalView{
//     type Filter = ExternalEntityFilter;
// }

// impl Fetch for ExternalView {

// }

// impl Iter for ExternalView {

// }

// impl <'data>View<'data> for ExternalView {
//     type Element = <Self::Fetch as IntoIndexableIter>::Item;
//     // type Fetch: Fetch + IntoIndexableIter<Item = Self::Element> + 'data;
//     // type Iter: Iterator<Item = Option<Self::Fetch>> + 'data;
//     type Read = [ComponentTypeId; 0];
//     type Write = [ComponentTypeId; 0];

//     unsafe fn fetch(
//         components: &'data Components,
//         archetypes: &'data [Archetype],
//         query: QueryResult<'data>
//     ) -> Self::Iter {}
    
//     fn validate(){}
    
//     fn validate_access(access: &ComponentAccess<'_>) -> bool {
//         true
//     }

//     fn reads_types() -> Self::Read {
//         []
//     }
    
//     fn writes_types() -> Self::Write {
//         [] 
//     }    
    
//     fn reads<T: Component>() -> bool {
//         true
//     }
    
//     fn writes<T: Component>() -> bool {
//         true
//     }
    
//     fn requires_permissions() -> Permissions<ComponentTypeId> {
//         Permissions::default() 
//     }
// }
