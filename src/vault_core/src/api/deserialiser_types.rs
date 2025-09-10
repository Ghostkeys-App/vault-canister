// Fixed-size header for cell data. size field is used to then extract the cell data.
// x and y are the coordinates of the cell in 2D space and are used as part of the 
// key in stable storage so they can be reported to the client on retrieval.
pub struct CellHeader {
    pub size : u16,
    pub x : u8,
    pub y : u8
}
impl CellHeader {
    pub fn new(header : Vec<u8>) -> Self {
        let size = u16::from_be_bytes([header[0], header[1]]);
        let x = u8::from_be_bytes([header[2]]);
        let y = u8::from_be_bytes([header[3]]);
        Self { size, x, y }
    }
}

// Describes a single cell in 2D space, used for logins and spreadsheets.
pub struct Cell {
    pub header : CellHeader,
    pub data : Vec<u8>
}
impl Cell {
    pub fn new(header: CellHeader, data : Vec<u8>) -> Self {
        Self { header, data }
    }
}

// Describes logins or a spreadsheet as 2D space transformed to a 1D array of cells.
pub struct Cells {
    pub cells : Vec<Cell>
}
impl Cells {
    pub fn new(cells : Vec<u8>) -> Self {
        let mut index = 0;
        let mut result = Vec::new();
        while index < cells.len() {
            let header = CellHeader::new(cells[index..index+4].to_vec());

            let size = header.size as usize;
            index += size_of::<CellHeader>();

            if size == 0 {
                result.push(Cell::new(header, vec![]));
                continue;
            }
            
            let cell = Cell::new(header, cells[index..index + size].to_vec());
            result.push(cell);

            index += size;
        }
        Self { cells : result }
    }
}

// Stripped-down version of Cell for deletions, only contains coordinates.
pub struct DeleteCell {
    pub x : u8,
    pub y : u8
}
impl DeleteCell {
    pub fn new(data : Vec<u8>) -> Self {
        let x = u8::from_be_bytes([data[0]]);
        let y = u8::from_be_bytes([data[1]]);
        Self { x, y }
    }
}

// Describes cells for deletion.
pub struct DeleteCells {
    pub cells : Vec<DeleteCell>
}
impl DeleteCells {
    pub fn new(cells : Vec<u8>) -> Self {
        let mut index = 0;
        let mut result = Vec::new();

        while index < cells.len() {
            let cell = DeleteCell::new(cells[index..index+size_of::<DeleteCell>()].to_vec());
            result.push(cell);
            index += size_of::<DeleteCell>();
        }
        Self { cells : result }
    }
}


// Identifies the "name" of a group of identities, usually the website or app the login is for.
// Thus this only needs to be unique on the x axis.
#[repr(align(1))]
pub struct LoginMetadataHeader {
    pub size : u16,
    pub x : u8,
}
impl LoginMetadataHeader {
    pub fn new(header : Vec<u8>) -> Self {
        let size = u16::from_be_bytes([header[0], header[1]]);
        let x = u8::from_be_bytes([header[2]]);
        Self { size, x }
    }
}

// Describes the metadata for a login column.
pub struct LoginMetadataEntry {
    pub header : LoginMetadataHeader,
    pub data : Vec<u8>
}
impl LoginMetadataEntry {
    pub fn new(header: LoginMetadataHeader, data : Vec<u8>) -> Self {
        Self { header, data }
    }
}

pub struct LoginMetadata {
    pub metadatas : Vec<LoginMetadataEntry>
}
impl LoginMetadata {
    pub fn new(metadatas : Vec<u8>) -> Self {
        let mut index = 0;
        let mut result = Vec::new();
        while index < metadatas.len() {
            let header = LoginMetadataHeader::new(metadatas[index..index+3].to_vec());
            let size = header.size as usize;
            index += 3;
            let metadata = LoginMetadataEntry::new(header, metadatas[index..index + size].to_vec());
            result.push(metadata);
            index += size;
        }
        Self { metadatas : result }
    }
}

// Describes the full login data, including metadata and entries.
pub struct LoginData {
    pub metadata : LoginMetadata,
    pub cells : Cells
}
impl LoginData {
    pub fn new(logindata : Vec<u8>) -> Self {
        // First 5 bytes is metadata size
        let metadata_size = u64::from_be_bytes([0, 0, 0, logindata[0], logindata[1], logindata[2], logindata[3], logindata[4]]) as usize;
        let metadata = LoginMetadata::new(logindata[4..4 + metadata_size].to_vec());
        let cells = Cells::new(logindata[4 + metadata_size..].to_vec());
        Self { metadata, cells }
    }
}

// Stripped-down version of LoginMetadataEntry for deletions, only contains x coordinate
// of targeted column
pub struct DeleteMetadataEntry {
    pub x : u8
}
impl DeleteMetadataEntry {
    pub fn new(x : u8) -> Self {
        Self { x }
    }
}
pub struct DeleteMetadatas {
    pub metadatas : Vec<DeleteMetadataEntry>
}
impl DeleteMetadatas {
    pub fn new(metadatas : Vec<u8>) -> Self {
        let mut result = Vec::new();

        for x in metadatas.iter() {
            let metadata = DeleteMetadataEntry::new(*x);
            result.push(metadata);
        }

        Self { metadatas : result }
    }
}

// Describes a global sync, containing complete login data and spreadsheet data.
pub struct GlobalSyncData {
    pub logins : LoginData,
    pub spreadsheet : Cells,
}
impl GlobalSyncData {
    pub fn new(data : Vec<u8>) -> Self {
        let spreadsheet_size = u64::from_be_bytes([0, 0, 0, data[0], data[1], data[2], data[3], data[5]]) as usize;
        let logins_size = data.len() - 5 - spreadsheet_size;
        let spreadsheet = Cells::new(data[5..5 + spreadsheet_size].to_vec());
        let logins = LoginData::new(data[5 + spreadsheet_size..5 + spreadsheet_size + logins_size].to_vec());
        Self { logins, spreadsheet }
    }
}