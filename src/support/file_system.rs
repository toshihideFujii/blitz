
#![allow(dead_code)]

const INVALID_FILE: i32 = -1;

// An enumeration for the file system's view of the type.
enum FileType {
  StatusError,
  FileNotFound,
  RegularFile,
  DirectoryFile,
  SymlinkFile,
  BlockFile,
  CharacterFile,
  FifoFile,
  SocketFile,
  TypeUnknown
}