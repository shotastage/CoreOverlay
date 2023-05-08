#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

int64_t add(int64_t first, int64_t second);

const char *ovr_arch();

const char *ovr_os();

const char *ovr_user_agent();

void ovry_info(const char *msg);

void ovry_log(const char *msg);

void ovry_warning(const char *msg);

void ovry_error(const char *msg);

void overlay_ping(const char *listen_on);

void c_exec_wasm_text_module(const char *wasm_text, const char *main_fn);

void c_exec_wasm_native_module(const unsigned char *data, uintptr_t data_length);

} // extern "C"
