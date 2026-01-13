//go:build windows

package main

/*
#cgo LDFLAGS: -L../lib -lwebp_validator
#include "../include/webp_validator.h"
#include <stdlib.h>
*/
import "C"

type WebpInfo struct {
	IsValid    bool
	Width      uint32
	Height     uint32
	HasAlpha   bool
	IsAnimated bool
	NumFrames  uint32
	Error      string
}

func ValidateWebp(data []byte) WebpInfo {
	if len(data) == 0 {
		return WebpInfo{
			IsValid: false,
			Error:   "data is empty",
		}
	}

	cData := C.CBytes(data)
	defer C.free(cData)

	result := C.validate_webp_ffi((*C.uint8_t)(cData), C.size_t(len(data)))

	info := WebpInfo{
		IsValid:    bool(result.is_valid),
		Width:      uint32(result.width),
		Height:     uint32(result.height),
		HasAlpha:   bool(result.has_alpha),
		IsAnimated: bool(result.is_animated),
		NumFrames:  uint32(result.num_frames),
	}

	if result.error_message != nil {
		info.Error = C.GoString(result.error_message)
		C.free_error_message(result.error_message)
	}

	return info
}
