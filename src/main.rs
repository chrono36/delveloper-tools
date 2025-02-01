mod app;
use std::{collections::BTreeMap, sync::Arc};

use app::App;
use egui::{FontData, FontDefinitions, FontFamily, FontId, TextStyle};

pub const PLAY_WRITE_FAMILY_NAME: &str = "Playwrite AU SA";
pub const HARMONYOS_FAMILY_NAME: &str = "HarmonyOS SANS SC";
const ALIBABAPUHUITI_FAMILY_NAME: &str = "AlibabaPuHuiTi-3-55-Regular";

fn main() -> eframe::Result<()> {
    const ICON: &[u8] = include_bytes!("../assets/logo.png");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Developer Tools")
            .with_taskbar(true)
            .with_icon(eframe::icon_data::from_png_bytes(ICON).unwrap()),
        ..Default::default()
    };

    eframe::run_native(
        "Developer Tools",
        native_options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(App::new(cc)))
        }),
    )
}

#[inline]
fn logo() -> TextStyle {
    TextStyle::Name("logo".into())
}

fn setup_fonts(ctx: &egui::Context) {
    load_fonts(ctx);

    let text_styles: BTreeMap<_, _> = [
        (
            egui::TextStyle::Heading,
            FontId::new(30.0, FontFamily::Proportional),
        ),
        (
            logo(),
            FontId::new(25.0, FontFamily::Name(PLAY_WRITE_FAMILY_NAME.into())),
        ),
        (
            egui::TextStyle::Name("SidebarHeader".into()),
            FontId::new(25.0, FontFamily::Name(PLAY_WRITE_FAMILY_NAME.into())),
        ),
        (
            egui::TextStyle::Name("Heading2".into()),
            FontId::new(25.0, FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Name("Context".into()),
            FontId::new(23.0, FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Body,
            FontId::new(18.0, FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Monospace,
            FontId::new(14.0, FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Button,
            FontId::new(14.0, FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Small,
            FontId::new(10.0, FontFamily::Proportional),
        ),
    ]
    .into();

    ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
}

fn load_fonts(ctx: &egui::Context) {
    let msyh_font = FontData::from_static(include_bytes!(
        "../assets/fonts/HarmonyOS_SansSC_Regular.ttf"
    ));
    let handwriting_font = FontData::from_static(include_bytes!("../assets/fonts/Playwrite.ttf"));

    let puhu_font = FontData::from_static(include_bytes!(
        "../assets/fonts/AlibabaPuHuiTi-3-55-Regular.ttf"
    ));

    let mut fonts = FontDefinitions::default();

    fonts
        .font_data
        .insert(HARMONYOS_FAMILY_NAME.to_owned(), Arc::from(msyh_font));
    fonts.font_data.insert(
        PLAY_WRITE_FAMILY_NAME.to_owned(),
        Arc::from(handwriting_font),
    );

    fonts
        .font_data
        .insert(ALIBABAPUHUITI_FAMILY_NAME.to_owned(), Arc::from(puhu_font));

    fonts.families.insert(
        FontFamily::Proportional,
        vec![HARMONYOS_FAMILY_NAME.to_owned()],
    );

    fonts.families.insert(
        FontFamily::Monospace,
        vec![ALIBABAPUHUITI_FAMILY_NAME.to_owned()],
    );

    fonts.families.insert(
        FontFamily::Name(PLAY_WRITE_FAMILY_NAME.into()),
        vec![PLAY_WRITE_FAMILY_NAME.to_owned()],
    );

    ctx.set_fonts(fonts);
}
