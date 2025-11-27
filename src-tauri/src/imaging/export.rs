use crate::errors::Error;
use crate::imaging::processes::ProcessResult;
use crate::state::ColorInfo;
use ::image::open;
use printpdf::*;
use std::fs;
use std::path::Path;

pub fn save_channels_to_disk(
    channels: &[ProcessResult],
    export_path: &str,
    base_filename: &str,
    colors: Option<&Vec<ColorInfo>>,
) -> Result<(), Error> {
    let export_dir = Path::new(export_path);
    if !export_dir.exists() {
        fs::create_dir_all(export_dir)?;
    }

    for (i, channel) in channels.iter().enumerate() {
        let img = open(&channel.image_path).map_err(|e| Error::Processing(e.to_string()))?;

        // Get the color name for this channel if available
        let color_suffix = colors
            .and_then(|c| c.get(i))
            .map(|color_info| format!("_{}", color_info.name.replace(" ", "_")))
            .unwrap_or_default();

        let filename = format!("{}_{}{}.png", base_filename, channel.channel, color_suffix);
        let save_path = export_dir.join(filename);
        img.save(&save_path)
            .map_err(|e| Error::Processing(e.to_string()))?;
    }

    Ok(())
}

pub fn save_channels_to_pdf(
    channels: &[ProcessResult],
    export_path: &str,
    base_filename: &str,
    colors: Option<&Vec<ColorInfo>>,
) -> Result<(), Error> {
    let export_dir = Path::new(export_path);
    if !export_dir.exists() {
        fs::create_dir_all(export_dir)?;
    }

    let pdf_filename = format!("{}.pdf", base_filename);
    let pdf_path = export_dir.join(pdf_filename);

    // Page dimensions (A4)
    let page_width = Mm(210.0);
    let page_height = Mm(297.0);
    let margin = Mm(20.0);

    // Create a new PDF document
    let mut doc = PdfDocument::new(base_filename);
    let mut pages = Vec::new();
    let mut warnings = Vec::new();

    // Helper function to add an image page
    let add_image_page = |doc: &mut PdfDocument,
                          raw_image: &RawImage,
                          label_text: String,
                          pages: &mut Vec<PdfPage>| {
        let image_id = doc.add_image(raw_image);

        let available_width = page_width.0 - 2.0 * margin.0;
        let available_height = page_height.0 - 2.0 * margin.0 - 15.0;

        let img_width_px = raw_image.width as f32;
        let img_height_px = raw_image.height as f32;

        let dpi_for_width = img_width_px / available_width * 25.4;
        let dpi_for_height = img_height_px / available_height * 25.4;
        let target_dpi = dpi_for_width.max(dpi_for_height).max(72.0);

        let final_width_mm = img_width_px / target_dpi * 25.4;
        let final_height_mm = img_height_px / target_dpi * 25.4;

        let x_offset = margin.0 + (available_width - final_width_mm) / 2.0;
        let y_offset = margin.0 + (available_height - final_height_mm) / 2.0;

        let ops = vec![
            Op::UseXobject {
                id: image_id,
                transform: XObjectTransform {
                    translate_x: Some(Pt(x_offset * 2.834645669)),
                    translate_y: Some(Pt(y_offset * 2.834645669)),
                    dpi: Some(target_dpi),
                    ..Default::default()
                },
            },
            Op::StartTextSection,
            Op::SetTextCursor {
                pos: Point {
                    x: Pt(margin.0 * 2.834645669),
                    y: Pt((page_height.0 - margin.0 + 5.0) * 2.834645669),
                },
            },
            Op::SetFontSizeBuiltinFont {
                size: Pt(14.0),
                font: BuiltinFont::Helvetica,
            },
            Op::WriteTextBuiltinFont {
                items: vec![TextItem::Text(label_text)],
                font: BuiltinFont::Helvetica,
            },
            Op::EndTextSection,
        ];

        pages.push(PdfPage::new(page_width, page_height, ops));
    };

    // Add pages for each channel
    for (i, channel) in channels.iter().enumerate() {
        let img_bytes =
            fs::read(&channel.image_path).map_err(|e| Error::Processing(e.to_string()))?;

        let raw_image = RawImage::decode_from_bytes(&img_bytes, &mut warnings)
            .map_err(|e| Error::Processing(e))?;

        // Get color name for this channel (use the RISO color name from UI)
        let color_info = colors
            .and_then(|c| c.get(i))
            .map(|c| format!(" - {}", c.name))
            .unwrap_or_default();
        let label_text = format!("Channel: {}{}", channel.channel, color_info);

        add_image_page(&mut doc, &raw_image, label_text, &mut pages);
    }

    // Add all pages to the document
    doc.with_pages(pages);

    // Save the PDF
    let pdf_bytes = doc.save(&PdfSaveOptions::default(), &mut warnings);
    fs::write(&pdf_path, pdf_bytes).map_err(|e| Error::Processing(e.to_string()))?;

    // Log any warnings
    for warning in &warnings {
        log::warn!("PDF generation warning: {:?}", warning);
    }

    Ok(())
}
