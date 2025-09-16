use ocrail;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let search_text = "MALDEV ACADEMY";
    println!("[*] Checking if '{}' is in the wallpaper...", search_text);

    if ocrail::get_text_in_wallpaper()?.contains(search_text) {
        println!("[+] Text '{}' found in wallpaper.", search_text);
    } else {
        println!("[-] Text '{}' not found in wallpaper.", search_text);
    }

    Ok(())
}
