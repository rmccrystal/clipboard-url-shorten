use urlshortener::{client::UrlShortener, providers::Provider};
use clipboard::{ClipboardProvider, ClipboardContext};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use url::Url;
use urlshortener::providers::ProviderError;

fn main() {
	let bitly_token = String::from("4107e7adbc03e75d06ec7440190896b1979c9ad0");
	let us = UrlShortener::new().unwrap();
	clipboard_loop(us, Provider::BitLy{token: bitly_token}).unwrap();
}

// The loop which will modify our clipboard
fn clipboard_loop(us: UrlShortener, provider: Provider) -> Result<(), Box<dyn Error>> {
	// The last thing the clipboard was set to
	// If this variable is different, the clipboard data will be handled
	let mut last_clipboard = String::new();
	let mut clipboard_context: ClipboardContext = ClipboardProvider::new().unwrap();
	loop {
		let clipboard = clipboard_context.get_contents()?;		// Get the new clipboard text
		if clipboard != last_clipboard {  						// If we have new content in the clipboard
			let new_clipboard_result = handle_clipboard_change(&clipboard, &us, &provider);
			match new_clipboard_result {		// Handle the clipboard result
				Err(err) => {
					last_clipboard = clipboard;		// We got an error setting the clipboard, so don't try this clipboard contents again.
					println!("{}", err)
				},
				Ok(Some(new_clipboard)) => {		// If we don't get an error and there is new clipboard text
					println!("updated clipboard to {}", new_clipboard);
					last_clipboard = new_clipboard.clone();    				// Update the last clipboard to the current clipboard
					clipboard_context.set_contents(new_clipboard)?;
				},
				Ok(None) => {}		// Don't do anything if we get None
			}
		}
		sleep(Duration::from_millis(100));
	}
}

/*
 * This function is ran whenever the clipboard is changed
 * Takes the clipboard text, the UrlShortener struct, and a provider
 * and returns the new clipboard data
 *
 * This function will return None if the clipboard should not
 * be changed and an error if there is an error
*/
fn handle_clipboard_change(clipboard_text: &String, us: &UrlShortener, provider: &Provider) -> Result<Option<String>, Box<dyn Error>> {
	if Url::parse(clipboard_text).is_err() {	// If we don't have a url, return nothing
		return Ok(None)
	}

	// Shorten the URL
	let url_result = us.generate(clipboard_text, provider);
	return match url_result {
		Ok(url) => {
			if !url.contains("https://") && !url.contains("http://") {		// If our response is not a link, our response is an error
				return Err(url.into())
			}
			Ok(Some(url.replace("\n", "").replace(" ", "")))	// Return the new url with no newlines or spaces
		},
		Err(ProviderError::Connection) => {
			Err(String::from("Error connecting to the URL shortening service.").into())
		},
		Err(ProviderError::Deserialize)=> {
			Err(String::from("Error deserializing the response from the URL shortening service.").into())
		}
	}
}