use ggez::audio::{SoundSource, Source};
use ggez::Context;

pub fn play_deal(ctx: &mut Context) {
    let mut source = Source::new(ctx, "/deal.wav").unwrap();
    source.set_volume(0.1);
    source.play_detached(ctx).unwrap();
}
pub fn play_drop(ctx: &mut Context) {
    let mut source = Source::new(ctx, "/drop.wav").unwrap();
    source.set_volume(0.1);
    source.set_pitch(1.7);
    source.play_detached(ctx).unwrap();
}
pub fn play_take(ctx: &mut Context) {
    let mut source = Source::new(ctx, "/take.wav").unwrap();
    source.set_volume(0.1);
    source.set_pitch(1.7);
    source.play_detached(ctx).unwrap();
}
