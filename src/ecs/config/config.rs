pub type ComponentType = u32;
pub type EntityType = u32;

pub const MAX_ENTITIES: EntityType = 5000;

pub const MAX_COMPONENTS: ComponentType = 32;

pub type Signature = bit_set::BitSet<u32>;
