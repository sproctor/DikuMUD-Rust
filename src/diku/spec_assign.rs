use std::collections::HashMap;

use diku::spec_procs::*;
use diku::structs::IndexData;

pub fn assign_mobiles(mob_index: &mut HashMap<u32, IndexData>) {
    mob_index.get_mut(&1).unwrap().func = Some(puff);
    mob_index.get_mut(&3060).unwrap().func = Some(cityguard);
    mob_index.get_mut(&3067).unwrap().func = Some(cityguard);
    //mob_index.get_mut(&3061).unwrap().func = Some(janitor);
}