use crate::{localizer::Localizer, Calc};

impl Calc {
  pub fn add_scales(&self, localizer: &Localizer) {
    let mut scale_manager = self.scale_manager.borrow_mut();
    scale_manager.add_scale("k", 3, &localizer.get_strings("thousand.variants"));
    scale_manager.add_scale("M", 6, &localizer.get_strings("million.variants"));
    scale_manager.add_scale("b", 9, &localizer.get_strings("billion.variants"));
  }
}
