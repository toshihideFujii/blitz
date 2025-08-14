
pub struct BlitxCustomCallStatus {
  message: Option<String>
}

// Set the BlitzCustomCallStatus to a success state. This is the default state.
pub fn blitz_custom_call_status_set_success(
  status: &mut BlitxCustomCallStatus)
{
  status.message = None;
}

// Set the BlitzCustomCallStatus to a failure state with the given error message.
// Does not take ownership of the supplied message string; instead copies the
// first 'message_len' bytes, or up to the null terminator, whichever comes
// first.
pub fn blitz_custom_call_status_set_failure(
  status: &mut BlitxCustomCallStatus, message: &String)
{
  status.message = Some(message.clone());
}

// Get a view of the internal error message of the BlitzCustomCallStatus. Only
// lives as long as the BlitzCustomCallStatus. Returns an empty optional if the
// result was "success".
fn _custom_call_status_get_message(
  status: &BlitxCustomCallStatus) -> Option<String>
{
  status.message.clone()
}