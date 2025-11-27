# Plan: Processing Effects on Generated Channels

## Objective
Enable the application of image effects (e.g., Halftone, Dither, Threshold) specifically to the individual CMYK channels generated from the source image, rather than applying them to the composite image before separation.

## Current Architecture Analysis
- **`src-tauri/src/imaging/processes.rs`**: Contains the `ImageProcessor` struct and `process_image` function.
- **Current Flow**:
  1. `ImageProcessor::new(img)`
  2. `.apply_filter(filter)` (Applies to composite `self.image`)
  3. `.apply_effect(effect)` (Applies to composite `self.image`)
  4. `.save()` (Iterates `self.processed_images`, but `separate_channels` is currently missing in the `process_image` chain in the provided code, or implicitly expected).
- **Issue**: Effects are currently applied to the RGB image *before* it is split into CMYK channels. This results in the CMYK separation of an already-effected image, rather than the effect being applied to the ink channels themselves (which is standard for things like Halftoning).

## Implementation Steps

### 1. Modify `ImageProcessor` in `processes.rs`
We need to extend the `ImageProcessor` to handle post-separation processing.

- **Add `apply_effect_to_channels` method**:
  - This method will take the `ImageEffect` as an argument.
  - It will iterate over `self.processed_images` (the separated channels).
  - It will apply the effect to each channel individually using the existing `get_effect` logic.
  - It replaces the old channel image with the new processed version.

### 2. Update `process_image` Workflow
Refactor the `process_image` function in `src-tauri/src/imaging/processes.rs` to follow the correct order of operations:

1. **Load Image**: Create `ImageProcessor`.
2. **Pre-processing**: Apply global filters (like Contrast/Brightness) to the composite image (`apply_filter`).
3. **Separation**: Split the image into CMYK channels (`separate_channels`).
4. **Channel Processing**: Apply the selected effect to each generated channel (`apply_effect_to_channels`).
5. **Save**: Save the resulting channels.

**Proposed Code Structure:**
```rust
ImageProcessor::new(img)
    .apply_filter(filter)      // Global adjustments
    .separate_channels()?      // Split to CMYK
    .apply_effect_to_channels(effect) // Apply Halftone/Dither to specific channels
    .save(&filename)
```

### 3. Verify `commands.rs`
Ensure the `process_selected_image` command correctly propagates the `ProcessSettings` to the updated `process_image` function.
- The current implementation of `process_selected_image` appears correct, passing `state.process_settings` down.

### 4. Testing
- **Input**: Load an image.
- **Action**: Select "Halftone" effect.
- **Expected Output**: 4 images (Cyan, Magenta, Yellow, Black), each looking like a grayscale halftone representation of that specific channel.
- **Current (Wrong) Output**: 4 images that look like CMYK separations of a Halftoned RGB image (or potentially empty/failed if `separate_channels` was missing).

## Future Considerations
- **Per-Channel Settings**: Eventually, we might want to allow different effects or parameters (e.g., different halftone angles) for different channels.
- **Preview**: Generating a composite preview of the processed channels to show the user what the final print might look like.
