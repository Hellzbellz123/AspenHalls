#[cfg(target_os = "windows")]
pub mod windowsonly {
    use windows::Graphics::{DisplayId, Capture::GraphicsCaptureItem};
    use windows::Graphics::Capture::GraphicsCaptureSession;

    pub fn init_game_bar() {
        if let Ok(support) = GraphicsCaptureSession::IsSupported(){
            print!("Is Game Capture Supported? {:?} \n", support);
            start_capture();
        } else {
            print!("screen capture is not supported for some reason")

        }
    }
    fn start_capture()
    {
        // The GraphicsCapturePicker follows the same pattern the
        // file pickers do.
        if let Ok(picker) = windows::Graphics::Capture::GraphicsCapturePicker::new(){
            if let Ok(itempick) = windows::Graphics::Capture::GraphicsCapturePicker::PickSingleItemAsync(&picker){
                print!("start pick single item{:?}", itempick)
            }
        }

        if let Ok(item) = windows::Graphics::Capture::GraphicsCaptureItem::TryCreateFromDisplayId(DisplayId::default()){
        }

        // GraphicsCaptureItem item = await picker.PickSingleItemAsync();

        // The item may be null if the user dismissed the
        // control without making a selection or hit Cancel.
        // if (item != null)
        // {
            // We'll define this method later in the document.
            // StartCaptureInternal(item);
        // }
    }
}

