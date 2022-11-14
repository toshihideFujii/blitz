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

enum Perms {
  NoPerms,
  OwnerRead,
  OwnerWrite,
  OwnerExe,
  OwnerAll,
  GroupRead,
  GroupWrite,
  GroupExe,
  GroupAll,
  OthersRead,
  OthersWrite,
  OthersExe,
  OthersAll,
  AllRead,
  AllWrite,
  AllExe,
  AllAll,
  SetUidOnExe,
  SetGidOnExe,
  StickeyBit,
  AllPerms,
  PermsNotKnown
}

struct BasicFileStatus {}

impl BasicFileStatus {
  pub fn type_() {}

  pub fn permissions() {}

  pub fn get_last_accessed_time() {}

  pub fn get_last_modification_time() {}
}

struct FileStatus {}

impl FileStatus {
  pub fn get_unique_id() {}

  pub fn get_link_count() {}
}

pub fn make_absolute() {}

pub fn create_directories() {}

pub fn create_directory() {}

pub fn create_link() {}

pub fn create_hard_link() {}

pub fn real_path() {}

pub fn expand_tilde() {}

pub fn current_path() {}

pub fn set_current_path() {}

pub fn remove() {}

pub fn remove_directories() {}

pub fn rename() {}

pub fn copy_file() {}

pub fn resize_file() {}

pub fn resize_file_before_mapping_readwrite() {}

pub fn md5_contents() {}

pub fn exists() {}

enum AccessMode {
  Exist,
  Write,
  Execute
}

pub fn access() {}

pub fn can_execute() {}

pub fn can_write() {}

pub fn equivalent() {}

pub fn is_local() {}

pub fn get_file_type() {}

pub fn is_directory() {}

pub fn is_regular_file() {}

pub fn is_symlink_file() {}

pub fn is_other() {}

pub fn status() {}

pub fn get_umask() {}

pub fn set_permissions() {}

pub fn get_permissions() {}

pub fn file_size() {}

pub fn set_last_access_and_modification_time() {}

pub fn status_known() {}

enum CreationDisposition {
  CreateAlways,
  CreateNew,
  OpenExisting,
  OpenAlways
}

enum FileAccess {
  Read,
  Write
}

enum OpenFlags {
  None,
  Text,
  CRLF,
  TextWithCRLF,
  Append,
  Delete,
  ChildInherit,
  UpdateAtime
}

pub fn create_unique_path() {}

pub fn careate_unique_file() {}

struct TempFile {}

impl TempFile {
  pub fn keep() {}

  pub fn discard() {}
}

pub fn create_temporary_file() {}

pub fn create_unique_directory() {}

pub fn get_potentially_unique_file_name() {}

pub fn get_potentially_unique_temp_file_name() {}

pub fn open_file() {}

pub fn open_native_file() {}

pub fn get_stdin_handle() {}

pub fn get_stdout_handle() {}

pub fn get_stderr_handle() {}

pub fn read_naative_file() {}

pub fn read_naative_file_to_eof() {}

pub fn read_naative_file_slice() {}

pub fn open_file_for_write() {}

pub fn open_native_file_for_write() {}

pub fn open_file_for_read_write() {}

pub fn open_native_file_for_read_write() {}

pub fn open_file_for_read() {}

pub fn open_native_file_for_read() {}

pub fn try_lock_file() {}

pub fn lock_file() {}

pub fn unlock_file() {}

pub fn close_file() {}

struct FileLocker {}

pub fn get_unique_id() {}

pub fn disk_space() {}

struct MappedFileRegion {}

impl MappedFileRegion {
}

pub fn get_main_executable() {}

struct DirectoryEntry {}