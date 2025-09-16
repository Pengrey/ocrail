use std::{error::Error, path::PathBuf};
use windows::{
    core::HSTRING,
    Graphics::Imaging::BitmapDecoder,
    Media::Ocr::OcrEngine,
    Storage::{FileAccessMode, StorageFile},
};

pub fn get_text_in_wallpaper() -> Result<String, Box<dyn Error>> {
    let appdata = std::env::var("APPDATA")?;
    let wallpaper_path: PathBuf = PathBuf::from(appdata).join("Microsoft\\Windows\\Themes\\TranscodedWallpaper");
    let path_hstring = HSTRING::from(wallpaper_path.as_os_str());

    let file = StorageFile::GetFileFromPathAsync(&path_hstring)?.get()?;
    let stream = file.OpenAsync(FileAccessMode::Read)?.get()?;
    let decoder = BitmapDecoder::CreateAsync(&stream)?.get()?;
    let bitmap = decoder.GetSoftwareBitmapAsync()?.get()?;

    // We use installed languages for better comp
    let engine = OcrEngine::TryCreateFromUserProfileLanguages()?;

    let result = engine.RecognizeAsync(&bitmap)?.get()?;

    Ok(result.Text()?.to_string())
}
