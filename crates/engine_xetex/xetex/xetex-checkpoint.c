#include "xetex-xetexd.h"
#include "xetex-checkpoint.h"
#include "dpx-pdfobj.h"

static const unsigned char *resume_bytes;
static size_t resume_length;
bool tt_checkpoint_enabled;
#define enabled tt_checkpoint_enabled
static bool allowed;
bool tt_checkpoint_resumed;
bool tt_checkpoint_dispatch_fetch;
static unsigned char resume_action;
static uint64_t dispatches, skipped_dispatches;
static uint64_t last_capture;
static unsigned char *capture_bytes;
static size_t capture_capacity;

uint64_t tt_checkpoint_skipped(void) { return skipped_dispatches; }
bool tt_checkpoint_restored(void) { return tt_checkpoint_resumed; }
static bool pending;
static int32_t checkpoint_input_level;
static unsigned char glyph_nodes[MEM_TOP + 1];

void tt_checkpoint_register_glyph(int32_t node) { glyph_nodes[node] = NATIVE_NODE_glyph_info_ptr(node) != NULL; }
void tt_checkpoint_unregister_glyph(int32_t node) { glyph_nodes[node] = 0; }

void tt_checkpoint_release(void)
{
    free(capture_bytes);
    capture_bytes = NULL;
    capture_capacity = 0;
}

void tt_checkpoint_set(const unsigned char *bytes, size_t length, bool record)
{
    tt_checkpoint_release();
    resume_bytes = bytes;
    resume_length = length;
    enabled = record;
    allowed = true;
    checkpoint_input_level = -1;
    pending = false;
    tt_checkpoint_resumed = false;
    tt_checkpoint_dispatch_fetch = false;
    resume_action = 0;
    dispatches = skipped_dispatches = 0;
    last_capture = 0;
    if (record || bytes) {
        memset(glyph_nodes, 0, sizeof(glyph_nodes));
        tt_checkpoint_ext_reset();
    }
}

void tt_checkpoint_disallow(void) { allowed = false; }

void tt_checkpoint_field(tt_checkpoint *cp, const char *name, void *value, size_t length)
{
    uint64_t header[2] = { 14695981039346656037ULL, length };
    for (const unsigned char *p = (const unsigned char *)name; *p; p++)
        header[0] = (header[0] ^ *p) * 1099511628211ULL;
    if (!cp->valid || length > 256 * 1024 * 1024) { cp->valid = false; return; }
    if (cp->restoring) {
        if (cp->position > cp->length || sizeof(header) > cp->length - cp->position ||
            memcmp(cp->bytes + cp->position, header, sizeof(header)) != 0 ||
            length > cp->length - cp->position - sizeof(header)) { cp->valid = false; return; }
        cp->position += sizeof(header);
        memcpy(value, cp->bytes + cp->position, length);
        cp->position += length;
    } else {
        size_t required = cp->length + sizeof(header) + length;
        if (required > 512 * 1024 * 1024) { cp->valid = false; return; }
        if (required > cp->capacity) {
            cp->capacity = required + 1024 * 1024;
            cp->bytes = xrealloc(cp->bytes, cp->capacity);
        }
        memcpy(cp->bytes + cp->length, header, sizeof(header));
        cp->length += sizeof(header);
        memcpy(cp->bytes + cp->length, value, length);
        cp->length += length;
    }
}

void tt_checkpoint_array(tt_checkpoint *cp, const char *name, void **value, size_t count, size_t item_size)
{
    if (!cp->valid || item_size == 0 || count > 256 * 1024 * 1024 / item_size) { cp->valid = false; return; }
    size_t length = count * item_size;
    if (cp->restoring) {
        if (cp->position > cp->length || length > cp->length - cp->position) { cp->valid = false; return; }
        *value = xcalloc(count == 0 ? 1 : count, item_size);
    } else if (*value == NULL && length != 0) { cp->valid = false; return; }
    tt_checkpoint_field(cp, name, *value, length);
}

void tt_checkpoint_used_array(tt_checkpoint *cp, const char *name, void **value, size_t capacity, size_t used, size_t item_size)
{
    if (!cp->valid || used > capacity || item_size == 0 || capacity > 256 * 1024 * 1024 / item_size) { cp->valid = false; return; }
    if (cp->restoring) {
        if (cp->position > cp->length || used * item_size > cp->length - cp->position) { cp->valid = false; return; }
        *value = xcalloc(capacity == 0 ? 1 : capacity, item_size);
    }
    tt_checkpoint_field(cp, name, *value, used * item_size);
}

