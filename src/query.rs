use legion::{
    query::{View, DefaultFilter, QueryResult},
    storage::{ComponentTypeId, Components, Component, Archetype},
    world::{ComponentAccess, Permissions}
};
use crate::filter::{ExternalEntityFilter};

struct ExternalView{}

impl DefaultFilter for ExternalView{
    type Filter = ExternalEntityFilter;
}
impl <'data>View for ExternalView{
    type Element: Send + Sync + 'data;
    type Fetch: Fetch + IntoIndexableIter<Item = Self::Element> + 'data;
    type Iter: Iterator<Item = Option<Self::Fetch>> + 'data;
    type Read: AsRef<[ComponentTypeId]>;
    type Write: AsRef<[ComponentTypeId]>;

    unsafe fn fetch(
        components: &'data Components,
        archetypes: &'data [Archetype],
        query: QueryResult<'data>
    ) -> Self::Iter{}
    fn validate(){}
    fn validate_access(access: &ComponentAccess<'_>) -> bool{}
    fn reads_types() -> Self::Read{}
    fn writes_types() -> Self::Write{}    
    fn reads<T: Component>() -> bool{}
    fn writes<T: Component>() -> bool{}
    fn requires_permissions() -> Permissions<ComponentTypeId>{}
}