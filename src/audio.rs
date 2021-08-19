use ggez::audio::{SoundSource, Source};
use ggez::Context;

pub struct Audio {
    pub deal: Source,
    pub drop: Source,
    pub take: Source,
}

impl Audio {
    pub fn new(ctx: &mut Context) -> Self {
        let mut deal = Source::new(ctx, "/deal.wav").unwrap();
        deal.set_volume(0.1);
        let mut drop = Source::new(ctx, "/drop.wav").unwrap();
        drop.set_volume(0.1);
        let mut take = Source::new(ctx, "/take.wav").unwrap();
        take.set_volume(0.1);
        Self { deal, drop, take }
    }
}