void tt_checkpoint_rle(tt_checkpoint *cp, const char *name, void **value, size_t count, size_t item_size)
{
    if (!cp->valid || item_size == 0 || count > 256 * 1024 * 1024 / item_size) { cp->valid = false; return; }
    if (cp->restoring) *value = xcalloc(count, item_size);
    unsigned char *items = *value;
    size_t position = 0;
    while (position < count && cp->valid) {
        size_t repeated = 0, literal = 0;
        uint64_t stride = 0;
        if (!cp->restoring) {
            repeated = 1;
            if (item_size == sizeof(uint64_t) && position + 1 < count) {
                uint64_t previous, next;
                memcpy(&previous, items + position * item_size, sizeof(previous));
                memcpy(&next, items + (position + 1) * item_size, sizeof(next));
                stride = next - previous;
                while (position + repeated < count) {
                    memcpy(&next, items + (position + repeated) * item_size, sizeof(next));
                    if (next - previous != stride) break;
                    previous = next;
                    repeated++;
                }
            } else {
                while (position + repeated < count &&
                    memcmp(items + position * item_size, items + (position + repeated) * item_size, item_size) == 0) repeated++;
            }
            if (repeated < 3) {
                repeated = 0;
                literal = 1;
                while (position + literal < count) {
                    size_t next = position + literal;
                    if (next + 2 < count) {
                        if (item_size == sizeof(uint64_t)) {
                            uint64_t a, b, c;
                            memcpy(&a, items + next * item_size, sizeof(a));
                            memcpy(&b, items + (next + 1) * item_size, sizeof(b));
                            memcpy(&c, items + (next + 2) * item_size, sizeof(c));
                            if (b - a == c - b) break;
                        } else if (memcmp(items + next * item_size, items + (next + 1) * item_size, item_size) == 0 &&
                            memcmp(items + next * item_size, items + (next + 2) * item_size, item_size) == 0) break;
                    }
                    literal++;
                }
            }
        }
        CP_VALUE(repeated);
        CP_VALUE(literal);
        CP_VALUE(stride);
        if (!cp->valid || repeated > count - position || literal > count - position ||
            (repeated == 0) == (literal == 0)) { cp->valid = false; return; }
        size_t length = repeated ? 1 : literal;
        tt_checkpoint_field(cp, name, items + position * item_size, length * item_size);
        if (cp->restoring && cp->valid && repeated) {
            if (item_size == sizeof(uint64_t)) {
                uint64_t word;
                memcpy(&word, items + position * item_size, sizeof(word));
                if (word != 0 || stride != 0) {
                    for (size_t i = 1; i < repeated; i++) {
                        word += stride;
                        memcpy(items + (position + i) * item_size, &word, sizeof(word));
                    }
                }
            } else {
                if (stride != 0) { cp->valid = false; return; }
                for (size_t i = 1; i < repeated; i++) memcpy(items + (position + i) * item_size, items + position * item_size, item_size);
            }
        }
        position += repeated + literal;
    }
}

void tt_checkpoint_memory(tt_checkpoint *cp)
{
    if (!cp->valid || lo_mem_max < 0 || hi_mem_min <= lo_mem_max || hi_mem_min > MEM_TOP) { cp->valid = false; return; }
    memory_word *low = NULL, *high = NULL;
    if (cp->restoring) {
        CP_RLE(low, lo_mem_max + 1);
        CP_RLE(high, MEM_TOP + 1 - hi_mem_min);
        if (cp->valid) {
            mem = xmalloc_array(memory_word, MEM_TOP + 1);
            memcpy(mem, low, (lo_mem_max + 1) * sizeof(memory_word));
            memcpy(mem + hi_mem_min, high, (MEM_TOP + 1 - hi_mem_min) * sizeof(memory_word));
        }
        free(low);
        free(high);
        return;
    }
    memory_word *original = mem;
    mem = xcalloc(MEM_TOP + 1, sizeof(memory_word));
    memcpy(mem, original, (lo_mem_max + 1) * sizeof(memory_word));
    memcpy(mem + hi_mem_min, original + hi_mem_min, (MEM_TOP + 1 - hi_mem_min) * sizeof(memory_word));
    int32_t free_block = rover;
    size_t visited = 0;
    do {
        if (free_block < 0 || free_block >= lo_mem_max || ++visited > MEM_TOP) { cp->valid = false; break; }
        int32_t size = mem[free_block].b32.s0;
        if (size < 2 || size > lo_mem_max + 1 - free_block) { cp->valid = false; break; }
        memset(mem + free_block + 2, 0, (size - 2) * sizeof(memory_word));
        free_block = mem[free_block + 1].b32.s1;
    } while (free_block != rover);
    for (int32_t node = 0; node <= MEM_TOP; node++) {
        if (glyph_nodes[node]) NATIVE_NODE_glyph_info_ptr(node) = NULL;
    }
    low = mem;
    high = mem + hi_mem_min;
    CP_RLE(low, lo_mem_max + 1);
    CP_RLE(high, MEM_TOP + 1 - hi_mem_min);
    free(mem);
    mem = original;
}

