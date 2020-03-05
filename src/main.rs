use urlshortener::{client::UrlShortener, providers::Provider};
use clipboard::{ClipboardContext, ClipboardProvider};
use std::fmt::Error;

// The service which will shorten our URLs
static PROVIDER: Provider = Provider::VGd;

fn main() {
	let us = UrlShortener::new().unwrap();
	clipboard_loop(us).unwrap();
}

// The loop which will modify our clipboard
fn clipboard_loop(us: UrlShortener) -> Result<(), Error> {
	// The last thing the clipboard was set to
	// If this variable is different, the clipboard data will be handled
	let mut last_clipboard = String::new();
	let mut provider = ClipboardProvider::new()?;
	loop {
		let clipboard_text = provider.get_contents()?;
		if clipboard_text != last_clipboard {   // If we have new content in the clipboard
			last_clipboard = clipboard_text;    // Update the last clipboard to the current clipboard
			let new_clipboard = handle_clipboard_change(clipboard, &us)?;
			provider.set_contents(new_clipboard)?
		}
	}
}

// This function is ran whenever the clipboard is changed
// Takes the clipboard text and the url shortener object
// and returns the new clipboard data
fn handle_clipboard_change(clipboard_text: String, us: &UrlShortener) -> Result<String, Error> {
	return Ok(Strings::new("test"))
}