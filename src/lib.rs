use image_webp::WebPDecoder;
use std::ffi::CString;
use std::io::Cursor;
use std::os::raw::c_char;

/// WebP image information
#[derive(Debug)]
pub struct WebpInfo {
    pub width: u32,
    pub height: u32,
    pub has_alpha: bool,
    pub is_animated: bool,
    pub num_frames: u32,
}

impl WebpInfo {
    fn new_valid(decoder: &WebPDecoder<Cursor<&[u8]>>) -> Self {
        WebpInfo {
            width: decoder.dimensions().0,
            height: decoder.dimensions().1,
            has_alpha: decoder.has_alpha(),
            is_animated: decoder.is_animated(),
            num_frames: decoder.num_frames(),
        }
    }
}

/// Validate WebP image format
pub fn validate_webp(data: &[u8]) -> Result<WebpInfo, String> {
    let reader = Cursor::new(data);

    match WebPDecoder::new(reader) {
        Ok(decoder) => Ok(WebpInfo::new_valid(&decoder)),
        Err(e) => Err(format!("webp format validation failed: {:?}", e)),
    }
}

/// C-compatible WebP validation result
#[repr(C)]
pub struct WebpValidationResult {
    pub is_valid: bool,
    pub width: u32,
    pub height: u32,
    pub has_alpha: bool,
    pub is_animated: bool,
    pub num_frames: u32,
    pub error_message: *mut c_char,
}

/// Validate WebP file via FFI
///
/// # Safety
/// Caller must ensure:
/// 1. `data` is a valid pointer to a byte array of length `len`
/// 2. `error_message` is freed using `free_error_message`
#[no_mangle]
pub unsafe extern "C" fn validate_webp_ffi(data: *const u8, len: usize) -> WebpValidationResult {
    if data.is_null() {
        return WebpValidationResult {
            is_valid: false,
            width: 0,
            height: 0,
            has_alpha: false,
            is_animated: false,
            num_frames: 0,
            error_message: CString::new("data pointer is null").unwrap().into_raw(),
        };
    }

    let slice = unsafe { std::slice::from_raw_parts(data, len) };

    match validate_webp(slice) {
        Ok(info) => WebpValidationResult {
            is_valid: true,
            width: info.width,
            height: info.height,
            has_alpha: info.has_alpha,
            is_animated: info.is_animated,
            num_frames: info.num_frames,
            error_message: std::ptr::null_mut(),
        },
        Err(err) => WebpValidationResult {
            is_valid: false,
            width: 0,
            height: 0,
            has_alpha: false,
            is_animated: false,
            num_frames: 0,
            error_message: CString::new(err).unwrap().into_raw(),
        },
    }
}

/// Free error message memory allocated by validate_webp_ffi
///
/// # Safety
/// Caller must ensure:
/// 1. `error_message` was returned by `validate_webp_ffi`
/// 2. This function is called only once per pointer
#[no_mangle]
pub unsafe extern "C" fn free_error_message(error_message: *mut c_char) {
    if !error_message.is_null() {
        unsafe {
            let _ = CString::from_raw(error_message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_validate_dynamic_webp() {
        let data = fs::read("images/dynamic.webp").expect("failed to read file");
        let result = validate_webp(&data);

        assert!(result.is_ok(), "dynamic webp should pass validation");

        let info = result.unwrap();
        assert!(info.is_animated, "should be identified as animated");
        assert!(
            info.num_frames > 1,
            "animated image should have multiple frames, actual: {}",
            info.num_frames
        );
        assert!(
            info.width > 0 && info.height > 0,
            "should have valid dimensions"
        );

        println!("dynamic webp test passed:");
        println!("  dimensions: {}x{}", info.width, info.height);
        println!("  has alpha: {}", info.has_alpha);
        println!("  frames: {}", info.num_frames);
    }

    #[test]
    fn test_validate_static_webp() {
        let data = fs::read("images/static.webp").expect("failed to read file");
        let result = validate_webp(&data);

        assert!(result.is_ok(), "static webp should pass validation");

        let info = result.unwrap();
        assert!(!info.is_animated, "should be identified as static");
        assert_eq!(info.num_frames, 0, "static image should have 0 frames");
        assert!(
            info.width > 0 && info.height > 0,
            "should have valid dimensions"
        );

        println!("static webp test passed:");
        println!("  dimensions: {}x{}", info.width, info.height);
        println!("  has alpha: {}", info.has_alpha);
    }

    #[test]
    fn test_validate_fake_webp() {
        let data = fs::read("images/fake.webp").expect("failed to read file");
        let result = validate_webp(&data);

        assert!(result.is_err(), "fake webp should fail validation");

        let error = result.unwrap_err();
        assert!(
            error.contains("webp format validation failed"),
            "error should contain 'webp format validation failed'"
        );
        assert!(
            error.contains("ChunkHeaderInvalid"),
            "error should contain 'ChunkHeaderInvalid', actual: {}",
            error
        );

        println!("fake webp test passed:");
        println!("  error message: {}", error);
    }

    #[test]
    fn test_webp_info_debug() {
        let data = fs::read("images/static.webp").expect("failed to read file");
        let result = validate_webp(&data);
        assert!(result.is_ok());

        let info = result.unwrap();
        let debug_str = format!("{:?}", info);

        assert!(
            debug_str.contains("WebpInfo"),
            "debug output should contain struct name"
        );
        println!("debug formatting test passed:");
        println!("  {:?}", info);
    }
}
