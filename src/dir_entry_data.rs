use mime::Mime;

#[derive(Debug)]
pub(crate) struct DirEntryData {
    mime_type: Mime,
    name: String,
    byte_data: Option<Vec<u8>>,
    string_data: Option<String>,
}

impl DirEntryData {

    pub(crate) fn new_with_data(mime_type: Mime, name: String, data: Vec<u8>) -> Self {
        DirEntryData {
            mime_type,
            name,
            byte_data: Some(data),
            string_data: None,
        }
    }

    // Constructor to create a new DirEntryData with string_data (String)
    pub(crate) fn new_with_string_data(mime_type: Mime, name: String, string_data: String) -> Self {
        DirEntryData {
            mime_type,
            name,
            byte_data: None,
            string_data: Some(string_data),
        }
    }

    // Public getter for mime_type
    pub(crate) fn mime_type(&self) -> &Mime {
        &self.mime_type
    }

    // Public getter for name
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    // Public getter for data as an Option<&[u8]>
    pub(crate) fn byte_data(&self) -> Option<&[u8]> {
        self.byte_data.as_deref() // Converts Option<Vec<u8>> to Option<&[u8]>
    }

    // Public getter for string_data
    pub(crate) fn string_data(&self) -> Option<&str> {
        self.string_data.as_deref() // Converts Option<String> to Option<&str>
    }

    pub(crate) fn is_string(&self) -> bool {
        self.byte_data.is_some()
    }
}