void tt_checkpoint_string(tt_checkpoint *cp, const char *name, char **value)
{
    if (!cp->valid) return;
    size_t length = cp->restoring ? 0 : (*value == NULL ? 0 : strlen(*value) + 1);
    CP_VALUE(length);
    if (!cp->valid) return;
    if (length > 1024 * 1024) { cp->valid = false; return; }
    tt_checkpoint_array(cp, name, (void **)value, length, 1);
    if (cp->restoring && length == 0) { free(*value); *value = NULL; }
    if (cp->restoring && cp->valid && length && (*value)[length - 1] != 0) cp->valid = false;
}

static void checkpoint_glyphs(tt_checkpoint *cp)
{
    CP_VALUE(glyph_nodes);
    for (int32_t node = 0; node <= MEM_TOP; node++) {
        if (!glyph_nodes[node]) continue;
        if (node > MEM_TOP - 6) { cp->valid = false; return; }
        size_t length = (size_t)NATIVE_NODE_glyph_count(node) * NATIVE_GLYPH_INFO_SIZE;
        if (cp->restoring) NATIVE_NODE_glyph_info_ptr(node) = NULL;
        tt_checkpoint_array(cp, "glyph", &NATIVE_NODE_glyph_info_ptr(node), length, 1);
    }
}

static void transfer(tt_checkpoint *cp)
{
    CP_VALUE(resume_action);
    CP_VALUE(dispatches);
    tt_checkpoint_engine(cp);
    if (!cp->valid) return;
    checkpoint_glyphs(cp);
    tt_checkpoint_shipout(cp);
    tt_checkpoint_pagebuilder(cp);
    tt_checkpoint_synctex(cp);
    tt_checkpoint_clock(cp);
    if (cp->valid && !tt_xetex_checkpoint_layout(cp, cp->restoring)) cp->valid = false;
}

bool tt_checkpoint_resume(void)
{
    if (!resume_bytes || !resume_length) return false;
    pdf_files_init();
    tt_checkpoint cp = { (unsigned char *)resume_bytes, resume_length, 0, 0, true, true };
    transfer(&cp);
    if (!cp.valid || cp.position != cp.length) _tt_abort("invalid engine checkpoint");
    tt_checkpoint_ext_reset();
    tt_checkpoint_resumed = true;
    skipped_dispatches = dispatches;
    last_capture = dispatches;
    return true;
}

static void capture(unsigned char action)
{
    if (!enabled || !allowed || in_initex_mode || shell_escape_enabled || semantic_pagination_enabled) return;
    if (in_open < 1 || expand_depth_count != 0) return;
    bool document_group = cur_level == LEVEL_ONE + 1 && cur_group == SEMI_SIMPLE_GROUP;
    if (cur_list.mode != VMODE || nest_ptr != 0 || (cur_level != LEVEL_ONE && !document_group) || cond_ptr != TEX_NULL ||
        output_active || scanner_status != NORMAL || cur_input.state == TOKEN_LIST || param_ptr != 0 ||
        history > HISTORY_WARNING_ISSUED || help_ptr != 0 || pseudo_files != TEX_NULL || trie_not_ready) return;
    for (int i = 0; i < 16; i++) if (read_open[i] != CLOSED) return;
    for (int i = 1; i <= in_open; i++) if (!input_file[i] || input_file[i]->conversionData) return;
    if (!tt_xetex_checkpoint_ready()) { enabled = false; return; }
    resume_action = action;
    tt_checkpoint cp = { capture_bytes, 0, capture_capacity, 0, false, true };
    transfer(&cp);
    if (cp.valid) {
        tt_xetex_checkpoint_emit(cp.bytes, cp.length);
        last_capture = dispatches;
    }
    capture_bytes = cp.bytes;
    capture_capacity = cp.capacity;
    resume_action = 0;
    pending = false;
}

void tt_checkpoint_try(void)
{
    if (!enabled) return;
    dispatches++;
    if (checkpoint_input_level < 0) checkpoint_input_level = in_open;
    if (in_open < checkpoint_input_level) pending = true;
    checkpoint_input_level = in_open;
    if (pending || (in_open == 1 && dispatches - last_capture >= 262144)) capture(0);
}

void tt_checkpoint_before_input(void)
{
    if (!enabled || !tt_checkpoint_dispatch_fetch || expand_depth_count != 0 ||
        cur_input.state == TOKEN_LIST || scanner_status != NORMAL || cur_list.mode != VMODE) return;
    if (cur_cmd == INPUT && cur_chr == 0) { capture(1); return; }
    if (cur_cmd < CALL || cur_cmd >= END_TEMPLATE || cur_cs < HASH_BASE || cur_cs > hash_top) return;
    char *name = gettexstring(hash[cur_cs].s1);
    bool input = strcmp(name, "input") == 0 || strcmp(name, "include") == 0;
    free(name);
    if (input) capture(2);
}

void tt_checkpoint_continue(void)
{
    unsigned char action = resume_action;
    resume_action = 0;
    if (action == 1) expand();
    else if (action == 2) macro_call();
    else if (action != 0) _tt_abort("invalid checkpoint continuation");
}
