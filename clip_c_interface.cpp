// Clip Library
// Copyright (c) 2015-2016 David Capello

#include <cstdint>
#include "clip.h"

extern "C" uint8_t clip_empty_format() {
  return static_cast<uint8_t>(clip::empty_format());
}

extern "C" uint8_t clip_text_format() {
  return static_cast<uint8_t>(clip::text_format());
}

extern "C" uint8_t clip_image_format() {
  return static_cast<uint8_t>(clip::image_format());
}

extern "C" bool clip_has(uint8_t format) {
  return clip::has(static_cast<uint8_t>(format));
}

extern "C" void* clip_get_image() {
  clip::image* img = new clip::image;
  if (!clip::get_image(*img)) {
    return nullptr;
  }

  return img;
}

extern "C" clip::image_spec clip_get_image_spec(void* ptr) {
  clip::image* img = static_cast<clip::image*>(ptr);
  return img->spec();
}

extern "C" char* clip_get_image_data(void* ptr) {
  clip::image* img = static_cast<clip::image*>(ptr);
  return img->data();
}

extern "C" void clip_delete_image(void* ptr) {
  clip::image* img = static_cast<clip::image*>(ptr);
  delete img;
}
// const char* data = img.data();
