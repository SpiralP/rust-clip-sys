#include <string>

#include "../clip/clip.h"

using namespace std;

const char* clip_get_text() {
  string text;
  if (!clip::get_text(text))
    return nullptr;

  size_t length = text.length() + 1;

  char* c_str = new char[length];
  strcpy_s(c_str, length, text.c_str());

  return c_str;
}

bool clip_set_text(const char* text) {
  return clip::set_text(text);
}

void clip_delete_text(const char* text) {
  delete[] text;
}

bool clip_set_image(void* ptr) {
  clip::image* img = static_cast<clip::image*>(ptr);
  return clip::set_image(*img);
}

void* clip_get_image() {
  clip::image* img = new clip::image;
  if (!clip::get_image(*img)) {
    delete img;
    return nullptr;
  }

  return img;
}

void* clip_create_image_from_data_spec(const void* data,
                                       clip::image_spec spec) {
  clip::image* img = new clip::image(data, spec);
  return img;
}

void* clip_create_image_from_spec(clip::image_spec spec) {
  clip::image* img = new clip::image(spec);
  return img;
}

void clip_delete_image(void* ptr) {
  clip::image* img = static_cast<clip::image*>(ptr);
  delete img;
}

clip::image_spec clip_get_image_spec(void* ptr) {
  clip::image* img = static_cast<clip::image*>(ptr);
  return img->spec();
}

char* clip_get_image_data(void* ptr) {
  clip::image* img = static_cast<clip::image*>(ptr);
  return img->data();
}
