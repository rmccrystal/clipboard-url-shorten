use systray::Application;

// Creates a tray which can manage the application
pub fn create() -> Result<(), systray::Error> {
    let mut app = Application::new()?;

    app.set_icon_from_file("F:\\git\\clipboard-url-shorten\\src\\icon.ico")?;
    app.set_tooltip("Clipboard URL Shortener")?;

    app.add_menu_item("Test", |_| {
        println!("Test");
        Ok::<_, systray::Error>(())
    })?;

    app.wait_for_message()?;
    Ok(())
}