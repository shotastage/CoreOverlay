pub enum ReplicaType {
    WinterStorage,
    SummerStorage,
    SpringStorage,
}

pub struct FileManager {
    pub replica_type: ReplicaType,
    pub replica_id: u32,
}
