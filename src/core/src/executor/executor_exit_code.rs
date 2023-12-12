#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExitCode {
  UncaughtException,
  UncaughtExceptionTwice,
  OOM,
  DiskStoreFailedToCreateDir,
  ExternalBlockStoreFailedToInitialize,
  ExternalBlockStoreFailedToCreateDir,
  HeartbeatFailure
}

pub fn explain_exit_code(exit_code: ExitCode) -> String {
  match exit_code {
    ExitCode::UncaughtException =>
      String::from("Uncaught exception"),
    ExitCode::UncaughtExceptionTwice =>
      String::from("Uncaught exception, and logging the exception failed"),
    ExitCode::OOM =>
      String::from("OutOfMemoryError"),
    ExitCode::DiskStoreFailedToCreateDir =>
      String::from("Failed to create local directory"),
    ExitCode::ExternalBlockStoreFailedToInitialize =>
      String::from("ExternalBlockStore failed to initialize"),
    ExitCode::ExternalBlockStoreFailedToCreateDir =>
      String::from("ExternalBlockStore failed to create a local temporary directory"),
    ExitCode::HeartbeatFailure =>
      String::from("Unable to send heartbeats to driver")
  }
}