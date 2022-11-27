#![forbid(unsafe_code)]

pub enum Handle<'a> {
    Dir(DirHandle<'a>),
    File(FileHandle<'a>),
    Content {
        file_path: &'a Path,
        content: &'a [u8],
    },
}

pub struct DirHandle<'a> {
    // TODO: your code goes here.
}

impl<'a> DirHandle<'a> {
    pub fn descend(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn path(&self) -> &Path {
        // TODO: your code goes here.
        unimplemented!()
    }
}

pub struct FileHandle<'a> {
    // TODO: your code goes here.
}

impl<'a> FileHandle<'a> {
    pub fn read(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn path(&self) -> &Path {
        // TODO: your code goes here.
        unimplemented!()
    }
}
