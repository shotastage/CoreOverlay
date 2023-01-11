#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct CoreOverlayDHTEngine {
  Keypair local_key;
  PeerId local_peer_id;
} CoreOverlayDHTEngine;

int64_t add(int64_t first, int64_t second);

const char *ovr_arch(void);

const char *ovr_os(void);

const char *ovr_user_agent(void);

struct CoreOverlayDHTEngine *new_dht(void);

void ovry_log(const char *msg);

void ovry_warning(const char *msg);

void ovry_error(const char *msg);

void c_exec_wasm_text_module(const char *wasm_text, const char *main_fn);

void c_exec_wasm_native_module(const unsigned char *data, uintptr_t data_length);
