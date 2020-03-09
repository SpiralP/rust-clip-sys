#define _CRT_SECURE_NO_WARNINGS

#include <cstdint>
#include <cstring>
#include <string>
#include <vector>

#include "../clip/clip.h"

using namespace std;
using namespace clip;

#define EXPORT extern "C"

struct FFICharPtr {
  size_t length;
  char* ptr;
};

EXPORT void FFICharPtr_delete(FFICharPtr char_ptr) {
  delete[] char_ptr.ptr;
}

EXPORT FFICharPtr clip_get_text() {
  string text;
  if (!clip::get_text(text)) {
    return {0, nullptr};
  }

  size_t length = text.length();

  char* c_str = new char[length + 1];
  strcpy(c_str, text.c_str());

  return {length, c_str};
}

EXPORT bool clip_set_text(const char* text) {
  return clip::set_text(text);
}
