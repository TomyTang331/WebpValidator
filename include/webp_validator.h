#ifndef WEBP_VALIDATOR_H
#define WEBP_VALIDATOR_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C"
{
#endif

    /**
     * WebP校验结果
     */
    typedef struct
    {
        bool is_valid;       // 是否为合法的WebP文件
        uint32_t width;      // 图片宽度
        uint32_t height;     // 图片高度
        bool has_alpha;      // 是否有透明通道
        bool is_animated;    // 是否为动态WebP
        uint32_t num_frames; // 帧数 (动态WebP)
        char *error_message; // 错误信息 (is_valid为true时为NULL)
                             // 使用free_error_message()释放内存
    } WebpValidationResult;

    /**
     * 校验WebP图片文件
     *
     * @param path WebP文件路径 (以null结尾的C字符串)
     * @return WebpValidationResult
     *
     */
    WebpValidationResult validate_webp_ffi(const char *path);

    /**
     * 释放validate_webp_ffi分配的错误消息内存
     *
     * @param error_message WebpValidationResult的*error_message
     *
     */
    void free_error_message(char *error_message);

#ifdef __cplusplus
}
#endif

#endif // WEBP_VALIDATOR_H
