const NUM_PMP_ENTRIES: usize = 4;

#[repr(C)]
pub struct ArchTaskDescriptor {
    pub pmp_addr: [u32; NUM_PMP_ENTRIES],
    pub pmp_cfg: u32,
}
