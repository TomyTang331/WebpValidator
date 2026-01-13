#ifndef WEBP_VALIDATOR_H
#define WEBP_VALIDATOR_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C"
{
#endif

    /**
     * WebP validation result
     */
    typedef struct
    {
        bool is_valid;       // Whether file is valid WebP
        uint32_t width;      // Image width
        uint32_t height;     // Image height
        bool has_alpha;      // Whether has alpha channel
        bool is_animated;    // Whether is animated WebP
        uint32_t num_frames; // Number of frames (for animated WebP)
        char *error_message; // Error message (NULL if is_valid is true)
                             // Free using free_error_message()
    } WebpValidationResult;

    /**
     * Validate WebP image file
     *
     * @param data Pointer to WebP file data
     * @param len Length of the data in bytes
     * @return WebpValidationResult
     */
    WebpValidationResult validate_webp_ffi(const uint8_t *data, size_t len);

    /**
     * Free error message memory allocated by validate_webp_ffi
     *
     * @param error_message Pointer from WebpValidationResult.error_message
     */
    void free_error_message(char *error_message);

#ifdef __cplusplus
}
#endif

#endif // WEBP_VALIDATOR_H
