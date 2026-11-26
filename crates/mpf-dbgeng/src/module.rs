/// Port of the lookup loop in `debug_target_get_module_info_by_base`.
///
/// `bases` holds the load base of each loaded module (`modules.info[i].params.Base`
/// for `i < modules.loaded`). Returns the index of the first match.
pub fn module_index_by_base(bases: &[u64], base: u64) -> Option<usize> {
    bases.iter().position(|&b| b == base)
}
