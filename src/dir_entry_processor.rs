use crate::dir_entry_data::DirEntryData;
use crate::dir_entry_info::DirEntryInfo;

#[async_trait::async_trait]
pub(crate) trait DirEntryProcessor {

    fn name(&self) -> &str;

    async fn process(&self, dir_entry_info: &mut DirEntryInfo) -> Result<Vec<DirEntryData>, std::io::Error>;

}
