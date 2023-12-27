use gst::prelude::*;
use anyhow::Error;
use gstrswebrtc::webrtcsink;
fn main() -> Result<(), Error>{
    gst::init()?;
    let pipeline = gst::Pipeline::new();
    let video_src: gst::Element = gst::ElementFactory::make("videotestsrc").build().unwrap();
    //let video_sink = webrtcsink::WebRTCSin, signal_name, after, callback));

    //pipeline.add_many([&video_src, &video_sink]).unwrap();

    Ok(())
}
