extern crate egg_mode;
extern crate tokio_core;
extern crate notify;

// mod tweet;
mod tweet;

use egg_mode::media::media_types::{image_gif};

use tokio_core::reactor::Core;

use egg_mode::media::UploadBuilder;
// use egg_mode::media::media_types::image_gif;
use egg_mode::tweet::DraftTweet;

// https://imgur.com/t/science_and_tech/Nr0ND
static OUTPUT: &'static [u8] = include_bytes!("7cnm2cI.gif");

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    println!("uploading gif...");
    let token = tweet::get_token();

    let builder = UploadBuilder::new(OUTPUT, image_gif());
    let media_handle = core.run(builder.call(&token, &handle)).unwrap();

    println!("sending tweet...");

    let draft = DraftTweet::new("need to make sure i can upload a gif in egg-mode, here's my test gif");
    let draft = draft.media_ids(&[media_handle.id]);

    let tweet = core.run(draft.send(&token, &handle)).unwrap();

    println!("message: {}", tweet.text);
}
