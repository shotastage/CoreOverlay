#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

int64_t add(int64_t first, int64_t second);

const char *arch(void);

void ovry_log(const char *msg);

void ovry_warning(const char *msg);

void ovry_error(const char *msg);

void load_wasm_text_module(const char *wasm_text, const char *main_fn);

void load_wasm_module(void);

int64_t wasm_test(void);
