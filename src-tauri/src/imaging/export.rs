use crate::errors::Error;
use crate::imaging::processes::ProcessResult;
use ::image::open;
use printpdf::*;
use std::fs;
use std::path::Path;

pub fn save_channels_to_disk(
    channels: &[ProcessResult],
    export_path: &str,
    base_filename: &str,
    colors: Option<&Vec<String>>,
) -> Result<(), Error> {
    let export_dir = Path::new(export_path);
    if !export_dir.exists() {
        fs::create_dir_all(export_dir)?;
    }

    for (i, channel) in channels.iter().enumerate() {
        let img = open(&channel.image_path).map_err(|e| Error::Processing(e.to_string()))?;

        // Get the color for this channel if available
        let color_suffix = colors
            .and_then(|c| c.get(i))
            .map(|hex| format!("_{}", hex.trim_start_matches('#')))
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
    colors: Option<&Vec<String>>,
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

    for (i, channel) in channels.iter().enumerate() {
        // Load the image
        let img_bytes =
            fs::read(&channel.image_path).map_err(|e| Error::Processing(e.to_string()))?;

        let raw_image = RawImage::decode_from_bytes(&img_bytes, &mut warnings)
            .map_err(|e| Error::Processing(e))?;

        // Add image to document resources and get its ID
        let image_id = doc.add_image(&raw_image);

        // Calculate scaling to fit the image on the page with margins
        let available_width = page_width.0 - 2.0 * margin.0;
        let available_height = page_height.0 - 2.0 * margin.0;

        // Convert image dimensions to mm (assuming 72 DPI for calculation)
        let dpi = 72.0;
        let img_width_mm = (raw_image.width as f32 / dpi) * 25.4;
        let img_height_mm = (raw_image.height as f32 / dpi) * 25.4;

        // Calculate scale to fit within available space
        let scale_x = available_width / img_width_mm;
        let scale_y = available_height / img_height_mm;
        let scale = scale_x.min(scale_y).min(1.0); // Don't upscale

        let final_width = img_width_mm * scale;
        let final_height = img_height_mm * scale;

        // Center the image on the page
        let x_offset = margin.0 + (available_width - final_width) / 2.0;
        let y_offset = margin.0 + (available_height - final_height) / 2.0;

        // Create page operations
        let ops = vec![
            // Add the image using XObject
            Op::UseXobject {
                id: image_id.clone(),
                transform: XObjectTransform {
                    translate_x: Some(Pt(x_offset * 2.834645669)), // Convert mm to pt
                    translate_y: Some(Pt(y_offset * 2.834645669)),
                    scale_x: Some(scale),
                    scale_y: Some(scale),
                    ..Default::default()
                },
            },
            // Add channel label
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
                items: vec![TextItem::Text({
                    let color_info = colors
                        .and_then(|c| c.get(i))
                        .map(|hex| format!(" ({})", hex))
                        .unwrap_or_default();
                    format!("Channel: {}{}", channel.channel, color_info)
                })],
                font: BuiltinFont::Helvetica,
            },
            Op::EndTextSection,
        ];

        // Create the page
        let page = PdfPage::new(page_width, page_height, ops);
        pages.push(page);
    }

    // Add all pages to the document
    doc.with_pages(pages);

    // Save the PDF
    let pdf_bytes = doc.save(&PdfSaveOptions::default(), &mut warnings);
    fs::write(&pdf_path, pdf_bytes).map_err(|e| Error::Processing(e.to_string()))?;

    Ok(())
}
